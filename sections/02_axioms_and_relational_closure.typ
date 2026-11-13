#import "../preamble.typ": *

= Purpose, Scope, and Epistemic Boundary Axioms <sec:axioms>

The Axiomatic Fundamentalism (AF) protocol establishes seven authorial commitments (`AF1`--`AF7`), five universal relational foundations (`RF1`--`RF5`), mechanical referencing rules, and an eight-check binary Reviewer Rubric. In its foundational formulation, enforcement relies on manual inspection: a human reviewer reads prose sequentially to judge whether midstream postulates were smuggled into a proof, whether citations resolve to exact anchors, and whether demonstrations remain isolated from foundational axioms. At the scale of multi-volume treatises and automated research workflows, manual inspection is neither deterministic nor context-bounded.

*Scope.* This extension formalizes structural traceability, premise accounting, tiered document isolation, blast-radius slicing, and repository provenance hygiene as decidable predicates over finite source representations. Empirical adequacy of physical measurements external to declared observable maps lies outside scope (`AF6`); verification decides whether interior claims follow without exterior premise leakage from the declared boundary.

#axiom-block("A1", "Epistemic Boundary Closure", scope: [Finite scientific manuscripts])[
  Within a declared model universe, every admissible interior statement belongs to the finite transitive mathematical closure of explicitly declared Distinctions, Axioms, and Modelling Choices; no undeclared exterior law, background container, or unanchored reference is admissible as a premise.
] <ax:boundary>

#axiom-block("A2", "Deterministic Clause Accountability", scope: [Verification passes])[
  Every normative diagnostic emitted against a manuscript is a total, deterministic function of its finite source text and maps to an explicit protocol clause anchor.
] <ax:accountability>

Two structural consequences follow directly from @ax:boundary and @ax:accountability without auxiliary modelling assumptions.

#consequence-block(
  "C1",
  "Acyclic Premise Dominance",
  proof: [From @ax:boundary by finite directed-walk induction. Suppose a finite set of interior claims $S$ forms a directed justification cycle $v_1 -> v_2 -> dots.c -> v_k -> v_1$ with no incoming edge from declared boundary premises. Collapsing the strongly connected component $S$ yields an interior supernode with in-degree zero outside the declared boundary, contradicting @ax:boundary. Any admissible derivation graph is therefore a finite Directed Acyclic Graph (DAG) whose source vertices lie exclusively in the declared premise boundary.],
)[
  Every sound finite derivation reduces to an acyclic directed graph in which every maximal backward dependency chain terminates at an explicitly declared Axiom or Modelling Choice.
] <con:dag>

#consequence-block(
  "C2",
  "Cardinality Accountability of Imported Spaces",
  proof: [From @ax:boundary and relational closure (`RF1`--`RF3`) by set-cardinality comparison. Let $cal(D)$ denote the finite set of declared distinctions in a manuscript ($|cal(D)| < infinity$). Every relation on $cal(D)$ of finite arity $m$ has cardinality at most $2^(|cal(D)|^m) < infinity$. Any symbol domain $Omega$ satisfying $|Omega| >= aleph_0$ introduces elements indistinguishable under $cal(D)$, constituting an exterior ontological commitment that must be explicitly declared and tracked rather than introduced silently.],
)[
  No infinite or continuum state space arises from finite declared distinctions alone; introducing a domain $Omega$ with $|Omega| > |cal(D)|$ carries a non-zero exterior ontological liability that must be recorded explicitly at the boundary.
] <con:cardinality>

$ |cal(D)| < infinity quad ==> quad forall m in bb(N), space |cal(P)(cal(D)^m)| = 2^(|cal(D)|^m) < aleph_0 . $ <eq:finite-closure>

@eq:finite-closure fixes the algebraic distinction between a finite Modelling Choice (`AF3`, parameterizing relations over already-declared distinctions $cal(D)$) and an uncountable continuum import (`AF4`/`RF2`, importing $2^(aleph_0)$ undistinguished points alongside ultraviolet limit obligations).

#literature-note[
  Axiomatic commitments `AF1`--`AF7`, relational foundations `RF1`--`RF5`, and binary reviewer rubric: @poyau2025af[§3]; @poyau2025af[§4]; @poyau2025af[§8]. Topological ordering and strongly connected component contraction on finite directed graphs: @cormen2022[§20.4, §20.5].
]
