#import "../preamble.typ": *

= Finite Stokes Verification, Tier Firewalls, and Graph Slicing <sec:stokes-slicing>

When a manuscript instantiates quantitative discrete transition models or spans a multi-volume treatise split between foundational proofs and operational worked ledgers, structural syntax checks combine with two exact algebraic verification layers: (i) declared finite boundary conservation under the Axiomatic Fundamentalism Calculus (AFC), and (ii) tiered quotient-graph immutability and bounded blast-radius slicing across `Main`, `Appendix`, and `Manual` files. In strict compliance with AF scope separation, AFC's finite Markov flux calculus remains an explicitly declared finite module rather than an unstated physical postulate.

#choice-block(
  "D5",
  "Declared Finite AFC Flux and Trit Capacity Module",
  scope: [Discrete transition models over finite distinction sets],
  rationale: [exact algebraic conservation and information-rate auditing],
)[
  a finite state system $(V, p, P)$ over a non-empty finite set of distinctions $V$ with probability vector $p : V -> [0, 1]$ ($sum_(i in V) p(i) = 1$) and row-stochastic transition kernel $P = (P_(i j))_(i, j in V)$ ($P_(i j) >= 0$, $sum_(j in V) P_(i j) = 1$). Define the *antisymmetric net edge flux*, *vertex divergence*, and *directed region boundary* for any subset $R subset.eq V$ by
  $ phi_(i j) := p(i) P_(i j) - p(j) P_(j i), quad op("div")(i) := sum_(j in V) phi_(i j), quad partial R := { (i, j) in R times (V without R) }, $ <eq:afc-flux-defs>
  and define the base-3 *trit channel capacity* of $P$ by $C_3(P) := max_p I_p^((3))(X; Y)$, where $I_p^((3))(X; Y) = sum_(i, j in V) p(i) P_(i j) log_3 (P_(i j) / (sum_k p(k) P_(k j)))$.
] <ch:afc-module>

#choice-block(
  "D6",
  "Tiered Treatise Quotient Graph and Blast-Radius Slice",
  scope: [Multi-file `.typ` and `.tex` repositories],
  rationale: [Main/Manual firewall and context-bounded review],
)[
  a treatise file index $cal(M) = (cal(F), cal(E)_cal(F), tau_cal(F), H)$ where each source file $f in cal(F)$ carries a *document tier* $tau_cal(F)(f) in {"Main", "Appendix", "Manual", "Other"}$, cryptographic digest $H(f) := "SHA256"(f) in {0, 1}^256$, and *frozen indicator* $"frozen"(f) = 1 <==> tau_cal(F)(f) in {"Main", "Appendix"}$. Let $cal(E)_cal(F) subset.eq cal(F) times cal(F)$ be the union of file-level `#include`/`\input` edges and cross-file label-reference edges, with symmetric adjacency $tilde(cal(E))_cal(F) := cal(E)_cal(F) union cal(E)_cal(F)^(-1)$. Given a patch touching file set $Delta subset.eq cal(F)$ and hop bound $h in bb(N)_0$, the *$h$-hop blast-radius slice* is
  $ cal(N)_h (Delta; cal(M)) := { g in cal(F) : min_(f in Delta) d_(tilde(cal(E))_cal(F))(f, g) <= h } . $ <eq:hop-slice>
] <ch:slicing>

#corollary-block(
  "K3",
  "Exact AFC Interior Annihilation and Capacity Bound",
  setup: [Let $(V, p, P)$ be a finite distinction transition system under @ch:afc-module and let $R subset.eq V$ be any declared region with interior sum $S_("int")(R) := sum_(i in R) sum_(j in R) phi_(i j)$, divergence sum $S_("div")(R) := sum_(i in R) op("div")(i)$, and boundary flux $S_(partial)(R) := sum_((i, j) in partial R) phi_(i j)$.],
  derivation: [From @eq:afc-flux-defs, swapping indices $(i, j) <-> (j, i)$ yields exact edge antisymmetry $phi_(i j) = -phi_(j i)$ and diagonal vanishing $phi_(i i) = 0$. Pairing unordered interior pairs ${i, j} subset.eq R$ gives $S_("int")(R) = sum_({i, j} subset.eq R) (phi_(i j) + phi_(j i)) = 0$. Splitting $op("div")(i) = sum_(j in R) phi_(i j) + sum_(j in V without R) phi_(i j)$ and summing over $i in R$ cancels $S_("int")(R)$ identically:
  $ sum_(i in R) op("div")(i) = S_("int")(R) + sum_((i, j) in partial R) phi_(i j) = sum_((i, j) in partial R) phi_(i j) . $ <eq:stokes-balance>
  Independently, for any one-step observable readout $Z$ generated across kernel $P$ ($X -> Y -> Z$), the data-processing inequality in base $3$ enforces $R_("trit") := I_p^((3))(X; Z) <= I_p^((3))(X; Y) <= C_3(P)$.],
  specialization: [In Declared Edge Computation (`D.E.C.`) ledgers with exact rational weights $w(e; B) in bb(Z)_(>= 0)$ and partition sum $Z(B) = sum_e w(e; B) > 0$, setting $P(e; B) = w(e; B) / Z(B)$ with admissibility gate $op("adm")(e; B) = 0 ==> P(e; B) = 0$ evaluates @eq:stokes-balance over $bb(Q)$ with zero floating-point rounding residual ($epsilon.alt = 0$).],
  calibration: [In IEEE-754 double-precision numerical fixtures, numerical tolerance $epsilon.alt > 0$ bounds floating-point representation noise while preserving exact structural antisymmetry on every declared edge pair.],
  falsification: [Any candidate flux table with $|phi_(i j) + phi_(j i)| > epsilon.alt$, $|S_("int")(R)| > epsilon.alt$, $|S_("div")(R) - S_(partial)(R)| > epsilon.alt$, or claimed per-step distinguishable trit rate $R_("trit") > C_3(P)$ falsifies @ch:afc-module and triggers a fatal verification failure.],
  provenance: [@ax:boundary, @ch:afc-module, @eq:afc-flux-defs, @eq:stokes-balance.],
) <cor:stokes>

#corollary-block(
  "K4",
  "One-Way Tier Firewall and 1-Hop Premise Completeness",
  setup: [Let $cal(M) = (cal(F), cal(E)_cal(F), tau_cal(F), H)$ be a tiered treatise index under @ch:slicing with frozen set $cal(F)_("frozen") = { f in cal(F) : "frozen"(f) = 1 }$, baseline manifest $cal(M)_0$, and a candidate revision patch touching $Delta subset.eq cal(F)_("Manual")$.],
  derivation: [Suppose the *one-way tier firewall* holds:
  $ cal(E)_cal(F) inter (cal(F)_("frozen") times cal(F)_("Manual")) = emptyset, quad text("and") quad forall f in cal(F)_("frozen"), space H(f) = H_0(f) . $ <eq:tier-firewall>
  Because every frozen file has byte-identical preimage under collision-resistant SHA-256 commitment $H(f) = H_0(f)$ and zero outgoing reference or include edges into $cal(F)_("Manual")$, the restricted derivation subgraph $cal(G)|_(cal(F)_("frozen"))$ is invariant under any modification confined to $Delta subset.eq cal(F)_("Manual")$. Conversely, every frozen `Main` or `Appendix` label `@lbl` directly cited by any modified file $f in Delta$ induces a directed edge $(f, g) in cal(E)_cal(F)$ with $g in cal(F)_("frozen")$, whence $d_(tilde(cal(E))_cal(F))(f, g) = 1$ and $g in cal(N)_1 (Delta; cal(M))$.],
  specialization: [For $h = 1$, $cal(N)_1 (Delta; cal(M))$ contains all modified `Manual` files together with 100% of their immediate `Main` premise files while excluding unrelated `Manual` chapters $u$ satisfying $d_(tilde(cal(E))_cal(F))(Delta, u) >= 2$.],
  calibration: [On a treatise of $|cal(F)|$ files with bounded per-chapter citation degree $Delta_("max") << |cal(F)|$, inspecting $cal(N)_h (Delta; cal(M))$ bounds review context size by $|Delta| (1 + Delta_("max")^h)$ rather than $O(|cal(F)|)$.],
  falsification: [Mutate a single byte in any $f in cal(F)_("frozen")$ or insert a single reference `@sec:manual-demo` inside a `Main` file; if the baseline digest verifier fails to report a frozen-hash mismatch or a `FirewallViolation`, @eq:tier-firewall is falsified.],
  provenance: [@ax:boundary, @con:dag, @ch:slicing, @eq:hop-slice, @eq:tier-firewall.],
) <cor:firewall-slice>

#literature-note[
  Discrete Stokes identity and base-3 Markov capacity in AFC: @poyau2025afc[§5, Cor. 1]; @poyau2025afc[App. A]. Main/Manual architectural separation and Declared Edge Computation (`D.E.C.`) ledger in AOD: @poyau2026aod[Main §5]; @poyau2026aod[Manual §4]. Data-processing inequality and discrete channel capacity: @shannon1948[§12, Theorem 11]; @cover2006[Ch. 2, §2.8]. Finite Markov chain balance: @norris1997[Ch. 1, §1.9].
]
