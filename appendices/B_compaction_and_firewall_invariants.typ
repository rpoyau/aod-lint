#import "../preamble.typ": *

= Compaction Boundaries and Firewall Invariants <app:compaction>

When `AOD-Lint` audits manuscripts within the Alpha-Omega Dynamics (`AOD`) motif tier over `AFC`, structural verification enforces strict separation between topological trace execution, motif typing, and editorial source management (@cor:stokes, @cor:firewall-slice). @tab:compaction-invariants codifies the protected invariants.

#figure(
  table(
    columns: (auto, 1fr, 1fr),
    inset: 6pt,
    align: (left, left, left),
    table.header(
      [*Invariant Class*], [*Admissible Formal Meaning*], [*Prohibited Conflation*],
    ),
    [Live Dwell Reduction],
    [Active idempotent reduction of repeated undifferentiated dwell along an executed walk, leaving the irreducible committed trace.],
    [Treating history as a static placed variable rather than the surviving trace of committed boundary steps.],
    [Interface Cancellation],
    [Exact cancellation of interior antisymmetric edge pairs $phi_(i j) + phi_(j i) = 0$ in @eq:stokes-balance while retaining lineage provenance.],
    [Equating Git line-count diffs or editorial text deletion with mathematical Stokes cancellation.],
    [Hinge Capacity Scope],
    [Conditional role restriction onto two opposed branch continuations ${-1, +1}$ when same-resolution hinge dwell is saturated.],
    [Elevating conditional ternary-to-binary hinge saturation to an unconditional universal 2-bit channel law (`RF5`).],
    [Minimal Closure],
    [At least two committed phase-bearing traversals together with a certified retained-start return witness.],
    [Admitting a closed cycle from dwell repetition or a bare cut-index increment alone.],
    [Harmonic Scope],
    [Downstream finite abelian character representation of an *already admitted* discrete recurrence.],
    [Invoking Fourier decomposition upstream to certify cycle or wave existence.],
    [`Main` / `Manual` Split],
    [`Main` owns frozen ontology, hypotheses, and proofs; `Manual` instantiates `D.E.C.` ledgers, code, and sector maps.],
    [Referencing `Manual` labels from `Main` (@eq:tier-firewall) or re-axiomatizing `Main` inside `Manual`.],
  ),
  caption: [Protected topological and architectural boundaries enforced across `AFC` and `AOD` manuscripts.],
) <tab:compaction-invariants>

#literature-note[
  No elevation of modelling limitations (`RF5`) and bookkeeping role of information (`RF4`): @poyau2025af[§4]. AOD motif typing, hinge saturation, finite harmonics, and `Main`/`Manual` architecture: @poyau2026aod[Main §3]; @poyau2026aod[Main §5].
]
