# AOD-Lint

### Executable Verification for the Axiomatic Fundamentalism Protocol

[![Extends AF](https://img.shields.io/badge/Extends_AF-10.5281%2Fzenodo.17561188-blue)](https://doi.org/10.5281/zenodo.17561188)
[![AFC](https://img.shields.io/badge/AFC-10.5281%2Fzenodo.17795590-4c6ef5)](https://doi.org/10.5281/zenodo.17795590)
[![AOD](https://img.shields.io/badge/A%CE%A9D-10.5281%2Fzenodo.20486270-2b8a3e)](https://doi.org/10.5281/zenodo.20486270)
[![License: MIT](https://img.shields.io/badge/License-MIT-black.svg)](./LICENSE)

**Version:** v0.2.0

## Title and subtitle

**Title:** AOD-Lint: Executable Verification for the Axiomatic Fundamentalism Protocol

**Subtitle:** Premise-Boundary Closure, Two-Pass Hoisting, and Tiered Treatise Hygiene

**Additional subtitle:** A Decidable Extension to Axiomatic Fundamentalism (AF)

## Description

$$\partial_{\mathrm{in}}\mathcal{G} \vdash \mathcal{V}_{\circ}, \qquad \Phi_{\mathrm{ext}}(\mathcal{V}_{\circ}) = 0, \qquad r_{\mathrm{AF}} \in \{0,1\}^8$$

> *Axioms → Distinctions → Relations → Motifs → Calculus.*  
> *Boundary premises determine interior claims; undeclared imports are exterior flux.*

## What `AOD-Lint` is

**AOD-Lint** extends the [Axiomatic Fundamentalism (AF)](https://doi.org/10.5281/zenodo.17561188) protocol from a human authoring and peer-review guideline to a **decidable finite-graph verification calculus**, paired with a native Rust static analyzer and treatise indexer (`src/`) for **Typst** (`.typ`) and **LaTeX** (`.tex`) scientific repositories.

Following the canonical two-volume treatise architecture of [Alpha↔Omega Dynamics (AΩD)](https://doi.org/10.5281/zenodo.20486270), this repository maintains strict separation between foundational protocol proofs (`Main`), operational instantiations (`Manual`), and executable Rust verification passes (`src/`):

- **`main.typ` (`main.pdf`) — The Protocol Extension (`Main`):** Formalizes epistemic boundary closure (`Axioms A1–A2`, `Consequences C1–C2`), typed manuscript derivation DAGs (`Choices D1–D2`, `Corollary K1`), two-pass premise hoisting and the 8-bit Reviewer Rubric Boolean meet-semilattice $\mathbb{B}_8 = (\{0,1\}^8, \wedge, \mathbf{1})$ (`Choices D3–D4`, `Corollary K2`), declared finite AFC Stokes and trit-capacity auditing (`Choice D5`, `Corollary K3`), one-way `Main`/`Manual` quotient-graph firewalls with SHA-256 baseline immutability and $h$-hop blast-radius slicing (`Choice D6`, `Corollary K4`), and source-tree epistemic hygiene with Stratified Axiomatic Inquiry (`Choices D7–D8`, `Corollary K5`, `Appendices A–D`).

- **`manual/main.typ` (`manual.pdf`) — Reference Engine & Worked Ledgers (`Manual`):** Instantiates `Main` without re-axiomatization. Covers the Rust crate architecture (`src/`), CLI execution spine (`check`, `digest`, `slice`, `hygiene`), lossless `typst-syntax` AST and paired `xr-hyper` LaTeX passes, worked nine-column Declared Edge Computation (`D.E.C.`) hinge-slide and discrete Stokes fixtures, repository state-leakage quarantine rules, and multi-agent `AxiomaticInquiryBundle` state machines (`Appendices A–B`).

- **`src/` (`aod-lint`) — Native Rust Compiler & Linter:** Self-hosted zero-GC binary and library crate verifying both this repository's own Typst treatise (`tests/test_self_manuscript.rs`) and upstream/downstream `.typ` and `.tex` manuscripts.

---

## Core Protocol Extensions (`Main §§1–5`)

1. **Epistemic Boundary Cut (`Corollary K1`).** Represents a manuscript as a finite typed graph $\mathcal{G} = (\mathcal{V}, \mathcal{E}, \tau, \alpha)$ partitioned into boundary premises $\mathcal{V}_{\partial} = \mathcal{V}_{\mathrm{Ax}} \sqcup \mathcal{V}_{\mathrm{Ch}}$, interior claims $\mathcal{V}_{\circ} = \mathcal{V}_{\mathrm{Con}} \sqcup \mathcal{V}_{\mathrm{Cor}} \sqcup \mathcal{V}_{\mathrm{Dem}}$, and exterior ungrounded imports $\mathcal{V}_{\mathrm{ext}}$. Soundness holds when exterior premise flux vanishes ($\Phi_{\mathrm{ext}}(\mathcal{V}_{\circ}) = 0$), demonstration back-coupling vanishes ($B_{\mathrm{dem}}(\mathcal{G}) = 0$), and the interior dependency graph is acyclic.

2. **Two-Pass Premise Hoisting & $\mathbb{B}_8$ Meet-Semilattice (`Corollary K2`).** Separates *stylistic premise-factoring defects* from *internal contradictions*. Pass 1 hoists inline assumptions $\mathcal{M}(\mathcal{G})$ into an explicit Dependency Bill of Materials $\mathcal{B}(\mathcal{G})$ and emits an **Ontological Liability Receipt** whenever continuum imports (`RF2`/`RF3`) exceed finite declared distinctions ($|\Omega| > |\mathcal{D}|$). Diagnostic sets project homomorphically onto the 8-bit Reviewer Rubric semilattice $\pi(D_1 \cup D_2) = \pi(D_1) \wedge \pi(D_2)$, guaranteeing order-independent convergence at `0b11111111` (`0xFF`).

3. **Declared Finite Stokes & Trit-Capacity Verification (`Corollary K3`).** Audits finite Markov transition kernels $P$ on declared distinction sets $V$: verifies exact edge antisymmetry $\phi_{ij} = -\phi_{ji}$, interior pair annihilation $S_{\mathrm{int}}(R) = 0$, discrete Stokes balance $\sum_{i \in R}\operatorname{div}(i) = \sum_{(i,j)\in\partial R}\phi_{ij}$, and base-3 mutual-information capacity $R_{\mathrm{trit}} \le C_3(P)$.

4. **One-Way Tier Firewall & Bounded Blast-Radius Slicing (`Corollary K4`).** Locks frozen `Main` and `Appendix` files under collision-resistant SHA-256 commitments ($H(f) = H_0(f)$), enforces $\mathcal{E}_{\mathcal{F}} \cap (\mathcal{F}_{\mathrm{frozen}} \times \mathcal{F}_{\mathrm{Manual}}) = \emptyset$, and extracts the $h$-hop reachability slice $\mathcal{N}_h(\Delta; \mathcal{M})$ around any pull-request diff $\Delta$ so bounded-context reviewers audit 100% of touched premises without full-corpus context overflow.

5. **Source-Tree Epistemic Hygiene & Stratified Inquiry (`Corollary K5`).** Enforces repository provenance purity ($L(\mathcal{R}) = |\mathcal{R} \cap \mathcal{S}_{\mathrm{leak}}| = 0$) by blocking checked-in agent instruction dumps, private cloud session URIs, container scratch paths, unchecked binary handoff archives, and non-figure root PDFs, while binding collaborative ambiguity escalations to the five-rung ladder `DISTINCTION ≺ RELATION ≺ MOTIF ≺ MODELING ≺ SCOPE`.

---

## Source layout

```text
aod-lint/
├── main.typ                        # Frozen Main publication container (Typst)
├── preamble.typ                    # AF minimal templates & General Mechanics corollary blocks
├── refs.bib                        # Main foundational bibliography
├── sections/                       # Main §§1–5 (00_title .. 06_source_hygiene_and_provenance_isolation)
├── appendices/                     # Main Appendices A–D (Rubric map, Compaction, Grammar, Glossary)
├── manual/                         # Paired Manual & operational reference package (Typst)
│   ├── main.typ                    # Manual container (title page, TOC, §§1–6, Appendices A–B)
│   ├── preamble.typ                # Cross-tier #main-* reference macros
│   ├── refs.bib                    # Manual systems & toolchain bibliography
│   ├── sections/                   # Manual §§1–6 (Architecture, CLI/AST, D.E.C./Stokes, Slicing, Hygiene, Agents)
│   └── appendices/                 # Manual Appendices A–B (Self-hosting matrix, Remediation catalog)
├── contracts/
│   └── main_baseline_manifest.json # Frozen SHA-256 & DAG baseline manifest for Main + Appendices
├── src/                            # Native Rust compiler & linter engine
│   ├── lib.rs                      # Library crate entry
│   ├── main.rs                     # CLI binary (check, digest, slice, hygiene)
│   ├── models/                     # RuleAnchor, Section8Rubric (u8), AodManifest
│   └── passes/                     # typst_ast, latex, stokes, digest, slice, hygiene
├── tests/                          # Integration & self-hosting verification suite
├── scripts/
│   └── build_release_bundle.py     # Detached PDF, test-ledger & SHA-256 release packager
├── BUILD.md                        # Reproducible build & release instructions
├── CANONICAL_VERSION.txt           # Active canonical release tag
└── .zenodo.json                    # Zenodo publication metadata
```

---

## Quickstart: Run the Engine & Compile the Publications

See [BUILD.md](./BUILD.md) for full toolchain and release packaging instructions. From the repository root:

```bash
# 1. Run the Rust integration & self-hosting manuscript test suite
cargo test

# 2. Audit the treatise in strict internal AF mode (Rubric must converge to 11111111)
cargo run --release -- check . --strict

# 3. Verify the frozen Main SHA-256 baseline, Main/Manual firewall, and include DAG
cargo run --release -- digest . --baseline contracts/main_baseline_manifest.json

# 4. Verify repository provenance hygiene (zero agent state or binary artifact leakage)
cargo run --release -- hygiene .

# 5. Compile the paired Main and Manual PDFs into detached dist/
mkdir -p dist
typst compile main.typ dist/main.pdf
typst compile --root . manual/main.typ dist/manual.pdf
```

To slice the blast radius of a pull request on a Typst or LaTeX treatise (such as `rpoyau/aod`):

```bash
aod-lint digest /path/to/treatise -o /tmp/manifest.json
git -C /path/to/treatise diff main...HEAD -- '*.typ' '*.tex' > /tmp/patch.diff
aod-lint slice --manifest /tmp/manifest.json --diff /tmp/patch.diff --hops 1 --json
```

---

## Keywords and subject areas

**Core:** axiomatic fundamentalism; formal verification; premise-boundary closure; typed derivation graphs; two-pass premise hoisting; Boolean meet-semilattice rubric; discrete Stokes verification; epistemic hygiene.

**Downstream:** static AST analysis; Typst and LaTeX treatise indexing; cryptographic baseline locking; blast-radius graph slicing; Declared Edge Computation (`D.E.C.`); multi-agent inquiry protocols.

Related [arXiv subject areas](https://arxiv.org/category_taxonomy):

| Scope | Category | Content |
| --- | --- | --- |
| Core | `cs.LO` — Logic in Computer Science | Decidable premise-boundary verification and lattice rubric semantics. |
| Core | `math.LO` — Logic | Axiomatic protocol extensions and finite relational closure. |
| Core | `cs.PL` — Programming Languages | Lossless AST static analysis, dependency hoisting, and pass homomorphisms. |
| Core | `cs.SE` — Software Engineering | Tiered treatise firewalls, blast-radius slicing, and repository provenance hygiene. |
| Mathematical models | `cs.DM` — Discrete Mathematics | Finite Markov flux conservation, DAG reachability, and discrete capacity bounds. |
| Downstream | `cs.AI` — Artificial Intelligence | Context-bounded multi-agent verification loops and stratified inquiry gates. |

---

## References

Synchronized with `refs.bib` and `manual/refs.bib`:

- Poyau, R. (2025). *Axiomatic Fundamentalism (AF): A Logical Protocol for Traceable Research*. Zenodo. https://doi.org/10.5281/zenodo.17561188.
- Poyau, R. (2025). *Axiomatic Fundamentalism Calculus (AFC): The Hidden Form of Stokes*. Zenodo. https://doi.org/10.5281/zenodo.17795590.
- Poyau, R. (2025). *General Mechanics*. Zenodo. https://doi.org/10.5281/zenodo.17561404.
- Poyau, R. (2026). *Alpha↔Omega Dynamics (AΩD): The Hidden Temporal Dynamics of Stokes*. Zenodo. https://doi.org/10.5281/zenodo.20486270.
- Shannon, C. E. (1948). A Mathematical Theory of Communication. *Bell System Technical Journal*, 27(3), 379–423. https://doi.org/10.1002/j.1538-7305.1948.tb01338.x.
- Cover, T. M., and Thomas, J. A. (2006). *Elements of Information Theory* (2nd ed.). Wiley-Interscience.
- Norris, J. R. (1997). *Markov Chains*. Cambridge University Press. https://doi.org/10.1017/CBO9780511810633.
- Davey, B. A., and Priestley, H. A. (2002). *Introduction to Lattices and Order* (2nd ed.). Cambridge University Press. https://doi.org/10.1017/CBO9780511809088.
- Cormen, T. H., Leiserson, C. E., Rivest, R. L., and Stein, C. (2022). *Introduction to Algorithms* (4th ed.). MIT Press.
- Mädje, L. (2022). *Typst: A Programmable Markup Language for Typesetting*. Technical University of Berlin.
- National Institute of Standards and Technology (2015). *Secure Hash Standard (SHS)*, FIPS PUB 180-4. https://doi.org/10.6028/NIST.FIPS.180-4.
- Klabnik, S., and Nichols, C. (2023). *The Rust Programming Language* (2nd ed.). No Starch Press.

---

## Build and release

Use `scripts/build_release_bundle.py` to compile `main.pdf` and `manual.pdf`, execute the Rust test and self-audit suite, and package version-neutral release artifacts into `dist/`:

```text
main.pdf
manual.pdf
source.zip
bundle.zip
tests.txt
BUNDLE_CONTENTS_SHA256.txt
SHA256.txt
```

Generated PDFs and archives are never committed to the Git working tree; `aod-lint hygiene .` enforces a clean source-only repository on every commit and pull request.

## License

Available under the **MIT License**. See [LICENSE](./LICENSE) for details.
