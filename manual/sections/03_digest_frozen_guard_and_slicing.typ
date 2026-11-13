#import "../preamble.typ": *

= Treatise Manifest, Frozen-Main SHA-256 Guard, and Blast-Radius Slicing <sec:man-digest-slice>

Reviewing revisions to large axiomatic treatises—exceeding 130 pages of coupled definitions, appendices, and manual fixtures—presents two dual failure modes: feeding an unindexed diff to a reviewer misses global label or motif breaks, whereas loading the entire treatise into a bounded context window dilutes premise verification. `AodDigestPass` (`src/passes/digest.rs`) and `AodSlicePass` (`src/passes/slice.rs`) operationalize #main-choice("ch:slicing", [Main Choice D6]) and #main-cor("cor:firewall-slice", [Main Cor. K4]).

== Step 1: Generating and Locking the Frozen Baseline Manifest <sec:man-baseline>

Running `aod-lint digest . -o contracts/main_baseline_manifest.json` traverses every `.typ` and `.tex` source file, computes its SHA-256 digest, classifies its document tier (`Main`, `Appendix`, `Manual`, `Other`), extracts `#include`/`\input` directives, headings, `<label>`/`\label` definitions, `@ref`/`\eqref`/`#main-*` cross-references, declared `Axiom`/`Choice`/`Def` headers, and canonical `AOD` motifs (`Monon`, `Dion`, `Duon`, `Duad`, `Dimonon`, `Ditrion`, `Tritrion`, `Tetratrion`, `RingGate`, `SADAR`, `ADAR`, `Awareness`, `DEC`, `Stokes`, `CurlingCurl`, `Hinge`).

During subsequent pull-request audits, passing `--baseline contracts/main_baseline_manifest.json` executes three structural gates simultaneously:

+ *Cryptographic Immutability:* Any modified, deleted, or newly injected file carrying `frozen: true` (`Main` or `Appendix`) fails verification immediately unless an explicit major-revision baseline update is authorized (`AF5` and `AF §10`).
+ *One-Way Tier Firewall:* Any edge originating in `Main` or `Appendix` that targets a label defined in `Manual` emits a `FirewallViolation` and exits with status `1`.
+ *Include Acyclicity:* Depth-first back-edge detection verifies zero `#include`/`\input` cycles across the treatise container graph.

== Step 2: Computing Bounded-Hop Blast-Radius Slices <sec:man-slicing>

Given a unified patch `patch.diff` touching `Manual` chapters $Delta subset.eq cal(F)_("Manual")$, `aod-lint slice` parses the diff headers `+++ b/...` and executes breadth-first reachability up to hop radius $h$ across `Include` and `Reference` edges (#main-eq("eq:hop-slice", [Main Eq. (9)])):

```bash
aod-lint slice \
  --manifest contracts/main_baseline_manifest.json \
  --diff patch.diff \
  --hops 1 \
  --json > context_bundle.json
```

At $h = 1$, the emitted `ContextBundle` (#main-app("app:manifest-grammar", [Main App. C])) contains:
- `touched_files`: exact `Manual` files modified by the patch;
- `impacted_files`: all touched files plus every `Main` or `Manual` file within 1 dependency hop (capturing 100% of `Main` corollaries and equations directly invoked by the modified chapters while excluding unrelated `Manual` benchmarks);
- `frozen_main_violations`: immediate alert list if the patch touches any `frozen: true` file; and
- `excerpts`: bounded line-range summaries with topological inclusion reasons (`touched directly by diff` vs. `within 1 hop(s) of touched graph`).

#literature-note[
  SHA-256 cryptographic hash specification: @nist2015sha[§6.2]. Paired `Main` and `Manual` build verification in AOD: @poyau2026aod[Manual §1].
]
