#import "../preamble.typ": *

= Declared Edge Computation (D.E.C.) Ledger and Stokes Fixtures <sec:man-stokes>

This section instantiates the finite conservation module of #main-choice("ch:afc-module", [Main Choice D5]) and #main-cor("cor:stokes", [Main Cor. K3]) through two worked verification fixtures: (i) the three-vertex antisymmetric boundary-flux audit executed in `tests/test_stokes.rs`, and (ii) the nine-column Declared Edge Computation (`D.E.C.`) hinge-slide ledger over the four-dimensional hypercube $Q_4$.

== Fixture 1: Three-Node Region Cut and Interior Annihilation <sec:man-stokes-fixture>

Consider the finite distinction set $V = {1, 2, 3}$ with region $R = {1, 2}$ and exterior complement $V without R = {3}$, audited by `AFCStokesChecker` (`src/passes/stokes.rs`). The declared antisymmetric edge fluxes $phi_(i j) = -phi_(j i)$ are:

$ phi_(1 2) = 3 / 2, quad phi_(2 1) = -3 / 2, quad phi_(2 3) = 2, quad phi_(3 2) = -2, quad phi_(1 3) = 1 / 2, quad phi_(3 1) = -1 / 2 . $ <eq:man-three-node-flux>

Evaluating interior cancellation, vertex divergences, and directed boundary flux across $partial R = {(1, 3), (2, 3)}$ yields:

+ *Interior pair annihilation:* $S_("int")(R) = phi_(1 2) + phi_(2 1) = 3/2 - 3/2 = 0$.
+ *Vertex divergences:* $op("div")(1) = phi_(1 2) + phi_(1 3) = 2$, $op("div")(2) = phi_(2 1) + phi_(2 3) = 1/2$, and $op("div")(3) = phi_(3 1) + phi_(3 2) = -5/2$ (global sum $sum_(i in V) op("div")(i) = 0$).
+ *Exact Stokes balance:*
  $ S_("div")(R) = op("div")(1) + op("div")(2) = 5 / 2 = phi_(1 3) + phi_(2 3) = S_(partial)(R) . $ <eq:man-stokes-verified>

Perturbing any single oriented entry (for example setting $phi_(2 1) = -1$) breaks antisymmetry ($|phi_(1 2) + phi_(2 1)| = 1/2 > 10^(-9)$) and produces a non-zero interior leak $S_("int")(R) = 1/2$, triggering an immediate `AFCStokesChecker` failure.

== Fixture 2: Nine-Column D.E.C. Hinge-Slide Audit at $Q_4$ Origin <sec:man-dec-ledger>

In `AOD` motif execution over `AFC`, each step at vertex $v = (0, 0, 0, 0) in V(Q_4)$ is recorded in the nine-column `D.E.C.` ledger schema $(B, v, e, v_e, sigma(e), op("adm")(e; B), w(e; B), P(e; B), op("route")(e; B))$. @tab:man-dec-slide demonstrates exact rational kernel normalization when additional same-resolution hinge dwell $(e_3, 0_D)$ is blocked ($op("adm")(e_3, 0_D; B|_t) = 0$) while slot $e_3$ re-enters the post-block slide pool as branch state $sigma' = +1$ with weights $w' = (3, 3, 1)$ and partition sum $Z_("slide") = 7$.

#figure(
  table(
    columns: (auto, auto, auto, auto, auto, auto, auto, auto, auto),
    inset: 4.5pt,
    align: center,
    table.header(
      [$B$], [$v$], [$e$], [$v_e$], [$sigma(e)$], [$op("adm")$], [$w$], [$P(e; B)$], [$op("route")$],
    ),
    [$B|_t$], [`0000`], [$e_3$], [`0010`], [$0_D$], [$0$], [$0$], [$0$], [`blocked`],
    [$B|_t$], [`0000`], [$e_1$], [`1000`], [$-1$], [$1$], [$3$], [$3/7$], [`returned`],
    [$B|_t$], [`0000`], [$e_2$], [`0100`], [$-1$], [$1$], [$3$], [$3/7$], [`returned`],
    [$B|_t$], [`0000`], [$e_3$], [`0010`], [$+1$], [$1$], [$1$], [$1/7$], [`outgoing`],
  ),
  caption: [Exact rational nine-column `D.E.C.` ledger audit under blocked same-resolution dwell and post-block hinge slide ($sum_e P(e; B|_t) = 3/7 + 3/7 + 1/7 = 1$).],
) <tab:man-dec-slide>

For window length $|omega| = 2$, reflection duration $"RD" = 5$, and ternary channel factor $C_3 = 3$ on the outgoing tunnel branch $(e_3, +1)$, clipped participation evaluates to $rho_omega^D = min(5, 2) = 2$, channel pressure $p^D = 3 times 2 = 6$, and weighted outgoing tunnel contribution $T_("tunnel")^D = (1/7) times 6 = 6/7 in bb(Q)$.

#literature-note[
  AFC discrete Stokes corollary: @poyau2025afc[§5, Cor. 1]. Nine-column Declared Edge Computation (`D.E.C.`) ledger and hinge-slide renormalization: @poyau2026aod[Manual §4].
]
