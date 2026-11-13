// manual/main.typ — Companion Manual & Reference Engine Specification (Active Tier)
#import "preamble.typ": *
#show: apply-publication-style

#page(numbering: none)[
  #align(center)[
    #set par(justify: false)
    #v(3.2em)
    #text(size: 19pt, weight: "bold", hyphenate: false)[
      AOD-Lint: Executable Verification for the \ Axiomatic Fundamentalism Protocol
    ]

    #v(0.65em)
    #text(size: 13.5pt, hyphenate: false)[
      Premise-Boundary Closure, Two-Pass Hoisting, and Tiered Treatise Hygiene
    ]

    #v(0.65em)
    #text(size: 11.5pt, style: "italic")[
      Rust Compiler Architecture, D.E.C. Audit Ledgers, and Multi-Agent Workflows
    ]

    #v(1.2em)
    #text(size: 15pt, weight: "bold")[Manual]

    #v(0.9em)
    #text(size: 12pt)[#aod-lint-author]

    #v(0.3em)
    #text(size: 10pt, fill: luma(80))[Version #aod-lint-version · #aod-lint-date]

    #v(1.6em)
    #math.equation(block: true, numbering: none)[
      $ partial_("in") cal(G) tack.r cal(V)_(circle.small) , quad pi(D) = mono("0b11111111") , quad L(cal(R)) = 0 $
    ]

    #v(2.2em)
    #text(size: 10pt, style: "italic")[Epitaph]
    #v(0.35em)
    #math.equation(block: true, numbering: none)[
      $ cal(N) equiv infinity , quad cal(N) -> cal(N) -> cal(N) equiv cal(N) $
    ]
    #v(0.35em)
    #text(size: 9.8pt, style: "italic")[
      AFC runs raw. The trace records the run. AOD-Lint audits the boundary.
    ]
  ]
]

#outline(indent: auto, depth: 2)
#pagebreak()

#include "sections/00_scope_and_architecture.typ"
#include "sections/01_cli_and_compiler_passes.typ"
#include "sections/02_dec_stokes_and_capacity_fixtures.typ"
#include "sections/03_digest_frozen_guard_and_slicing.typ"
#include "sections/04_hygiene_and_leakage_quarantine.typ"
#include "sections/05_stratified_agent_inquiry_protocol.typ"

#pagebreak()
#show: apply-appendix-numbering

#include "appendices/A_worked_audit_ledger_fixtures.typ"
#include "appendices/B_diagnostic_remediation_catalog.typ"

#pagebreak()
#bibliography("refs.bib", title: [References], style: "ieee")
