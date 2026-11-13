#import "../preamble.typ": *

= Repository Hygiene Pass and Agent State Quarantine <sec:man-hygiene>

Autonomous coding harnesses routinely generate workspace instruction files, acceptance ledgers, zip handoff bundles, compressed execution traces, and host-specific scratch metadata during long-horizon campaigns. Checking such operational residue into a scientific Git repository violates #main-choice("ch:hygiene", [Main Choice D7]) and #main-cor("cor:hygiene", [Main Cor. K5]). `RepoHygienePass` (`src/passes/hygiene.rs`) enforces clean source-only repositories via `aod-lint hygiene`.

== Quarantine Classification Matrix <sec:man-hygiene-matrix>

@tab:man-hygiene-classes enumerates the six deterministic inspection rules executed by `RepoHygienePass`. Any `Fatal` or `Error` diagnostic clears bit $b_4$ (`C5_DEMOS_ISOLATED`) of `Section8Rubric` and halts CI with exit code `1`.

#figure(
  table(
    columns: (auto, auto, 1fr, 1fr),
    inset: 5.5pt,
    align: (left, center, left, left),
    table.header(
      [*Leakage Class*], [*Severity*], [*Detected Artifact Pattern*], [*Required Remediation*],
    ),
    [Agent Config & Prompts],
    [`Fatal`],
    [Tracked agent hidden directories or prompt/boot instruction files committed to scientific source.],
    [Remove from scientific repo; store portable skills in dedicated tooling repositories.],
    [Private Session Tokens],
    [`Fatal`],
    [Personal cloud library URLs, session file identifiers, or account-scoped query strings in text files.],
    [Purge leaked identifiers and rewrite unmerged branch history prior to publication.],
    [Sandbox Scratch Paths],
    [`Error`],
    [Ephemeral container workspace paths or developer home-directory paths embedded in bindings.],
    [Normalize all provenance and manifest paths to repository-relative coordinates (`"."`).],
    [Archives & Root PDFs],
    [`Error`],
    [Checked-in `.zip`/`.tar.gz`/`.json.gz` handoffs, LaTeX `.aux`/`.fls` caches, or non-figure `.pdf` files.],
    [Generate PDFs and release bundles in detached `dist/` CI steps; cite papers via `refs.bib`.],
    [Agent Handoff Ledgers],
    [`Error`],
    [Tracked agent review narratives, acceptance CSVs, handoff manifests, or vendored producer copies.],
    [Verify invariants via executable `--check` scripts rather than static handoff receipts.],
    [Oversized State Blobs],
    [`Warning`],
    [Tracked non-figure files exceeding `--max-blob-bytes` (default $1$ MiB) or internal tracker codes.],
    [Regenerate bulk simulation outputs deterministically from compact input fixtures.],
  ),
  caption: [Quarantine classes enforced by `aod-lint hygiene` under #main-cor("cor:hygiene", [Main Cor. K5]).],
) <tab:man-hygiene-classes>

== Figure Asset Exemption and Detached Release Packaging <sec:man-hygiene-figures>

Vector graphics under `figures/` or `*/figures/` (`.pdf`, `.svg`, `.png`) are legitimate manuscript inputs belonging to $cal(S)_("src")$ and are explicitly exempted from both the root-PDF prohibition and the `--max-blob-bytes` state-dump ceiling. Compiled publication artifacts (`main.pdf`, `manual.pdf`, `source.zip`, `bundle.zip`, `tests.txt`, `SHA256.txt`) are written exclusively to the ignored `dist/` directory by `scripts/build_release_bundle.py` so working checkouts remain 100% source-pure.

#literature-note[
  Scope discipline (`AF6`) and release versioning (`AF §10`): @poyau2025af[§3]; @poyau2025af[§10].
]
