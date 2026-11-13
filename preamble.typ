// preamble.typ — Canonical Typst styling and AF / General Mechanics structural macros

#let aod-lint-version = "v0.2.0"
#let aod-lint-date = "November 13, 2026"
#let aod-lint-author = "Reginald Poyau"

#let apply-publication-style(body) = {
  set page(
    paper: "us-letter",
    margin: (x: 1in, y: 1in),
    numbering: "1",
  )
  set text(
    font: ("Libertinus Serif", "New Computer Modern"),
    size: 11pt,
    lang: "en",
  )
  set par(
    justify: true,
    leading: 0.62em,
    spacing: 0.95em,
  )
  set heading(numbering: "1.1")
  set math.equation(numbering: "(1)")

  show heading.where(level: 1): it => block(above: 1.5em, below: 0.75em)[
    #text(size: 13.5pt, weight: "bold")[#it]
  ]
  show heading.where(level: 2): it => block(above: 1.2em, below: 0.55em)[
    #text(size: 11.5pt, weight: "bold")[#it]
  ]
  show heading.where(level: 3): it => block(above: 0.95em, below: 0.45em)[
    #text(size: 11pt, weight: "bold", style: "italic")[#it]
  ]
  show raw.where(block: true): it => block(
    width: 100%,
    inset: 8pt,
    radius: 2pt,
    fill: luma(247),
    stroke: 0.5pt + luma(210),
    text(size: 9.2pt, it),
  )

  // Allow AF / GM structural figures to break naturally across pages and left-align
  show figure.where(kind: "af-axiom"): set block(breakable: true, width: 100%)
  show figure.where(kind: "af-consequence"): set block(breakable: true, width: 100%)
  show figure.where(kind: "af-choice"): set block(breakable: true, width: 100%)
  show figure.where(kind: "af-corollary"): set block(breakable: true, width: 100%)
  show figure.where(kind: "af-axiom"): it => align(left, it.body)
  show figure.where(kind: "af-consequence"): it => align(left, it.body)
  show figure.where(kind: "af-choice"): it => align(left, it.body)
  show figure.where(kind: "af-corollary"): it => align(left, it.body)

  body
}

#let apply-appendix-numbering(body) = {
  counter(heading).update(0)
  set heading(numbering: "A.1")
  body
}

// AF §9 Minimal Author Templates (wrapped as referenceable figures for exact @ref resolution)
#let axiom-block(id, name, scope: [Finite manuscript graph], body) = figure(
  kind: "af-axiom",
  supplement: [Axiom],
  numbering: n => "A" + str(n),
  outlined: false,
  block(
    width: 100%,
    inset: (left: 10pt, right: 8pt, y: 7pt),
    stroke: (left: 2pt + black),
    fill: luma(250),
    above: 0.9em,
    below: 0.9em,
  )[
    *Axiom #id (#name).* #emph[#body] #h(1fr) [#emph[Scope:] #scope]
  ],
)

#let consequence-block(id, name, proof: [], body) = figure(
  kind: "af-consequence",
  supplement: [Consequence],
  numbering: n => "C" + str(n),
  outlined: false,
  block(
    width: 100%,
    inset: (left: 10pt, right: 8pt, y: 6pt),
    stroke: (left: 1.2pt + luma(90)),
    above: 0.85em,
    below: 0.85em,
  )[
    *Consequence #id (#name).* #emph[#body] \
    #emph[Proof sketch.] #proof
  ],
)

#let choice-block(id, name, scope: [Manuscript verification], rationale: [clarity only], body) = figure(
  kind: "af-choice",
  supplement: [Choice],
  numbering: n => "D" + str(n),
  outlined: false,
  block(
    width: 100%,
    inset: (left: 10pt, right: 8pt, y: 6pt),
    stroke: (left: 1pt + luma(140)),
    above: 0.85em,
    below: 0.85em,
  )[
    [#emph[Scope:] #scope] *Choice #id (#name).* #emph[We define] #body #h(1fr) [#emph[Rationale:] #rationale]
  ],
)

// General Mechanics Corollary Block (7-part strict hierarchy)
#let corollary-block(
  id,
  title,
  setup: [],
  derivation: [],
  specialization: [],
  calibration: [],
  falsification: [],
  provenance: [],
) = figure(
  kind: "af-corollary",
  supplement: [Corollary],
  numbering: n => "K" + str(n),
  outlined: false,
  block(above: 1.05em, below: 1.05em, width: 100%, breakable: true)[
    *Corollary #id (#title).*

    *Setup.* #setup

    *Derivation.* #derivation

    *Specialization.* #specialization

    *Calibration.* #calibration

    *Falsification.* #falsification

    #emph[Provenance.] #provenance
  ],
)

// General Mechanics localized external citation block
#let literature-note(body) = block(above: 0.65em, below: 0.95em)[
  #emph[Literature note.] #body
]
