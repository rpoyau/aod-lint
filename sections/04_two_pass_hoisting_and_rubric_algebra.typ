#import "../preamble.typ": *

= Two-Pass Premise Hoisting and Rubric Meet-Semilattice <sec:hoisting>

Scientific manuscripts encountered outside strict axiomatic repositories frequently interleave tacit assumptions—midstream limit operations, unnamed coupling constants, or continuum state containers—inside derivation prose while remaining algebraically non-contradictory once those hidden assumptions are granted. Conversely, a manuscript may declare clean headers yet fail internal conservation or relational closure. Extending AF to audit both strict internal treatises and arbitrary external papers requires separating *premise-factoring defects* from *internal soundness failures*.

#choice-block(
  "D3",
  "Two-Pass Premise Hoisting Operator",
  scope: [External diagnostic decompilation],
  rationale: [separation of stylistic factoring from logical soundness],
)[
  the *hoisting operator* $cal(H) : cal(G) |-> cal(G)^sharp$ acting in two sequential passes:
  + *Pass 1 (Premise Hoisting and Liability Accounting):* Partition exterior tokens $cal(V)_("ext") = cal(M)(cal(G)) union.sq cal(U)(cal(G))$ into *hoistable midstream assumptions* $cal(M)(cal(G))$ (named external principles, undeclared parameters, and continuum containers) and *unresolvable defects* $cal(U)(cal(G))$ (dangling labels and contradictory definitions). Construct the hoisted boundary bill of materials
    $ cal(B)(cal(G)) := cal(V)_(partial) union cal(M)(cal(G)), $ <eq:hoisted-bom>
    emitting a non-fatal factoring warning for each $u in cal(M)(cal(G))$ and an explicit *Ontological Liability Receipt* $Lambda(u) in {0, 1}$ flagging $Lambda(u) = 1$ whenever $u$ imports an infinite state space (@con:cardinality, e.g., `smooth manifold` or `Hilbert space`).
  + *Pass 2 (Hoisted Relational Closure):* Evaluate boundary dominance (@cor:boundary-cut) and declared algebraic conservation checks on the transformed graph $cal(G)^sharp$ with effective boundary $cal(B)(cal(G))$ and residual exterior set $cal(U)(cal(G))$.
] <ch:hoisting>

#choice-block(
  "D4",
  "Section 8 Boolean Meet-Semilattice",
  scope: [Binary reviewer rubric evaluation],
  rationale: [order-independent multi-pass grading],
)[
  the *rubric state space* $bb(B)_8 := ({0, 1}^8, and, bold(1))$ where bitwise conjunction $(r and s)_k = min(r_k, s_k)$ for $k in {0, dots, 7}$ acts as the meet operator with top element $bold(1) = (1, 1, 1, 1, 1, 1, 1, 1) equiv mono("0xFF")$. Let $Sigma_("diag")$ denote the set of emitted diagnostics $d = ("sev"(d), "rule"(d))$. The *rubric projection* $pi : cal(P)(Sigma_("diag")) -> bb(B)_8$ maps a finite diagnostic set $D subset.eq Sigma_("diag")$ to
  $ pi(D) := bold(1) and and.big_(d in D_("err")) mu("rule"(d)), quad D_("err") := { d in D : "sev"(d) in {"Error", "Fatal"} }, $ <eq:rubric-meet>
  where $mu("rule") in bb(B)_8$ clears the unique bit $b_k$ corresponding to Reviewer Rubric check $C_(k+1)$ ($k in {0, dots, 7}$) and leaves the remaining seven bits equal to $1$. Convergence holds iff $pi(D) = bold(1)$.
] <ch:rubric>

#corollary-block(
  "K2",
  "Order-Independent Rubric Convergence and Mode Separation",
  setup: [Let $D = union.big_(j=1)^m D_j$ be the union of diagnostics produced by $m >= 1$ verification passes (syntax, cross-reference, conservation, and source-hygiene passes) evaluated under either *Strict Mode* ($sigma = 1$, where $u in cal(M)(cal(G))$ carries severity `Error`) or *Diagnostic Hoisting Mode* ($sigma = 0$, where $u in cal(M)(cal(G))$ carries severity `Warning`).],
  derivation: [In $bb(B)_8$, bitwise meet $and$ is associative, commutative, and idempotent ($x and x = x$) with identity $bold(1)$. Substituting $D = D_1 union D_2$ into @eq:rubric-meet yields
  $ pi(D_1 union D_2) = bold(1) and and.big_(d in (D_1 union D_2)_("err")) mu("rule"(d)) = pi(D_1) and pi(D_2) . $ <eq:rubric-homomorphism>
  By induction over $m$ passes, $pi(union.big_(j=1)^m D_j) = and.big_(j=1)^m pi(D_j)$ is invariant under arbitrary permutations, shardings, or duplicate emissions of the pass schedule. Furthermore, since Pass 1 assigns severity `Error` to $cal(M)(cal(G))$ when $sigma = 1$ and severity `Warning` when $sigma = 0$, we obtain the exact inclusion $D_("err")^(sigma = 0) subset.eq D_("err")^(sigma = 1)$, and monotonicity of $and$ implies $pi_1(cal(G)) <= pi_0(cal(G))$ componentwise.],
  specialization: [When $cal(U)(cal(G)) = emptyset$ and $cal(M)(cal(G)) != emptyset$, Diagnostic Mode yields $pi_0(cal(G)) = bold(1)$ accompanied by $|cal(M)(cal(G))| >= 1$ hoisted premise receipts (`CONDITIONALLY CONSISTENT`), whereas Strict Mode clears the corresponding `AF3`/`AF4`/`RF3` rubric bits ($pi_1(cal(G)) != bold(1)$).],
  calibration: [Representing $r in bb(B)_8$ as an unsigned 8-bit integer (`u8`) maps full AF convergence directly to the exact machine invariant $r = 255$ (`0b11111111`), requiring a single byte per audited target.],
  falsification: [Execute two distinct pass orderings $(P_1, P_2)$ and $(P_2, P_1)$ on any manuscript corpus and compare the resulting 8-bit words; any discrepancy $pi(P_1 union P_2) != pi(P_2 union P_1)$ or any Strict-Mode pass on a manuscript with $cal(M)(cal(G)) != emptyset$ falsifies @eq:rubric-homomorphism.],
  provenance: [@ax:accountability, @con:cardinality, @ch:hoisting, @ch:rubric, @eq:hoisted-bom, @eq:rubric-meet, @eq:rubric-homomorphism.],
) <cor:hoisting-rubric>

#literature-note[
  Allowed versus disallowed moves and the 8-item binary Reviewer Rubric: @poyau2025af[§6]; @poyau2025af[§8]. Distributive lattice and finite meet-semilattice homomorphisms: @davey2002[Ch. 2, §2.19].
]
