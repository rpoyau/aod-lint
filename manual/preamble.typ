// manual/preamble.typ — Paired Manual macros and cross-tier Main anchor references
#import "../preamble.typ": (
  aod-lint-author,
  aod-lint-date,
  aod-lint-version,
  apply-appendix-numbering,
  apply-publication-style,
  literature-note,
)

// Cross-tier Main anchor references (indexed by AodDigestPass into Manual -> Main edges)
#let main-sec(id, display) = box[#text(weight: "semibold")[#display]]
#let main-ax(id, display) = box[#text(weight: "semibold")[#display]]
#let main-con(id, display) = box[#text(weight: "semibold")[#display]]
#let main-choice(id, display) = box[#text(weight: "semibold")[#display]]
#let main-cor(id, display) = box[#text(weight: "semibold")[#display]]
#let main-eq(id, display) = box[#text(weight: "semibold")[#display]]
#let main-app(id, display) = box[#text(weight: "semibold")[#display]]
#let main-tab(id, display) = box[#text(weight: "semibold")[#display]]
