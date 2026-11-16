# Implemented linter interfaces

`aod-lint` reads LaTeX and Typst source. Its diagnostics support review; they do
not reconstruct every proof, compile a document, hoist premises, or certify AF.

## Source checks

```sh
aod-lint check manuscript.tex --json
aod-lint labels main.tex --json
```

`check` uses lexical patterns for citation locators, broad reference ranges and
possible premise dependencies. `--strict` makes premise-review candidates errors
for a selected workflow. Such an error is still a lexical finding: inspect the
actual declaration and scope before concluding that a premise is undeclared.
The command never modifies a source or resolves an imported scientific law.

`check --json` schema **2.0** replaces the former `converged` and `rubric_bits`
fields. It reports `coverage: static_source`, `semantic_review: not_checked`, and
all eight AF rubric questions as `not_checked`. A diagnostic does not itself
settle a semantic rubric question. `candidate_dependencies` and each diagnostic's
`dependency_candidate` flag replace the misleading hoisting fields; actual
hoisting is `not_performed`. Consumers of the old fields must migrate.

The [labels command](LABEL_HYGIENE.md) has a separate **1.0** schema, scoped to
one document driver. It checks supported literal definitions/references and
warns on project naming. Unsupported expansion stays explicit partial coverage.

## Source indexing and preservation

```sh
aod-lint digest . --output source-manifest.json
aod-lint digest . --freeze-main --output preservation-baseline.json
aod-lint digest . --baseline preservation-baseline.json --json
aod-lint slice --manifest source-manifest.json --files sections/topic.tex --json
```

`digest` indexes source files, hashes, literal relationships and motif-name
matches. It currently inventories the source directory recursively, including
inactive files; it is not the driver's compiled dependency closure. Main,
Appendix, `manual/` and `manual2/` ownership are classified separately.
The Main-to-Manual reference firewall remains active regardless of freezing.
When a name has multiple indexed definitions, the index links all candidates
and records unresolved scope in `coverage_gaps`. It does not let a historical
copy overwrite the Main definition. A Main-to-Manual finding requires all known
literal candidates to belong to Manual; the actual driver/build must resolve
ambiguous names. Old manifests without `coverage_gaps` remain readable.

Main and Appendix are **not frozen by default**. `--freeze-main` records an
explicit preservation policy in each relevant file's `frozen` field. Passing
`--baseline` enforces the hashes marked frozen in that supplied manifest,
including existing older manifests; it does not require `--freeze-main` again.
This is an owner-selected preservation check, not a general ban on Main changes.
JSON includes `frozen_baseline_violations`, so a failed baseline check retains
its reasons as well as a nonzero exit. Older manifests without that field remain
readable.

`slice` follows indexed relationships to prepare review context. It reports
modifications to files explicitly frozen in its input manifest. Its current
diff parser is hunk-based: use `--files` with a complete changed-path list for
rename-only changes. JSON/Python/archive contents are not semantic source slices.
These coverage limits must accompany downstream review claims.

A failed check exits 1. `labels` and `digest` use exit 2 for reported partial coverage
without definite errors. A zero exit never substitutes for source-bound review,
mathematical proof, a publication build or physical validation.
