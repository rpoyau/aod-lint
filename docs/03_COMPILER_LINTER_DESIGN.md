# 03. `aod-lint` Compiler & Linter Specification

`aod-lint` acts as a "Rust borrow-checker and `cargo audit` for scientific thought."

## 1. Two Operating Modes
1. **Strict Internal Mode (`aod`, `afc`, `general-mechanics`):**
   - Zero ungrounded imports tolerated.
   - `main` checksum locked.
   - All variables must map to declared Machian distinctions and D.E.C. edges.
2. **Universal Diagnostic Decompiler Mode (Any External Scientific Paper):**
   - **Two-Pass Dependency Hoisting:** Even poorly organized papers can be scientifically consistent.
     - *Pass 1 (Hoisting):* Scans the paper, catches inline/midstream assumptions, tacit limits (`N -> inf`), external laws, and unlabeled constants, hoisting them into a unified **Axiomatic Bill of Materials (Manifest)**.
     - *Pass 2 (Relational Closure):* Grants all hoisted premises and checks whether the internal derivation graph closes and Stokes balances without contradiction.

## 2. Diagnostic Spectrum: Errors vs. Warnings
- **`FATAL / ERROR` (Broken Science):**
  - Internal contradiction even after hoisting.
  - AFC Stokes leak (`S_div != S_d` or `S_int != 0`).
  - Channel capacity violation (`R > C(P)`).
  - Mutating frozen `main` in strict mode.
- **`WARNING` (Stylistically Loose AF Factoring):**
  - `[AF4]` Midstream import hoisted from body text instead of Section 2.
  - `[AF1 / §6]` Defensive philosophical essays defending axioms instead of stating them once.
  - `[AF3 / §7.4]` Unlabeled constant/parameter not wrapped in explicit `Choice Dk` block.
  - `[AF7]` Missing concrete falsification hooks tied to derived claims.

## 3. "Check Your Imports" & Transitive Citation Supply-Chain Audit
- When an external paper imports heavy ontology (e.g., Smooth Manifold `M`), the linter reports:
  `Verdict: CONDITIONALLY CONSISTENT — CHECK YOUR IMPORTS` alongside an **Ontological Liability Receipt** (cardinality inflation, singularity risk, hidden absolute space).
- **Transitive Reference Audit (Expanded Roadmap):**
  - When Paper A cites `@paperB[Theorem 2]`, an upstream inconsistent reference (`⊥ => anything`, *ex falso quodlibet*) poisons Paper A's foundation. Expanded tooling crawls cited DOIs/arXiv preprints and grades upstream AF consistency.

## 4. The "Be Nice" Academic Social Contract (§7 Mechanics)
- **Strict Anchor Mandate (AF §7, Bullet 2 & §8 Check 7):**
  - Every linter rule itself MUST cite an exact AF clause (`AF1`–`AF7`, `RF1`–`RF5`, `§7.1`–`§7.4`, `§8.1`–`§8.8`).
  - Forbids vague ranges (`see §§2–5`, `see the literature`).
  - Requires exact anchors on citations (`@munkres1984[Theorem 14.2, p. 88]`, not bare `@munkres1984`).
- **Constructive Tone ("Be Nice"):**
  - Citations honor academic provenance and the reader's time. Diagnostics must read like a helpful, respectful senior collaborator providing exact Typst auto-fixes—never snarky referee prose.

## 5. Section 8 Binary Rubric (`u8` Bitmask)
All 8 checks (`c1`..`c8`) map to bits `0..7` of a `u8`. Convergence (`Omega`) requires `0b1111_1111` (`0xFF`):
- Bit 0 (`c1`): Axioms stated once, symbols defined, scope fixed.
- Bit 1 (`c2`): Immediate consequences via pure math only.
- Bit 2 (`c3`): Modelling choices declared explicitly.
- Bit 3 (`c4`): Corollaries proved without external laws.
- Bit 4 (`c5`): Demonstrations instantiate earlier steps only (`manual` isolation).
- Bit 5 (`c6`): Falsification hooks tied to derived statements.
- Bit 6 (`c7`): Unique labels, precise single anchors, consistent units/notation.
- Bit 7 (`c8`): Relational foundations RF1–RF5 respected.
