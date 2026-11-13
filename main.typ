// main.typ — Main publication container (Frozen Ontological & Protocol Extension Tier)
#import "preamble.typ": *
#show: apply-publication-style

#include "sections/00_title.typ"
#include "sections/01_abstract.typ"

#outline(indent: auto, depth: 2)
#pagebreak()

#include "sections/02_axioms_and_relational_closure.typ"
#include "sections/03_formal_verification_calculus.typ"
#include "sections/04_two_pass_hoisting_and_rubric_algebra.typ"
#include "sections/05_dec_stokes_and_blast_radius_slicing.typ"
#include "sections/06_source_hygiene_and_provenance_isolation.typ"

#pagebreak()
#show: apply-appendix-numbering

#include "appendices/A_rule_anchor_and_rubric_map.typ"
#include "appendices/B_compaction_and_firewall_invariants.typ"
#include "appendices/C_manifest_and_context_bundle_grammar.typ"
#include "appendices/D_glossary_and_symbol_table.typ"

#pagebreak()
#bibliography("refs.bib", title: [References], style: "ieee")
