# AFC finite checks and their limits

Bind the actual AFC source and declared model before using a result. The finite
model uses a family of states `V`, a normalized nonnegative memory `p` and a
row-stochastic kernel `P`. Define

```text
phi(i,j) = p(i) P(i,j) - p(j) P(j,i)
div(i) = sum_j phi(i,j)
```

For a region `R`, antisymmetry cancels internal pairs and gives

```text
sum_(i in R) div(i) = sum_(i in R, j outside R) phi(i,j).
```

The boundary consists of ordered crossing pairs. Keep their orientation and
original records even when signed contributions cancel. This identity does not
supply a metric, filled cell complex, physical clock or empirical law.

The library's `AFCStokesChecker` currently checks supplied floating-point flux
and divergence with a caller-provided tolerance. It does not authenticate model
inputs, derive the flux from `p,P`, perform exact rational arithmetic, or certify
the entire AFC source. The `check` manuscript command does not run this checker.
Use the target repository's exact fixtures for exact claims.

Aggregation preserves the stated divergence identity; a Markov quotient needs
its own construction and applicable lumpability conditions. Flux isotropy depends
on memory as well as the kernel: kernel symmetry alone is insufficient. A joint
invariance hypothesis is a sufficient condition, not a universal necessity for
accidental zero-flux cases.

Path-information bounds and channel capacity remain conditional on their stated
branching, input and repeated-use assumptions. They are not universal physical
capacity or runtime-complexity laws. No channel-capacity checker is implemented
by this repository's manuscript CLI.

The bound calculus and source-issue guidance are maintained in the
[afc skill](https://github.com/rpoyau/aod-skills/tree/main/skills/afc).
