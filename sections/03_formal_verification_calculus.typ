#import "../preamble.typ": *

= Typed Derivation Graphs and Epistemic Boundary Cuts <sec:calculus>

To evaluate @ax:boundary and @ax:accountability mechanically on arbitrary scientific manuscripts, we equip the parsed syntax tree of a treatise with a typed finite graph representation.

#choice-block(
  "D1",
  "Typed Manuscript Derivation Graph",
  scope: [Parsed `.typ` and `.tex` manuscripts],
  rationale: [exact representation of premise and reference topology],
)[
  a manuscript graph to be a finite tuple $cal(G) = (cal(V), cal(E), tau, alpha)$ where:
  + $cal(V) = cal(V)_(partial) union.sq cal(V)_(circle.small) union.sq cal(V)_("ext")$ is a finite vertex partition into *boundary premises* $cal(V)_(partial) = cal(V)_("Ax") union.sq cal(V)_("Ch")$ (declared Axioms and Modelling Choices), *interior claims* $cal(V)_(circle.small) = cal(V)_("Con") union.sq cal(V)_("Cor") union.sq cal(V)_("Dem")$ (Consequences, Corollaries, and Demonstrations), and *exterior ungrounded tokens* $cal(V)_("ext")$ (midstream external laws, undeclared absolutes, and unresolved labels);
  + $cal(E) subset.eq cal(V) times cal(V)$ is the finite set of directed dependency and citation edges $(u, v)$ indicating that statement $v$ invokes $u$;
  + $tau : cal(V) -> {"Ax", "Ch", "Con", "Cor", "Dem", "Ext"}$ assigns each vertex its syntactic role; and
  + $alpha : cal(E) -> {0, 1}$ is the *anchor ambiguity indicator*, taking value $alpha(e) = 0$ if edge $e$ resolves to a unique internal label or carries an explicit external locus supplement, and $alpha(e) = 1$ if $e$ is a broad range (such as `see §§2–5`) or an unanchored external citation.
] <ch:graph>

#choice-block(
  "D2",
  "Exterior Premise Flux and Reference Deficit",
  scope: [Interior vertex subsets $U subset.eq cal(V)_(circle.small)$],
  rationale: [additive boundary accounting],
)[
  the *exterior premise in-degree* of $v in cal(V)_(circle.small)$ and the *aggregate exterior premise flux* into $U subset.eq cal(V)_(circle.small)$ by
  $ d_("ext")^(-)(v) := |{ u in cal(V)_("ext") : (u, v) in cal(E) }|, quad Phi_("ext")(U) := sum_(v in U) d_("ext")^(-)(v), $ <eq:ext-flux>
  together with the *anchor ambiguity count* $A_("amb")(cal(G)) := sum_(e in cal(E)) alpha(e)$ and the *demonstration back-coupling count*
  $ B_("dem")(cal(G)) := |{ (u, v) in cal(E) : u in cal(V)_("Dem") and v in cal(V)_(partial) union cal(V)_("Con") union cal(V)_("Cor") }| . $ <eq:dem-leak>
] <ch:flux>

#corollary-block(
  "K1",
  "Epistemic Boundary Cut Sufficiency",
  setup: [Let $cal(G) = (cal(V), cal(E), tau, alpha)$ be a typed manuscript graph under @ch:graph with flux functionals $Phi_("ext")$, $A_("amb")$, and $B_("dem")$ defined in @ch:flux, and assume the interior subgraph $cal(G)|_(cal(V)_(partial) union cal(V)_(circle.small))$ has no isolated interior vertices.],
  derivation: [Partition incoming edges of any interior non-demonstration claim $v in cal(V)_("Con") union cal(V)_("Cor")$ across the disjoint vertex decomposition $cal(V) = cal(V)_(partial) union.sq (cal(V)_("Con") union cal(V)_("Cor")) union.sq cal(V)_("Dem") union.sq cal(V)_("ext")$. Summing over $U_("core") = cal(V)_("Con") union cal(V)_("Cor")$ yields the exact partition identity
  $ sum_(v in U_("core")) d^(-)(v) = sum_(v in U_("core")) d_(partial)^(-)(v) + sum_(v in U_("core")) d_("core")^(-)(v) + B_("dem", "core")(cal(G)) + Phi_("ext")(U_("core")) . $ <eq:cut-partition>
  When $Phi_("ext")(cal(V)_(circle.small)) = 0$, $B_("dem")(cal(G)) = 0$, and $cal(G)|_(cal(V)_(partial) union cal(V)_(circle.small))$ is acyclic (@con:dag), finite induction on topological rank $k >= 1$ implies that every $v in U_("core")$ possesses a non-empty ancestor set contained strictly in $cal(V)_(partial)$, while demonstrations $cal(V)_("Dem")$ read from $cal(V)_(partial) union U_("core")$ without feeding backward.],
  specialization: [Restricting $U_("core")$ to immediate consequences $cal(V)_("Con")$ requires additionally $d_("Ch")^(-)(v) = 0$ for all $v in cal(V)_("Con")$ (`AF2`: immediate consequences depend on $cal(V)_("Ax")$ and pure mathematics alone, prior to Modelling Choices $cal(V)_("Ch")$).],
  calibration: [Each unit increment $Delta Phi_("ext") = 1$ corresponds to exactly one syntactic violation locus `(file, line, column)` in the parsed abstract syntax tree, yielding unit-slope diagnostic counting with zero free scale parameters.],
  falsification: [Inject a single midstream external postulate edge $(u_("ext"), v)$ with $u_("ext") in cal(V)_("ext")$ (for instance, invoking `by Gauss's law` inside a corollary derivation) or a reverse demonstration edge $(u_("dem"), v_("cor"))$. If the verifier reports $Phi_("ext")(cal(V)_(circle.small)) = 0$ and $B_("dem")(cal(G)) = 0$, @eq:cut-partition is falsified.],
  provenance: [@ax:boundary, @con:dag, @ch:graph, @ch:flux, @eq:ext-flux, @eq:dem-leak, @eq:cut-partition.],
) <cor:boundary-cut>

#literature-note[
  Mechanical labelling and single-anchor referencing: @poyau2025af[§7]. Corollary-block structural presentation and localized literature provenance: @poyau2025gm[§1].
]
