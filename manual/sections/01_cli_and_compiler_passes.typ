#import "../preamble.typ": *

= CLI Grammar, AST Traversal, and Paired LaTeX Verification <sec:man-cli>

The `aod-lint` binary exposes four deterministic subcommands corresponding to the four verification obligations derived in `Main`. Every subcommand supports human-readable terminal reporting alongside `--json` structured serialization for automated CI and multi-agent pipelines.

== Command-Line Execution Spine <sec:man-commands>

```bash
# 1. Premise-boundary & General Mechanics style verification (single file or directory)
cargo run --release -- check . --strict
cargo run --release -- check external_paper.typ --json

# 2. Treatise manifest generation & frozen Main SHA-256 verification
cargo run --release -- digest . --baseline contracts/main_baseline_manifest.json

# 3. Bounded-hop blast-radius ContextBundle extraction from a git diff
git diff main...HEAD -- '*.typ' '*.tex' > /tmp/patch.diff
cargo run --release -- slice --manifest contracts/main_baseline_manifest.json \
  --diff /tmp/patch.diff --hops 1 --json

# 4. Source-tree provenance & agent-leakage quarantine audit
cargo run --release -- hygiene . --max-blob-bytes 1048576
```

Exit status follows #main-choice("ch:rubric", [Main Choice D4]): `0` when zero `Error` or `Fatal` diagnostics occur (`Section8Rubric == 0b11111111`), and `1` whenever any `Error`, `Fatal`, frozen-tier mutation, include cycle, or `Main -> Manual` firewall violation is detected.

== Lossless Typst AST Pass (`TypstAstPass`) <sec:man-typst-pass>

Unlike line-oriented regular-expression scripts, `TypstAstPass` parses `.typ` source files into a full lossless concrete syntax tree via `typst-syntax` (`SyntaxNode`), tracking exact byte offsets `(line, column)` recursively across child nodes:

+ *Raw Subtree Isolation:* Traversal halts at `SyntaxKind::Raw` nodes so code listings, CLI examples, and quoted lexical tokens never masquerade as mathematical derivation prose.
+ *Internal Anchor vs. External Citation Splitting:* On `SyntaxKind::Ref`, typed internal structural prefixes (`ax:`, `con:`, `ch:`, `cor:`, `eq:`, `sec:`, `app:`, `tab:`, `fig:`, `def:`, `motif:`) satisfy single-anchor precision (#main-choice("ch:graph", [Main Choice D1])) directly, whereas external bibliography keys require localization inside `#literature-note[...]` together with an explicit `[§/Theorem/p.]` supplement.
+ *Mode-Controlled Premise Hoisting:* On `SyntaxKind::Text`, midstream external-law invocations and uncountable continuum imports (`RF3` / #main-con("con:cardinality", [Main Consequence C2])) emit `Warning` with `hoisted: true` in Diagnostic Mode (#main-choice("ch:hoisting", [Main Choice D3])) and `Error` with `hoisted: false` in `--strict` mode (#main-cor("cor:hoisting-rubric", [Main Cor. K2])).

== Paired LaTeX Verification Pass (`LatexPass`) <sec:man-latex-pass>

Because upstream foundational treatises (`AF`, `AFC`, `General Mechanics`, and `AOD`) maintain archival `.tex` sources, `LatexPass` enforces identical epistemic invariants across LaTeX manuscripts:

+ *Comment Stripping and Macro Argument Filtering:* Unescaped `%` comments and `#1` macro parameter placeholders inside `preamble.tex` definitions are stripped prior to reference extraction.
+ *Cross-Document `xr-hyper` Resolution:* Paired `Manual` references using `\mainequation{main:...}`, `\mainsection{main:...}`, `\mainappendix{main:...}`, and `\maintable{main:...}` resolve cleanly across the `main:` prefix to their frozen `Main` label definitions without false dangling-reference alerts.
+ *Brace-Balanced Literature-Note Spans:* Multi-line `\aodliteraturenote{...}` blocks are tracked via brace depth so `\cite[§X]{key}` inside localized literature notes passes cleanly while inline prose `\cite{key}` raises a `Sec7PreciseAnchors` warning.

#literature-note[
  Programmable markup parsing and concrete syntax trees in Typst: @madje2022typst[§3]. Localized literature-note convention: @poyau2025gm[§1].
]
