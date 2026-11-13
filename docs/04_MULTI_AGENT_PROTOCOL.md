# 04. Multi-Agent Change Protocol & 130+ Page Slicing (`aod-skills`)

Reviewing PRs like `rpoyau/aod#43` cannot dump 130+ pages of dense Typst into an LLM window, nor can it inspect only the isolated diff blind to global invariants.

## 1. Digest & Blast-Radius Slicing Pipeline
1. **`aod-digest`:** Parses the 130+ page Typst treatise via `typst-syntax` into a compact `aod-manifest.json` (~20 KB) indexing:
   - Declared Distinctions (`V`)
   - Declared Edges (`E_origin -> E_target`)
   - Named Motifs & Section `<label>` anchors
   - Cryptographic SHA-256 hashes of frozen `main` sections.
2. **`aod-slice` (Reachability / Blast Radius):**
   - Takes `gh pr diff <PR>` + `aod-manifest.json`.
   - Computes graph reachability on touched edges/motifs.
   - Extracts ONLY the diff + impacted 3–5 page subgraph slice into the `Context Bundle`.

## 2. Deterministic State Machine & Role Bundles (`alpha -> omega`)
- **Lead / Controller:** Manages state transitions, enforces convergence (`Section8Rubric == 0xFF` + green CI), halts or merges.
- **Reviewer:** Audits `Context Bundle` via `aod-lint --json`. Emits immutable `Feedback Bundle` or `Axiomatic Inquiry Bundle`.
- **Planner:** Converts `Feedback Bundle` into ordered, non-conflicting steps (`Request Bundle`).
- **Writer:** Applies code/Typst patches (converting residual LaTeX to Typst, hoisting `Choice` blocks, fixing anchors).
- **Tester:** Executes `cargo test`, Python/Rust D.E.C. runners, and `typst compile` -> emits `Verification Bundle`. Loops back to Writer automatically on compiler errors.

## 3. Stratified Axiomatic Inquiry Gate (Restricting Agent Questions)
Agents are FORBIDDEN from asking open-ended conversational questions ("What do you prefer?"). Every human interrupt MUST be an `Axiomatic Inquiry Bundle` typed strictly to the broken rung of the ladder:
1. `DISTINCTION` Gate: Are `A` and `B` distinct cuts against `N`, or equivalent?
2. `RELATION` Gate: Is coupling directed (`A -> B`), antisymmetric flux (`A <-> B`), or uncoupled?
3. `MOTIF` Gate: Does this cycle form a Ring Gate, SADAR scan, conserved oscillator, or dissipation?
4. `MODELING` Gate: Which explicit `Choice D` representation encodes this motif?
5. `SCOPE` Gate: Is this a registered Motif skill, or a demonstration strictly for `manual.typ`?
