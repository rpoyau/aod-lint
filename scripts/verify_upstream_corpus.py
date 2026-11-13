#!/usr/bin/env python3
"""Verify aod-lint against canonical upstream LaTeX repositories (aod, afc, general-mechanics).

Supports both offline local checkouts (--corpus-dir) and shallow CI clones (--clone-dir).
"""

from __future__ import annotations

import argparse
import collections
import json
import os
import pathlib
import subprocess
import sys
import tempfile
from typing import Any


UPSTREAM_SPECS: list[dict[str, Any]] = [
    {
        "name": "aod",
        "repo": "rpoyau/aod",
        "url": "https://github.com/rpoyau/aod.git",
        "min_files": 40,
        "min_edges": 500,
        "required_tiers": {"Main", "Appendix", "Manual"},
        "max_check_warnings": 0,
        "test_manual_slice": True,
        "allowed_hygiene_error_files": set(),
    },
    {
        "name": "afc",
        "repo": "rpoyau/afc",
        "url": "https://github.com/rpoyau/afc.git",
        "min_files": 8,
        "min_edges": 40,
        "required_tiers": {"Main", "Appendix"},
        "max_check_warnings": 10,  # pre-GM inline \cite remarks in AFC v1
        "test_manual_slice": False,
        "allowed_hygiene_error_files": set(),
    },
    {
        "name": "general-mechanics",
        "repo": "rpoyau/general-mechanics",
        "url": "https://github.com/rpoyau/general-mechanics.git",
        "min_files": 20,
        "min_edges": 180,
        "required_tiers": {"Main", "Appendix"},
        "max_check_warnings": 0,
        "test_manual_slice": False,
        "allowed_hygiene_error_files": {"docs/axiomatic-fundamentalism.pdf"},
    },
]


def resolve_binary(repo_root: pathlib.Path, explicit_bin: str | None) -> pathlib.Path:
    if explicit_bin:
        p = pathlib.Path(explicit_bin).resolve()
        if not p.is_file():
            raise RuntimeError(f"Specified --bin binary does not exist: {p}")
        return p

    for candidate in (
        repo_root / "target" / "release" / "aod-lint",
        repo_root / "target" / "debug" / "aod-lint",
    ):
        if candidate.is_file() and os.access(candidate, os.X_OK):
            return candidate

    subprocess.run(
        ["cargo", "build", "--release"],
        cwd=repo_root,
        check=True,
    )
    built = repo_root / "target" / "release" / "aod-lint"
    if not built.is_file():
        raise RuntimeError(f"Failed to build aod-lint binary at {built}")
    return built


def prepare_checkout(
    spec: dict[str, Any],
    corpus_dir: pathlib.Path | None,
    clone_dir: pathlib.Path,
) -> pathlib.Path:
    if corpus_dir is not None:
        local_path = (corpus_dir / spec["name"]).resolve()
        if not local_path.is_dir():
            raise RuntimeError(
                f"Expected upstream checkout for {spec['repo']} at {local_path}"
            )
        return local_path

    clone_dir.mkdir(parents=True, exist_ok=True)
    dest = (clone_dir / spec["name"]).resolve()
    if not (dest / ".git").exists():
        subprocess.run(
            ["git", "clone", "--depth", "1", spec["url"], str(dest)],
            check=True,
        )
    return dest


def run_json_cmd(cmd: list[str]) -> tuple[int, dict[str, Any], str]:
    proc = subprocess.run(cmd, capture_output=True, text=True, check=False)
    try:
        payload = json.loads(proc.stdout)
    except json.JSONDecodeError as exc:
        raise RuntimeError(
            f"Command {' '.join(cmd)} did not emit valid JSON (exit {proc.returncode}):\n"
            f"STDOUT:\n{proc.stdout}\nSTDERR:\n{proc.stderr}"
        ) from exc
    return proc.returncode, payload, proc.stderr


def audit_repository(
    binary: pathlib.Path,
    spec: dict[str, Any],
    target_dir: pathlib.Path,
    scratch: pathlib.Path,
) -> dict[str, Any]:
    failures: list[str] = []

    # 1. Strict premise-boundary & General Mechanics check
    check_rc, check_json, _ = run_json_cmd(
        [str(binary), "check", str(target_dir), "--strict", "--json"]
    )
    diags = check_json.get("diagnostics", [])
    sev_counts = collections.Counter(d.get("severity", "Unknown") for d in diags)

    if check_rc != 0:
        failures.append(f"check --strict exited with {check_rc} (expected 0)")
    if not check_json.get("converged", False) or check_json.get("rubric_bits") != "11111111":
        failures.append(
            f"check --strict failed Section8Rubric convergence: {check_json.get('rubric_bits')}"
        )
    if check_json.get("hoisted_dependencies", 0) != 0:
        failures.append(
            f"unexpected hoisted_dependencies={check_json.get('hoisted_dependencies')}"
        )
    if sev_counts["Fatal"] > 0 or sev_counts["Error"] > 0:
        failures.append(
            f"unexpected Fatal/Error diagnostics in check --strict: {dict(sev_counts)}"
        )
    if sev_counts["Warning"] > spec["max_check_warnings"]:
        failures.append(
            f"check warnings {sev_counts['Warning']} exceeded ceiling {spec['max_check_warnings']}"
        )

    # 2. Treatise digest, tier classification, firewall, and acyclicity
    manifest_path = scratch / f"{spec['name']}_manifest.json"
    digest_proc = subprocess.run(
        [str(binary), "digest", str(target_dir), "-o", str(manifest_path)],
        capture_output=True,
        text=True,
        check=False,
    )
    if digest_proc.returncode != 0:
        failures.append(
            f"digest exited with {digest_proc.returncode}: {digest_proc.stderr.strip()}"
        )
    manifest = json.loads(manifest_path.read_text(encoding="utf-8"))
    files = manifest.get("files", [])
    edges = manifest.get("edges", [])
    tier_counts = collections.Counter(f.get("tier", "Other") for f in files)
    firewall_violations = manifest.get("firewall_violations", [])
    include_cycles = manifest.get("include_cycles", [])

    if len(files) < spec["min_files"]:
        failures.append(f"indexed {len(files)} files (expected >= {spec['min_files']})")
    if len(edges) < spec["min_edges"]:
        failures.append(f"extracted {len(edges)} edges (expected >= {spec['min_edges']})")
    missing_tiers = spec["required_tiers"] - set(tier_counts.keys())
    if missing_tiers:
        failures.append(f"missing expected document tiers {sorted(missing_tiers)}")
    if firewall_violations:
        failures.append(f"detected {len(firewall_violations)} Main->Manual firewall violation(s)")
    if include_cycles:
        failures.append(f"detected {len(include_cycles)} include cycle(s)")

    # 3. 1-hop blast-radius slice check on paired Main + Manual treatises
    slice_impacted = 0
    if spec["test_manual_slice"]:
        manual_candidates = [
            f["path"]
            for f in files
            if f.get("tier") == "Manual" and len(f.get("references", [])) > 0
        ]
        if not manual_candidates:
            failures.append("no Manual file with outgoing references found for slice test")
        else:
            sample_manual = manual_candidates[0]
            diff_path = scratch / f"{spec['name']}_sample.diff"
            diff_path.write_text(
                f"--- a/{sample_manual}\n+++ b/{sample_manual}\n@@ -1 +1 @@\n",
                encoding="utf-8",
            )
            slice_rc, slice_json, _ = run_json_cmd(
                [
                    str(binary),
                    "slice",
                    "--manifest",
                    str(manifest_path),
                    "--diff",
                    str(diff_path),
                    "--hops",
                    "1",
                    "--json",
                ]
            )
            slice_impacted = len(slice_json.get("impacted_files", []))
            if slice_rc != 0:
                failures.append(f"slice exited with {slice_rc}")
            if slice_json.get("frozen_main_violations"):
                failures.append(
                    f"unexpected frozen_main_violations on Manual patch: {slice_json.get('frozen_main_violations')}"
                )
            if slice_impacted < 2:
                failures.append(
                    f"1-hop slice from {sample_manual} failed to expand upstream neighborhood (impacted={slice_impacted})"
                )

    # 4. Source-tree repository hygiene audit
    hygiene_rc, hygiene_json, _ = run_json_cmd(
        [str(binary), "hygiene", str(target_dir), "--json"]
    )
    h_diags = hygiene_json.get("diagnostics", [])
    h_sev = collections.Counter(d.get("severity", "Unknown") for d in h_diags)
    error_files = {
        d.get("file", "") for d in h_diags if d.get("severity") in ("Error", "Fatal")
    }

    if h_sev["Fatal"] > 0:
        failures.append(f"fatal agent/session leak detected in hygiene: {error_files}")
    unexpected_hygiene_errors = error_files - spec["allowed_hygiene_error_files"]
    if unexpected_hygiene_errors:
        failures.append(
            f"unexpected hygiene error artifact(s): {sorted(unexpected_hygiene_errors)}"
        )
    if not error_files and hygiene_rc != 0:
        failures.append(f"hygiene exited {hygiene_rc} despite zero Fatal/Error diagnostics")

    return {
        "repo": spec["repo"],
        "files": len(files),
        "edges": len(edges),
        "tiers": dict(tier_counts),
        "strict_rubric": check_json.get("rubric_bits"),
        "check_warnings": sev_counts["Warning"],
        "check_infos": sev_counts["Info"],
        "firewall_violations": len(firewall_violations),
        "include_cycles": len(include_cycles),
        "slice_impacted": slice_impacted,
        "hygiene_rubric": hygiene_json.get("rubric_bits"),
        "hygiene_known_exceptions": sorted(error_files & spec["allowed_hygiene_error_files"]),
        "passed": len(failures) == 0,
        "failures": failures,
    }


def format_markdown_summary(results: list[dict[str, Any]]) -> str:
    lines = [
        "### Upstream Canonical Corpus Verification (`aod-lint`)",
        "",
        "| Repository | Source Files | Graph Edges | Strict Rubric | Check Warn / Info | Firewall / Cycles | Hygiene Status | Gate |",
        "| :--- | ---: | ---: | :---: | :---: | :---: | :--- | :---: |",
    ]
    for r in results:
        hygiene_cell = (
            f"`{r['hygiene_rubric']}` (clean)"
            if not r["hygiene_known_exceptions"]
            else f"`{r['hygiene_rubric']}` (known legacy ref: `{', '.join(r['hygiene_known_exceptions'])}`)"
        )
        gate = "PASS" if r["passed"] else "FAIL"
        lines.append(
            f"| `{r['repo']}` | {r['files']} | {r['edges']} | `{r['strict_rubric']}` | "
            f"{r['check_warnings']} / {r['check_infos']} | "
            f"{r['firewall_violations']} / {r['include_cycles']} | "
            f"{hygiene_cell} | **{gate}** |"
        )
    lines.append("")
    return "\n".join(lines)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--bin", default=None, help="Path to compiled aod-lint binary")
    parser.add_argument(
        "--corpus-dir",
        default=None,
        help="Existing parent directory containing aod/, afc/, and general-mechanics/",
    )
    parser.add_argument(
        "--clone-dir",
        default=None,
        help="Temporary directory for shallow git clones when --corpus-dir is omitted",
    )
    parser.add_argument(
        "--summary-md",
        default=None,
        help="Optional path (e.g. $GITHUB_STEP_SUMMARY) to append Markdown summary table",
    )
    parser.add_argument(
        "--json-out",
        default=None,
        help="Optional path to write structured JSON verification report",
    )
    args = parser.parse_args()

    repo_root = pathlib.Path(__file__).resolve().parents[1]
    binary = resolve_binary(repo_root, args.bin)
    corpus_dir = pathlib.Path(args.corpus_dir).resolve() if args.corpus_dir else None

    with tempfile.TemporaryDirectory(prefix="aod_lint_corpus_") as tmp:
        tmp_path = pathlib.Path(tmp)
        clone_dir = (
            pathlib.Path(args.clone_dir).resolve()
            if args.clone_dir
            else (tmp_path / "clones")
        )
        results = []
        for spec in UPSTREAM_SPECS:
            checkout = prepare_checkout(spec, corpus_dir, clone_dir)
            res = audit_repository(binary, spec, checkout, tmp_path)
            results.append(res)

    md = format_markdown_summary(results)
    print(md)

    if args.summary_md:
        summary_path = pathlib.Path(args.summary_md)
        with summary_path.open("a", encoding="utf-8") as handle:
            handle.write(md + "\n")

    if args.json_out:
        out_path = pathlib.Path(args.json_out)
        out_path.parent.mkdir(parents=True, exist_ok=True)
        out_path.write_text(json.dumps(results, indent=2) + "\n", encoding="utf-8")

    failed = [r for r in results if not r["passed"]]
    if failed:
        for r in failed:
            for msg in r["failures"]:
                print(f"FAIL [{r['repo']}]: {msg}", file=sys.stderr)
        return 1

    return 0


if __name__ == "__main__":
    sys.exit(main())
