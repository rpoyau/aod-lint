#!/usr/bin/env python3
"""Deterministic publication and release packager for AOD-Lint."""

from __future__ import annotations

import argparse
import hashlib
import os
import shutil
import subprocess
import sys
import zipfile
from pathlib import Path


EXCLUDED_PREFIXES = (".git/", "target/", "dist/", "__pycache__/", ".pytest_cache/")
EXCLUDED_NAMES = {"main.pdf", "manual.pdf", "tests.txt", "Cargo.lock"}


def sha256_file(path: Path) -> str:
    h = hashlib.sha256()
    with path.open("rb") as f:
        for chunk in iter(lambda: f.read(65536), b""):
            h.update(chunk)
    return h.hexdigest()


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
    args = parser.parse_args()

    repo_root = Path(__file__).resolve().parent.parent
    outdir = (repo_root / args.outdir).resolve()
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

    # 3. Package flat source.zip
    source_rel_paths = collect_source_paths(repo_root)
    source_zip = outdir / "source.zip"
    write_deterministic_zip(
        source_zip, [(p.as_posix(), repo_root / p) for p in source_rel_paths]
    )

    # 4. Package bundle.zip with main.pdf first
    bundle_members = [
        ("main.pdf", main_pdf),
        ("manual.pdf", manual_pdf),
        ("tests.txt", tests_txt),
        ("source.zip", source_zip),
    ]
    bundle_contents_sha = outdir / "BUNDLE_CONTENTS_SHA256.txt"
    bundle_contents_sha.write_text(
        "".join(f"{sha256_file(path)}  {name}\n" for name, path in bundle_members),
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
        bundle_zip,
        tests_txt,
        bundle_contents_sha,
    ]
    sha256_txt = outdir / "SHA256.txt"
    sha256_txt.write_text(
        "".join(f"{sha256_file(p)}  {p.name}\n" for p in deliverables),
        encoding="utf-8",
    )

    print(f"Built release bundle in {outdir}:")
    for p in [*deliverables, sha256_txt]:
        print(f"  {p.name} ({p.stat().st_size} bytes)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
