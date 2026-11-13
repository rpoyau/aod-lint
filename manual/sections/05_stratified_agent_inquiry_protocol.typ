#import "../preamble.typ": *

= Multi-Agent Verification Loop and Stratified Inquiry Bundles <sec:man-agents>

When multi-agent teams collaborate on an AF-governed repository, unconstrained conversational handoffs introduce premise drift. This section instantiates the convergence loop of #main-cor("cor:hoisting-rubric", [Main Cor. K2]) and the five-rung Stratified Axiomatic Inquiry Gate of #main-choice("ch:inquiry", [Main Choice D8]).

== Five-Role Deterministic Convergence Machine <sec:man-roles>

Agent execution is partitioned across five bounded roles communicating strictly through immutable JSON bundles until `Section8Rubric == 0b11111111` (`0xFF`) and all compiler/test gates pass:

+ *Lead (Controller):* Owns state transitions, invokes `aod-lint digest` and `aod-lint slice`, and halts or approves publication when `rubric_bits == "11111111"` and `L(R) == 0`.
+ *Reviewer:* Runs `aod-lint check --strict --json` and `aod-lint hygiene --json` over the `ContextBundle`, emitting either a `FeedbackBundle` of localized `RuleAnchor` diagnostics or a typed `AxiomaticInquiryBundle`.
+ *Planner:* Orders `FeedbackBundle` items into non-conflicting file edits respecting `Main`/`Manual` tier separation.
+ *Writer:* Applies minimal `Manual` or code patches, hoists explicit `Choice` blocks, and localizes external citations into `#literature-note[...]`.
+ *Tester:* Executes `cargo test`, `typst compile main.typ`, `typst compile manual/main.typ`, and `aod-lint` self-checks, returning automatic compiler feedback on any non-zero exit code.

== Stratified Axiomatic Inquiry Bundle Schema <sec:man-inquiry-schema>

Whenever an agent encounters genuine scientific ambiguity that cannot be deduced from declared boundary premises, open-ended stylistic questions are prohibited. The escalation must serialize an `AxiomaticInquiryBundle` bound to the lowest unresolved gate in #main-eq("eq:inquiry-ladder", [Main Eq. (12)]):

```json
{
  "gate": "RELATION",
  "blocked_locus": "manual/sections/02_dec_stokes_and_capacity_fixtures.typ:14",
  "upstream_distinctions": ["v_1", "v_2", "v_3"],
  "question": "Select the coupling relation on edge (v_1, v_2) within scope B:",
  "options": [
    "Directed one-way transition (P_12 > 0, P_21 = 0)",
    "Antisymmetric reversible pair with net flux phi_12 = -phi_21",
    "Blocked inadmissible channel (adm(e_12; B) = 0 => P_12 = 0)"
  ]
}
```

@tab:man-inquiry-gates summarizes the five admissible inquiry gates in strict precedence order.

#figure(
  table(
    columns: (auto, auto, 1fr),
    inset: 6pt,
    align: (center, left, left),
    table.header(
      [*Order*], [*Gate Identifier*], [*Admissible Escalation Question*],
    ),
    [1], [`DISTINCTION`], [Are candidate states distinct cuts against $cal(N)$, or identical under reduction?],
    [2], [`RELATION`], [Is coupling between declared distinctions directed, reversible, or blocked?],
    [3], [`MOTIF`], [Which closed return or boundary motif classifies the admitted trace?],
    [4], [`MODELING`], [Which explicit finite `Choice D` kernel or capacity bound parameterizes the motif?],
    [5], [`SCOPE`], [Does the addition belong to a major-version `Main` amendment or an active `Manual` fixture?],
  ),
  caption: [Precedence-ordered gates for `AxiomaticInquiryBundle` escalations (#main-choice("ch:inquiry", [Main Choice D8])).],
) <tab:man-inquiry-gates>

#literature-note[
  Relational precedence (`RF1`: distinction precedes relation) and scope fixing (`AF1`, `AF6`): @poyau2025af[§3]; @poyau2025af[§4].
]
