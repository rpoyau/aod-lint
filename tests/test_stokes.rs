use std::collections::{HashMap, HashSet};
use aod_lint::passes::stokes::AFCStokesChecker;

#[test]
fn test_afc_stokes_invariants() {
    let mut flux = HashMap::new();
    flux.insert((1, 2), 1.5);
    flux.insert((2, 1), -1.5);
    flux.insert((2, 3), 2.0);
    flux.insert((3, 2), -2.0);
    flux.insert((1, 3), 0.5);
    flux.insert((3, 1), -0.5);

    assert!(AFCStokesChecker::verify_antisymmetry(&flux, 1e-9));

    let region: HashSet<usize> = [1, 2].into();
    let all_nodes: HashSet<usize> = [1, 2, 3].into();

    let s_int = AFCStokesChecker::verify_internal_cancellation(&region, &flux);
    assert!(s_int.abs() < 1e-9);

    let mut divergence = HashMap::new();
    divergence.insert(1, 2.0);
    divergence.insert(2, 0.5);
    divergence.insert(3, -2.5);

    let (balanced, s_div, s_boundary) =
        AFCStokesChecker::verify_stokes_balance(&region, &all_nodes, &flux, &divergence, 1e-9);

    assert!(balanced);
    assert!((s_div - 2.5).abs() < 1e-9);
    assert!((s_boundary - 2.5).abs() < 1e-9);
}
