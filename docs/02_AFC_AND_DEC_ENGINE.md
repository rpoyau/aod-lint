# 02. Axiomatic Fundamentalism Calculus (AFC) & D.E.C. Engine

Reference: Reginald Poyau (Dec 2025), *"N ≡ ∞: Axiomatic Fundamentalism Calculus (AFC) - The Hidden Form of Stokes"*.

## 1. The Monadic Axiom: Null = Infinity
- **Axiom 1:** `N ≡ ∞ ≡ χ`.
- AFC is AF with zero physical postulates: a Null state `N` carries zero distinctions and zero relations. Without distinctions, there are no boundaries; an undifferentiated state is boundless infinity (`∞`), supporting recursive Null splittings `N = (N, N, ...)`.

## 2. Choices 1–5 & Declared Edge Computation (D.E.C.)
1. **Choice 1 (Memory State):** Finite family of Distinctions `V`; probability assignment `p: V -> [0, 1]`, `sum_{i in V} p(i) = 1`. (Observer/Modeller is simply a reference label for `(p, P)`—no separate dualist object).
2. **Choice 2 (Markov Kernel):** Transition matrix `P = (P_ij)` on `V`, `P_ij >= 0`, `sum_j P_ij = 1`. Update: `p_{t+1}(j) = sum_i p_t(i) P_ij`.
3. **Choice 3 (Antisymmetric Net Flux):**
   `phi_ij := p(i) P_ij - p(j) P_ji`, satisfying strict antisymmetry `phi_ij = -phi_ji`.
4. **Choice 4 (Divergence):**
   `div(i) := sum_{j in V} phi_ij`.
5. **Choice 5 (Regions & Boundary Cut):**
   Subset `R ⊆ V` induces inside/outside cut against complement `V \ R` with directed boundary `dR := {(i, j) : i in R, j not in R}`.

## 3. Corollary 1: The AFC Discrete Stokes Identity
For any Region `R ⊆ V`:
`sum_{i in R} div(i) = sum_{(i, j) in dR} phi_ij`

- **Internal Annihilation to Null:** Because `phi_ij = -phi_ji`, all interior exchanges cancel identically:
  `S_int(R) := sum_{i in R} sum_{j in R} phi_ij = sum_{{i,j} ⊂ R} (phi_ij + phi_ji) = 0`.
- Discrete Exterior Calculus (chains `C`, boundary `dC`, cochain `omega`, coboundary `d omega`) is isomorphic to pure relational bookkeeping across a cut.

## 4. Path Complexity, Clustering, Isotropy & Trit Capacity Bounds
- **Base-3 (Trits):** Every distinction induces 3 relational states (Inside, Outside, Boundary/Cut).
- **Consequence 3 (Scale-Invariant Path Growth):** Under `m_t <= c * t`, distinguishable Memory paths grow as `log_3(N_n) = O(n log n)` trits.
- **Consequence 4 (Null Clustering & Parallel Additivity):** Disjoint regions add cleanly: `D(U R_l) = sum D(R_l)`. Any subset `C` can be clustered into a single effective node preserving Stokes.
- **Consequence 5 (Isotropic Ignorance):** Equivalence classes `~` with no internal distinctions preserve flux topology under permutation.
- **Appendix A (Channel Capacity Check):** Mutual information in trits `I_p(X; Y)` defines capacity `C(P) = max_p I_p(X; Y)`. Per-step distinguishable information rate `R` MUST satisfy `R <= C(P)` (Corollary 3).

## 5. Automated Falsification Hooks (Executable CI Tests)
1. **Hook 1 (Internal Cancellation):** Assert `|S_int(R)| == 0` for all `R`.
2. **Hook 2 (Boundary Balance):** Assert `S_div(R) == S_d(R)` across `dR`.
3. **Hook 3 (Capacity Consistency):** Assert claimed information rate `R <= C(P)`.
