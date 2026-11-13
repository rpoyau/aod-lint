#import "../preamble.typ": *

= Source-Tree Epistemic Hygiene and Stratified Inquiry <sec:hygiene-inquiry>

Modern scientific repositories are authored and audited jointly by human researchers and autonomous software agents. Without formal source-tree boundary rules, agent execution transcripts, private session identifiers, ephemeral container paths, unchecked binary handoff archives, and multi-megabyte intermediate state dumps leak into version control alongside mathematical source. Under `AF6` (Scope Discipline) and `AF7` (Traceability), an irreproducible or session-tainted artifact checked into a scientific repository is an ungrounded exterior object: it inflates repository entropy, leaks private provenance, and substitutes static cached output for executable verification.

#choice-block(
  "D7",
  "Source-Tree Provenance Partition",
  scope: [Version-controlled scientific repositories],
  rationale: [quarantine of non-source execution state],
)[
  a partition of tracked repository paths $cal(R) = cal(S)_("src") union.sq cal(S)_("leak")$ where *admissible scientific source* $cal(S)_("src")$ consists strictly of human-auditable manuscripts, bibliographies, vector figure assets, declared schema contracts, compact input fixtures, and deterministic verification code, while *prohibited state leakage* $cal(S)_("leak")$ comprises:
  + agent prompt and orchestration directories or instruction dumps;
  + private cloud session URIs, personal library file tokens, and ephemeral container or host scratch paths;
  + bundled handoff archives, compressed execution trace dumps, and compiled non-figure root PDFs; and
  + tracked non-figure data blobs exceeding a declared byte ceiling $B_("max")$ that are deterministically regenerable from $(cal(S)_("src"), cal(U))$.
] <ch:hygiene>

#choice-block(
  "D8",
  "Stratified Axiomatic Inquiry Gate",
  scope: [Collaborative and multi-agent escalation],
  rationale: [elimination of ungrounded open-ended design drift],
)[
  an ordered five-rung *escalation lattice* isomorphic to the generative AF ladder,
  $ "Distinction" prec "Relation" prec "Motif" prec "Modelling" prec "Scope", $ <eq:inquiry-ladder>
  requiring every ambiguity interrupt $Q = (g, ell, {o_1, dots, o_k})$ emitted during authoring or review to bind to a single gate $g$ in @eq:inquiry-ladder, cite the exact blocked source locus $ell$, and enumerate finite mutually exclusive axiomatic resolutions ${o_1, dots, o_k}$ rather than open-ended stylistic prompts.
] <ch:inquiry>

#corollary-block(
  "K5",
  "Artifact-Free Reproducibility and Leakage Quarantine",
  setup: [Let $cal(R)$ be a tracked repository tree equipped with deterministic evaluation scripts $cal(U)$ mapping $cal(S)_("src")$ to derived outputs $Y = cal(U)(cal(S)_("src"))$, and let $L(cal(R)) := |cal(R) inter cal(S)_("leak")|$ count tracked elements violating @ch:hygiene.],
  derivation: [For any deterministically computable output $Y = cal(U)(cal(S)_("src"))$, conditional algorithmic complexity satisfies $K(Y | cal(S)_("src"), cal(U)) = O(1)$: storing a static snapshot $hat(Y) in cal(R)$ contributes zero independent axiomatic or empirical premise information while introducing the stale-state divergence hazard $hat(Y) != cal(U)(cal(S)_("src"))$ whenever $cal(S)_("src")$ advances. Similarly, any file containing host-local scratch paths or private session tokens fails environment-independent replay under @ax:accountability. Enforcing
  $ L(cal(R)) = 0 quad text("and") quad cal(U)_("--check")(cal(S)_("src")) = "PASS" $ <eq:hygiene-invariant>
  ensures both zero state-leakage contamination (`C5` demonstration/scope isolation in @ch:rubric) and self-contained verification from clean source checkouts.],
  specialization: [When an escalation arises during automated repair of @eq:hygiene-invariant or @cor:boundary-cut, @ch:inquiry restricts the inquiry gate $g$ to the lowest unresolved rung in @eq:inquiry-ladder (`Distinction` before `Relation`, `Relation` before `Motif`, `Motif` before `Modelling`, `Modelling` before `Scope`), preventing downstream parameter tuning before upstream distinctions are fixed.],
  calibration: [Setting default blob threshold $B_("max") = 2^20$ bytes ($1$ MiB) separates compact human-auditable CSV/JSON test tables from multi-megabyte serialized execution state dumps while exempting declared `figures/` assets.],
  falsification: [Commit an agent configuration file, a private session token URI, an ephemeral container scratch path, or a compiled root `main.pdf` into a test repository; if the hygiene pass returns $L(cal(R)) = 0$ or leaves Rubric bit `C5` ($b_4$) set to $1$, @eq:hygiene-invariant is falsified.],
  provenance: [@ax:boundary, @ax:accountability, @ch:rubric, @ch:hygiene, @ch:inquiry, @eq:inquiry-ladder, @eq:hygiene-invariant.],
) <cor:hygiene>

#literature-note[
  Scope discipline (`AF6`), traceability (`AF7`), and version-controlled release discipline: @poyau2025af[§3]; @poyau2025af[§10]. Source-neutral release packaging and executed fixture verification: @poyau2026aod[Manual §1].
]
