#import "../preamble.typ": *

= Constructive Diagnostic Remediation Catalog <app:man-remediation>

Under the academic social contract of AF §7, diagnostics must provide constructive, copy-pasteable Typst or LaTeX remediations rather than adversarial referee commentary. @tab:man-remediation lists canonical before-and-after transformations for the primary `RuleAnchor` diagnostics emitted by `TypstAstPass`, `LatexPass`, and `RepoHygienePass`.

#figure(
  table(
    columns: (auto, 1fr, 1fr),
    inset: 5.5pt,
    align: (left, left, left),
    table.header(
      [*Rule Anchor*], [*Non-Compliant Pattern*], [*Compliant Remediation*],
    ),
    [`§7.2` Range],
    [Prose pointer using broad range `see §§2–5` or `see the literature`.],
    [Point to a single exact anchor such as `@cor:stokes` or `@eq:stokes-balance`.],
    [`§7.2` Citation],
    [Inline prose citation `@poyau2025af` without supplement or outside literature note.],
    [Localize at block end: `#literature-note[Protocol: @poyau2025af[§3].]`.],
    [`AF4` Import],
    [Midstream step justified `by Gauss's law` inside a corollary proof.],
    [Declare finite boundary premise in `Choice Dk` and derive via `@eq:stokes-balance`.],
    [`RF3` Continuum],
    [Unaccounted `smooth manifold` or `Hilbert space` assumed mid-derivation.],
    [Replace with finite distinction set $V$ (`Choice Dk`) or hoist explicit liability receipt.],
    [`Main/Manual`],
    [`Main` file referencing `@sec:man-stokes` in `manual/`.],
    [Keep `Main` self-contained; reference `Main` from `Manual` via `#main-cor(...)`.],
    [`AF6` Hygiene],
    [Tracked `main.pdf`, `.zip` bundle, or personal cloud URI in repository.],
    [Add build outputs to `.gitignore`, emit into `dist/`, and strip session URIs.],
  ),
  caption: [Constructive diagnostic-to-remediation reference catalog.],
) <tab:man-remediation>

#literature-note[
  Mechanical labelling and single-anchor reference discipline: @poyau2025af[§7].
]
