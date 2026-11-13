# Build and release

This source tree is version-neutral. Release artifact filenames are stable (`main.pdf`, `manual.pdf`, `source.zip`, `bundle.zip`, `tests.txt`, `BUNDLE_CONTENTS_SHA256.txt`, `SHA256.txt`). The release version is read from `CANONICAL_VERSION.txt` and recorded in metadata and manifests. Compiled PDFs and archives are written exclusively to the ignored `dist/` directory so the tracked source tree remains 100% compliant with `aod-lint hygiene`.

## Dependencies

Install a stable Rust toolchain (`rustc`, `cargo`), Python 3 (for release packaging), and the [Typst CLI](https://github.com/typst/typst/releases) (`typst >= 0.13`):

```bash
# Verify toolchains
rustc --version
cargo --version
typst --version
python3 --version
```

## Paired Manuscript Build (`Main` then `Manual`)

Compile `main.typ` first and `manual/main.typ` second with `--root .` so `manual/preamble.typ` resolves shared layout primitives from the root `preamble.typ` while keeping `Main` strictly independent of `Manual`:

```bash
mkdir -p dist
typst compile main.typ dist/main.pdf
typst compile --root . manual/main.typ dist/manual.pdf
```

## Self-Hosting Verification Gates

Before packaging publication artifacts, execute the Rust test suite and all self-hosting `aod-lint` passes against the repository itself:

```bash
# 1. Unit, Stokes, LaTeX, slicing, hygiene, and self-hosting manuscript tests
cargo test

# 2. Strict internal AF & General Mechanics style audit (must report Rubric 11111111)
cargo run --release -- check . --strict

# 3. Frozen Main/Appendix SHA-256 guard, firewall check, and include-cycle check
cargo run --release -- digest . --baseline contracts/main_baseline_manifest.json

# 4. Source-tree provenance & agent state-leakage quarantine check
cargo run --release -- hygiene .
```

If an intentional major-version amendment modifies `main.typ`, `preamble.typ`, `sections/`, or `appendices/` (`AF §10`), refresh the frozen baseline manifest explicitly in the same pull request:

```bash
cargo run --release -- digest . -o contracts/main_baseline_manifest.json
```

## Automated Release Bundle Packaging

Run `scripts/build_release_bundle.py` from the repository root to execute the paired Typst builds, run the full `cargo test` and self-verification suite, and emit deterministic release archives and SHA-256 receipts under `dist/`:

```bash
python3 scripts/build_release_bundle.py --outdir dist
```

## Artifact order

For Zenodo or archival repositories with a default preview document, place `main.pdf` first:

```text
main.pdf
manual.pdf
source.zip
bundle.zip
tests.txt
BUNDLE_CONTENTS_SHA256.txt
SHA256.txt
```

`bundle.zip` preserves `main.pdf` as its first member. `.zenodo.json` is maintained at the repository root for GitHub-Zenodo release synchronization.
