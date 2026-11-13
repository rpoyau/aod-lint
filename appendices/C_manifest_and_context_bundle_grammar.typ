#import "../preamble.typ": *

= Manifest and Context-Bundle Algebraic Grammar <app:manifest-grammar>

The treatise indexer and bounded-hop slicer defined in @ch:slicing serialize deterministic JSON records consumed by automated CI pipelines and bounded-context reviewers. Their canonical record schemas are fixed by @eq:manifest-tuple and @eq:context-bundle-tuple:

$ cal(M) = chevron.l "version", "root", cal(F), cal(E)_cal(F), cal(W)_("firewall"), cal(C)_("include") chevron.r, $ <eq:manifest-tuple>

where each file record $f in cal(F)$ is the tuple

$ f = chevron.l "path", tau_cal(F), "frozen", "sha256", "lines", "includes", "headings", "labels", "refs", "distinctions", "motifs" chevron.r, $ <eq:file-record>

and each $h$-hop blast-radius slice (@eq:hop-slice) produces the self-contained *Context Bundle*

$ cal(K)_h (Delta; cal(M)) = chevron.l h, Delta, cal(N)_h (Delta; cal(M)), cal(W)_("frozen"), cal(X)_("excerpts") chevron.r . $ <eq:context-bundle-tuple>

In @eq:manifest-tuple, `"root"` is normalized to the repository-relative token `"."` so cryptographic baseline manifests remain bit-for-bit reproducible across distinct build hosts (@cor:hygiene). In @eq:context-bundle-tuple, $cal(W)_("frozen")$ lists any touched paths $f in Delta$ carrying `"frozen": true`, halting unapproved mutations to `Main` or `Appendix` tiers before downstream review executes.

#literature-note[
  Breadth-first neighborhood computation on finite graphs: @cormen2022[§20.2].
]
