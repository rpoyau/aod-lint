#import "../preamble.typ": *

= Self-Hosting Verification Matrix and Benchmark Ledger <app:man-self-audit>

A verification protocol must satisfy its own invariants (`AF7`). `AOD-Lint` is self-hosted: the Rust integration test `tests/test_self_manuscript.rs` executes every verification pass against this repository's own `Main` and `Manual` Typst sources and `contracts/main_baseline_manifest.json` on every test run. @tab:man-self-matrix records the verified invariant gates.

#figure(
  table(
    columns: (auto, auto, 1fr, auto),
    inset: 5.5pt,
    align: (left, left, left, center),
    table.header(
      [*Gate ID*], [*Executable Command / Test*], [*Verified Invariant*], [*Expected State*],
    ),
    [`G1-Check`], [`aod-lint check . --strict`], [Zero midstream imports, zero continuum leaks, 100% `#literature-note` localization across all `.typ` files.], [`11111111` (`0xFF`)],
    [`G2-Digest`], [`aod-lint digest . --baseline ...`], [Zero `Main -> Manual` firewall leaks, zero include cycles, exact SHA-256 match on all 13 frozen `Main`/`Appendix` files.], [`0` violations],
    [`G3-Slice`], [`aod-lint slice --hops 1`], [1-hop blast-radius slice from `Manual` fixture resolves its exact `Main` corollary files without false positives.], [`0` frozen touches],
    [`G4-Hygiene`], [`aod-lint hygiene .`], [Zero tracked agent prompts, zero personal cloud tokens, zero scratch paths, zero checked-in PDFs or archives.], [`11111111` (`0xFF`)],
    [`G5-Stokes`], [`cargo test --test test_stokes`], [Exact antisymmetry $phi_(i j) = -phi_(j i)$, $S_("int")(R) = 0$, and $S_("div")(R) = S_(partial)(R) = 5/2$.], [`PASS`],
    [`G6-Build`], [`typst compile main.typ` & `manual/main.typ`], [Clean paired PDF compilation of `main.pdf` and `manual.pdf` with zero unresolved labels or warnings.], [`exit 0`],
  ),
  caption: [Self-hosting verification ledger executed on `rpoyau/aod-lint`.],
) <tab:man-self-matrix>

#literature-note[
  Traceability and concrete falsification testing (`AF7`): @poyau2025af[§3].
]
