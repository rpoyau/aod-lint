#!/usr/bin/env python3
"""Deterministic publication and Zenodo release packager for AOD-Lint."""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import re
import shutil
import subprocess
import sys
import zipfile
from pathlib import Path


EXCLUDED_PREFIXES = (".git/", "target/", "dist/", "__pycache__/", ".pytest_cache/")
EXCLUDED_NAMES = {"main.pdf", "manual.pdf", "tests.txt", "patch_summary.txt", "Cargo.lock"}

VALID_DATACITE_RELATIONS = {
    "isCitedBy",
    "cites",
    "isSupplementTo",
    "isSupplementedBy",
    "isContinuedBy",
    "continues",
    "isNewVersionOf",
    "isPreviousVersionOf",
    "isPartOf",
    "hasPart",
    "isReferencedBy",
    "references",
    "isDocumentedBy",
    "documents",
    "isCompiledBy",
    "compiles",
    "isVariantFormOf",
    "isOriginalFormOf",
    "isIdenticalTo",
    "isAlternateIdentifier",
    "isReviewedBy",
    "reviews",
    "isDerivedFrom",
    "isSourceOf",
}


def sha256_file(path: Path) -> str:
    h = hashlib.sha256()
    with path.open("rb") as f:
        for chunk in iter(lambda: f.read(65536), b""):
            h.update(chunk)
    return h.hexdigest()


def read_canonical_version(repo_root: Path) -> str:
    p = repo_root / "CANONICAL_VERSION.txt"
    if not p.is_file():
        raise FileNotFoundError("CANONICAL_VERSION.txt is required at repository root")
    m = re.search(r"Canonical version:\s*(\S+)", p.read_text(encoding="utf-8"))
    if not m:
        raise ValueError("CANONICAL_VERSION.txt missing 'Canonical version: <vX.Y.Z>'")
    return m.group(1)


def validate_zenodo_metadata(repo_root: Path, version: str) -> None:
    sync_script = repo_root / "scripts" / "sync_zenodo_references.py"
    if sync_script.is_file():
        subprocess.run([sys.executable, str(sync_script), "--check"], check=True)

    meta_path = repo_root / ".zenodo.json"
    if not meta_path.is_file():
        raise FileNotFoundError("repository-root .zenodo.json is required for GitHub-Zenodo sync")

    data = json.loads(meta_path.read_text(encoding="utf-8"))
    if data.get("version") != version:
        raise ValueError(
            f".zenodo.json version {data.get('version')!r} does not match CANONICAL_VERSION.txt {version!r}"
        )
    if not data.get("title"):
        raise ValueError(".zenodo.json title is required")
    creators = data.get("creators") or []
    if not creators or "," not in creators[0].get("name", ""):
        raise ValueError(".zenodo.json creators[0].name must use 'Family, Given' format")
    if not data.get("references"):
        raise ValueError(".zenodo.json references must be synchronized from refs.bib and manual/refs.bib")
    for rel in data.get("related_identifiers", []):
        relation = rel.get("relation")
        if relation not in VALID_DATACITE_RELATIONS:
            raise ValueError(
                f"Invalid Zenodo/DataCite related_identifiers relation {relation!r}"
            )
        if not rel.get("scheme"):
            raise ValueError(f"Missing scheme on related_identifier {rel.get('identifier')!r}")


def find_typst() -> str:
    found = shutil.which("typst")
    if found:
        return found
    local_candidate = Path.home() / ".local" / "bin" / "typst"
    if local_candidate.is_file() and os.access(local_candidate, os.X_OK):
        return str(local_candidate)
    raise RuntimeError("typst CLI not found on PATH or ~/.local/bin/typst")


def run_checked(cmd: list[str], cwd: Path) -> str:
    proc = subprocess.run(
        cmd,
        cwd=str(cwd),
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
        text=True,
        check=False,
    )
    if proc.returncode != 0:
        sys.stderr.write(proc.stdout)
        raise RuntimeError(f"Command failed ({proc.returncode}): {' '.join(cmd)}")
    return proc.stdout


def collect_source_paths(repo_root: Path) -> list[Path]:
    git_dir = repo_root / ".git"
    if git_dir.exists() and shutil.which("git"):
        out = run_checked(["git", "ls-files"], repo_root)
        paths = [Path(line.strip()) for line in out.splitlines() if line.strip()]
        return sorted(p for p in paths if (repo_root / p).is_file())

    collected: list[Path] = []
    for root, dirs, files in os.walk(repo_root):
        rel_root = Path(root).relative_to(repo_root)
        dirs[:] = [
            d
            for d in sorted(dirs)
            if d not in {".git", "target", "dist", "__pycache__", ".pytest_cache"}
        ]
        for fname in sorted(files):
            if fname in EXCLUDED_NAMES:
                continue
            rel = rel_root / fname if str(rel_root) != "." else Path(fname)
            rel_posix = rel.as_posix()
            if any(rel_posix.startswith(pfx) for pfx in EXCLUDED_PREFIXES):
                continue
            collected.append(rel)
    return sorted(collected)


def write_deterministic_zip(zip_path: Path, members: list[tuple[str, Path]]) -> None:
    with zipfile.ZipFile(zip_path, "w", compression=zipfile.ZIP_DEFLATED) as zf:
        for arcname, src_path in members:
            info = zipfile.ZipInfo(arcname, date_time=(2026, 1, 1, 0, 0, 0))
            mode = 0o755 if os.access(src_path, os.X_OK) else 0o644
            info.external_attr = (mode & 0xFFFF) << 16
            info.compress_type = zipfile.ZIP_DEFLATED
            zf.writestr(info, src_path.read_bytes())


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--outdir",
        type=Path,
        default=Path("dist"),
        help="Output directory for release artifacts (default: dist)",
    )
    parser.add_argument(
        "--patch-summary",
        type=Path,
        default=Path("PATCH_SUMMARY.txt"),
        help="Optional patch summary input file",
    )
    args = parser.parse_args()

    repo_root = Path(__file__).resolve().parent.parent
    version = read_canonical_version(repo_root)
    validate_zenodo_metadata(repo_root, version)

    outdir = (repo_root / args.outdir).resolve()
    if outdir.exists():
        shutil.rmtree(outdir)
    outdir.mkdir(parents=True, exist_ok=True)

    typst_bin = find_typst()
    main_pdf = outdir / "main.pdf"
    manual_pdf = outdir / "manual.pdf"

    # 1. Compile Main then Manual
    run_checked([typst_bin, "compile", "main.typ", str(main_pdf)], repo_root)
    run_checked(
        [typst_bin, "compile", "--root", ".", "manual/main.typ", str(manual_pdf)],
        repo_root,
    )

    # 2. Execute cargo test & self-audits into dist/tests.txt
    logs: list[str] = []
    for cmd in [
        ["cargo", "test", "--quiet"],
        ["cargo", "run", "--quiet", "--", "check", ".", "--strict"],
        [
            "cargo",
            "run",
            "--quiet",
            "--",
            "digest",
            ".",
            "--baseline",
            "contracts/main_baseline_manifest.json",
        ],
        ["cargo", "run", "--quiet", "--", "hygiene", "."],
    ]:
        logs.append(f"$ {' '.join(cmd)}\n" + run_checked(cmd, repo_root))

    tests_txt = outdir / "tests.txt"
    tests_txt.write_text("\n".join(logs), encoding="utf-8")

    patch_summary_src = (repo_root / args.patch_summary).resolve()
    patch_summary = outdir / "patch_summary.txt"
    if patch_summary_src.is_file():
        shutil.copy2(patch_summary_src, patch_summary)
    else:
        patch_summary.write_text(
            f"{version} release bundle generated by scripts/build_release_bundle.py.\n",
            encoding="utf-8",
        )

    # 3. Package flat source.zip
    source_rel_paths = collect_source_paths(repo_root)
    source_zip = outdir / "source.zip"
    write_deterministic_zip(
        source_zip, [(p.as_posix(), repo_root / p) for p in source_rel_paths]
    )

    # 4. Package bundle.zip with main.pdf first (matching rpoyau/aod order)
    bundle_members = [
        ("main.pdf", main_pdf),
        ("manual.pdf", manual_pdf),
        ("source.zip", source_zip),
        ("tests.txt", tests_txt),
        ("patch_summary.txt", patch_summary),
    ]
    bundle_contents_sha = outdir / "BUNDLE_CONTENTS_SHA256.txt"
    bundle_contents_sha.write_text(
        "".join(f"{sha256_file(path)}  {name}\n" for name, path in sorted(bundle_members)),
        encoding="utf-8",
    )
    bundle_members.append(("BUNDLE_CONTENTS_SHA256.txt", bundle_contents_sha))

    bundle_zip = outdir / "bundle.zip"
    write_deterministic_zip(bundle_zip, bundle_members)

    # 5. Top-level SHA256.txt
    deliverables = [
        main_pdf,
        manual_pdf,
        source_zip,
        tests_txt,
        patch_summary,
        bundle_contents_sha,
        bundle_zip,
    ]
    sha256_txt = outdir / "SHA256.txt"
    sha256_txt.write_text(
        "".join(f"{sha256_file(p)}  {p.name}\n" for p in deliverables),
        encoding="utf-8",
    )

    print(f"Built release bundle for {version} in {outdir}:")
    for p in [*deliverables, sha256_txt]:
        print(f"  {p.name} ({p.stat().st_size} bytes)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
