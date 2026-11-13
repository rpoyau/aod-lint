#import "../preamble.typ": *

= Glossary and Symbol Table <app:glossary>

Nothing enters the verification model beyond the finite objects, partitions, and operators listed in @tab:symbol-glossary (`RF2` model closure).

#figure(
  table(
    columns: (auto, 1fr, auto),
    inset: 6pt,
    align: (left, left, center),
    table.header(
      [*Symbol*], [*Definition*], [*Origin*],
    ),
    [$cal(D)$], [Declared finite family of distinctions in a manuscript.], [@sec:axioms],
    [$cal(G) = (cal(V), cal(E), tau, alpha)$], [Typed finite manuscript derivation and reference graph.], [@ch:graph],
    [$cal(V)_(partial), cal(V)_(circle.small), cal(V)_("ext")$], [Boundary premises, interior claims, and exterior ungrounded tokens.], [@ch:graph],
    [$Phi_("ext")(U)$], [Aggregate exterior premise flux entering interior subset $U$.], [@eq:ext-flux],
    [$A_("amb")(cal(G)), B_("dem")(cal(G))$], [Anchor ambiguity count and demonstration back-coupling count.], [@ch:flux],
    [$cal(H), cal(B)(cal(G)), Lambda(u)$], [Two-pass hoisting operator, hoisted bill of materials, and liability receipt.], [@ch:hoisting],
    [$bb(B)_8 = ({0, 1}^8, and, bold(1))$], [Distributive Boolean meet-semilattice of the Section 8 Reviewer Rubric.], [@ch:rubric],
    [$pi(D)$], [Homomorphic diagnostic-set projection onto $bb(B)_8$ (`0xFF` at convergence).], [@eq:rubric-meet],
    [$(V, p, P), phi_(i j), op("div")(i)$], [Finite AFC state distribution, Markov kernel, net flux, and divergence.], [@ch:afc-module],
    [$C_3(P)$], [Base-3 (trit) mutual-information channel capacity of kernel $P$.], [@ch:afc-module],
    [$cal(M), cal(F)_("frozen"), H(f)$], [Treatise manifest, frozen `Main`/`Appendix` file set, and SHA-256 hash.], [@ch:slicing],
    [$cal(N)_h (Delta; cal(M))$], [Bounded $h$-hop blast-radius neighborhood around patch file set $Delta$.], [@eq:hop-slice],
    [$cal(S)_("src"), cal(S)_("leak"), L(cal(R))$], [Admissible source tree, quarantined leakage set, and tracked leak count.], [@ch:hygiene],
    [$Q = (g, ell, {o_k})$], [Stratified Axiomatic Inquiry bundle bound to ladder rung $g$.], [@ch:inquiry],
  ),
  caption: [Complete glossary of symbols and formal constructs in `Main`.],
) <tab:symbol-glossary>

#literature-note[
  Glossary and universal relational closure conventions: @poyau2025af[§4].
]
