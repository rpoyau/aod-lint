#import "../preamble.typ": *

= Scope, Crate Topology, and Module-to-Corollary Map <sec:man-scope>

This Manual instantiates the formal verification calculus established in #main-sec("sec:axioms", [Main §1]) through #main-sec("sec:hygiene-inquiry", [Main §5]) as a deterministic, memory-safe Rust engine (`aod-lint`). In strict observance of demonstration isolation (`AF §5.6` and #main-cor("cor:firewall-slice", [Main Cor. K4])), no new axioms or ontological commitments originate in this Manual; every data structure, CLI subcommand, and integration fixture executes an already-declared mathematical object from `Main`.

== Source Layout and Tier Separation <sec:man-layout>

The repository combines the paired Typst treatise (`Main` and `Manual`) with the Rust verification crate under `src/` and its executable invariant suite under `tests/`:

```text
aod-lint/
├── main.typ, preamble.typ, refs.bib   # Frozen Main treatise container & bibliography
├── sections/00..06_*.typ              # Frozen Main §§1–5 (Axioms A1–A2, Choices D1–D8, Cor. K1–K5)
├── appendices/A..D_*.typ              # Frozen Main Appendices A–D
├── manual/                            # Active Manual & operational reference package
│   ├── main.typ, preamble.typ, refs.bib
│   ├── sections/00..05_*.typ
│   └── appendices/A..B_*.typ
├── contracts/
│   └── main_baseline_manifest.json    # Cryptographic SHA-256 baseline lock for frozen Main
├── src/                               # Native Rust verification library & CLI binary
│   ├── lib.rs, main.rs
│   ├── models/{diagnostics,manifest,rubric}.rs
│   └── passes/{typst_ast,latex,stokes,digest,slice,hygiene}.rs
└── tests/                             # Deterministic integration & self-hosting tests
```

== Rust Module-to-Main Provenance Map <sec:man-module-map>

@tab:man-module-map binds each Rust source module in `src/` to its governing mathematical definition and corollary in `Main`.

#figure(
  table(
    columns: (auto, auto, 1fr),
    inset: 6pt,
    align: (left, center, left),
    table.header(
      [*Rust Module*], [*Main Anchor*], [*Instantiated Operator*],
    ),
    [`src/models/diagnostics.rs`], [#main-ax("ax:accountability", [Main Axiom A2])], [Total `RuleAnchor` enumeration (`RF1`--`RF5`, `AF1`--`AF7`, `§7.1`--`§7.4`, Hygiene) and `Diagnostic` record.],
    [`src/models/rubric.rs`], [#main-cor("cor:hoisting-rubric", [Main Cor. K2])], [`Section8Rubric(u8)` meet-semilattice $bb(B)_8$ with homomorphic `apply_diagnostics` (`0xFF` convergence).],
    [`src/models/manifest.rs`], [#main-app("app:manifest-grammar", [Main App. C])], [Serializable `AodManifest`, `FileRecord`, `DocumentTier`, `EdgeRecord`, and `FirewallViolation` schemas.],
    [`src/passes/typst_ast.rs`], [#main-cor("cor:boundary-cut", [Main Cor. K1])], [Lossless `typst-syntax` AST traversal, `#literature-note` scoping, and two-pass premise hoisting.],
    [`src/passes/latex.rs`], [#main-cor("cor:boundary-cut", [Main Cor. K1])], [Paired `.tex` verifier with brace-balanced `\aodliteraturenote` tracking and `xr-hyper` support.],
    [`src/passes/stokes.rs`], [#main-cor("cor:stokes", [Main Cor. K3])], [`AFCStokesChecker` verifying edge antisymmetry, interior annihilation, and region boundary balance.],
    [`src/passes/digest.rs`], [#main-cor("cor:firewall-slice", [Main Cor. K4])], [`AodDigestPass` computing SHA-256 locks, Tarjan cycle detection, and `Main -> Manual` firewall audits.],
    [`src/passes/slice.rs`], [#main-cor("cor:firewall-slice", [Main Cor. K4])], [`AodSlicePass` extracting $h$-hop reachability `ContextBundle` neighborhoods from unified diffs.],
    [`src/passes/hygiene.rs`], [#main-cor("cor:hygiene", [Main Cor. K5])], [`RepoHygienePass` quarantining agent configs, personal session URIs, scratch paths, and binary dumps.],
  ),
  caption: [Traceability map from Rust crate modules under `src/` to `Main` formal constructs.],
) <tab:man-module-map>

#literature-note[
  Demonstration isolation in the AF authoring protocol: @poyau2025af[§5]. Ownership and zero-cost abstraction guarantees in Rust: @klabnik2023rust[Ch. 4, §4.1].
]
