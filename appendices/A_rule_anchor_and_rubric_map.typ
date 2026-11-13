#import "../preamble.typ": *

= Rule Anchor and Section 8 Rubric Projection Table <app:rubric-map>

@tab:rubric-projection records the exact deterministic mapping $mu : "RuleAnchor" -> bb(B)_8$ defined in @ch:rubric (@eq:rubric-meet). Every error-level diagnostic clears a unique bit of the 8-bit convergence mask $r in {0, 1}^8$; full convergence requires $r = mono("0b11111111") = mono("0xFF")$.

#figure(
  table(
    columns: (auto, auto, auto, 1fr),
    inset: 6pt,
    align: (left, center, center, left),
    table.header(
      [*Protocol Clause*], [*Bit*], [*Mask Clear*], [*Verification Obligation*],
    ),
    [`AF1`], [$b_0$ (`C1`)], [`~0x01`], [State axioms once, symbols defined, scope fixed; no defense essays.],
    [`AF2`], [$b_1$ (`C2`)], [`~0x02`], [Derive immediate consequences via pure mathematics only.],
    [`AF3` / `§7.4`], [$b_2$ (`C3`)], [`~0x04`], [Declare all modelling choices and parameters explicitly at first use.],
    [`AF4`], [$b_3$ (`C4`)], [`~0x08`], [Prohibit midstream imports of external laws or empirical postulates.],
    [`AF6` / Hygiene], [$b_4$ (`C5`)], [`~0x10`], [Isolate demonstrations (`Manual`) and quarantine non-source agent state.],
    [`AF7`], [$b_5$ (`C6`)], [`~0x20`], [Bind concrete falsification thresholds to specific derived statements.],
    [`§7.1`--`§7.3`], [$b_6$ (`C7`)], [`~0x40`], [Enforce unique labels, single precise anchors, and consistent notation.],
    [`RF1`--`RF5`], [$b_7$ (`C8`)], [`~0x80`], [Respect relational closure, no absolutes, and bookkeeping-only information.],
  ),
  caption: [Deterministic mapping from AF protocol clauses to the 8-bit Section 8 Reviewer Rubric word $bb(B)_8$.],
) <tab:rubric-projection>

*Severity semantics.* Diagnostics carrying severity `Info` or `Warning` preserve $r = mono("0xFF")$ while surfacing hoisted dependencies (@ch:hoisting) or localized citation-formatting advisories; diagnostics carrying severity `Error` or `Fatal` clear their mapped bit via @eq:rubric-meet and force a non-zero verifier exit code.

#literature-note[
  Eight binary checks of the AF Reviewer Rubric: @poyau2025af[§8].
]
