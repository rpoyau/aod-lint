# aod-lint

Static source diagnostics for AOD LaTeX and Typst manuscripts. The tool checks
literal references, indexes source dependencies and prepares bounded review
context. It does not compile documents or certify scientific validity.

```sh
cargo test
cargo run -- labels main.tex --json
cargo run -- check sections/example.tex --json
cargo run -- digest . --output source-manifest.json
cargo run -- slice --manifest source-manifest.json --files sections/example.tex --json
```

See [label hygiene](docs/LABEL_HYGIENE.md) for document scopes, imported labels,
naming warnings and partial coverage; [interfaces](docs/03_COMPILER_LINTER_DESIGN.md)
for JSON schemas, preservation options and migration from older outputs; and
[source authority](docs/01_ONTOLOGY_AND_AF_PROTOCOL.md) for AF/AFC/AOD scope.
Main freezing is an explicit preservation option, not the default ownership rule.
