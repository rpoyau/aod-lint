# Label hygiene

`aod-lint labels` checks one LaTeX or Typst document driver and its supported
literal includes. Run it separately for Main and each Manual: independent
documents may reuse a label. The command reads sources without rewriting them.

```sh
aod-lint labels main.tex --json
aod-lint labels manual/main.tex --external-labels external-labels.json --json
```

It reports duplicate and empty labels, ambiguous references, unresolved literal
references, missing includes and include cycles. Diagnostics include source
locations; duplicates retain the earlier locations. JSON output binds each read
source to its SHA-256 hash.

## Imported Main labels

`--external-labels` takes a JSON array of actual imported names, with their import
prefixes. Obtain these from the bound Main source or compiled auxiliary file;
adding a name to this list does not prove that Main defines it. The JSON file's
exact bytes are hashed in the report.

The Main-reference wrapper convention must match the selected document:

- AOD wrappers preserve their argument, for example
  `\mainequation{main:eq:example}`. The default adds no prefix.
- Matter I wrappers add `main:` to an argument such as
  `\mainequation{eq:example}`. Select `--main-reference-prefix main:`.

Here `eq:example` is illustrative; supply only names present in the actual Main.
The command does not guess the convention or remove accidental double prefixes.

```sh
aod-lint labels manual2/main.tex \
  --external-labels external-labels.json \
  --main-reference-prefix main: --json
```

## Project-name warnings

Advisory checks flag standalone `M1`, `M2`, `M3` and `R00`–`R99` tokens in literal
labels, headings and active source paths, case-insensitively. `M31` and `manual2`
are not milestone tokens. The established `manual2/data/m1/` application namespace
is exempt from the `M1` path warning.

Use scientific subject names for new material. Existing anchors may be referenced
externally: preserve them or migrate them deliberately with their consumers.
Warnings never rename a file or anchor automatically, and a marker alone does
not establish that content is a review artifact.

To inspect other filenames, pass an explicit JSON array of repository-relative
paths, for example `["manual/data/m3/example.json"]`, with `--paths paths.json`.
This checks names only; it does not read or execute those files. Absolute paths
and `..` components are rejected. The list's bytes are hashed in JSON output.
`--no-name-warnings` disables only these advisory checks.

## Coverage and exit codes

| Exit | Status | Meaning |
| --- | --- | --- |
| 0 | `LITERAL_SCAN_CLEAR` | No issue in the supported literal scan. |
| 0 | `WARNINGS` | Advisory diagnostics only, with no recorded coverage gap. |
| 1 | `ISSUES_FOUND` | At least one definite error; coverage may also be partial. |
| 1 | `INPUT_ERROR` | The driver or supplied context cannot be used. |
| 2 | `PARTIAL` | Unsupported or dynamic behavior requires further checking. |

LaTeX includes resolve from the driver directory; Typst includes resolve from
the including file. Comments and supported verbatim/raw examples are ignored.
Repeated includes are inspected again, so repeated definitions remain visible.
Each invocation is limited to 512 file visits and 64 nested includes.
Typst root-relative paths (`/chapter.typ`) require the compiler's project root
and are reported as partial, rather than read from the operating-system root.
After `\includeonly`, subsequent `\include` commands are skipped with a coverage
gap; selection and saved auxiliary state require the build. `\input` is unaffected.

This is a static source check, not document compilation. Macro definitions and
conditional bodies are masked and reported as coverage gaps. Unevaluated Typst
loops, bindings, function calls and code blocks cannot supply literal anchors.
Custom macro
expansion, dynamic includes, TeX external imports and Typst module execution
require explicit context or the document build. Known AOD reference wrappers
are handled literally; arbitrary custom wrappers are not expanded. Nested
operands are outside the supported literal syntax.

With partial coverage, unresolved LaTeX targets are qualified warnings rather
than definite missing-anchor errors. An unresolved Typst `@name` may instead be
a bibliography citation, so it is a warning with partial coverage. Duplicate
literal definitions remain errors.

Run the normal document build to verify expanded and imported references. A clear
literal scan is neither a scientific review nor certification of AF compliance.
