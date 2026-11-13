# Antigravity Session Boot Instructions

Before taking any action in `rpoyau/aod-lint`, `rpoyau/aod`, or `rpoyau/aod-skills`, read all four specifications in `./docs/` in strict order:
1. `docs/01_ONTOLOGY_AND_AF_PROTOCOL.md`
2. `docs/02_AFC_AND_DEC_ENGINE.md`
3. `docs/03_COMPILER_LINTER_DESIGN.md`
4. `docs/04_MULTI_AGENT_PROTOCOL.md`

Then:
1. Verify `cargo test` passes cleanly.
2. Create branch `feat/rust-core`, commit all files, push, and open the initial PR via `gh pr create`.
3. Proceed to build `aod-digest` (130p Typst manifest + frozen `main` SHA-256 guard) and `aod-slice` (D.E.C. blast-radius slicer for `rpoyau/aod#43`).
