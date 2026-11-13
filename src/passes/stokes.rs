use std::collections::{HashMap, HashSet};

pub struct AFCStokesChecker;

impl AFCStokesChecker {
    pub fn verify_antisymmetry(flux_matrix: &HashMap<(usize, usize), f64>, tol: f64) -> bool {
        for (&(i, j), &phi_ij) in flux_matrix {
            let phi_ji = *flux_matrix.get(&(j, i)).unwrap_or(&0.0);
            if (phi_ij + phi_ji).abs() > tol {
                return false;
            }
        }
        true
    }

    pub fn verify_internal_cancellation(
        region: &HashSet<usize>,
        flux_matrix: &HashMap<(usize, usize), f64>,
    ) -> f64 {
        let mut s_int = 0.0;
        for &i in region {
            for &j in region {
                if let Some(&phi) = flux_matrix.get(&(i, j)) {
                    s_int += phi;
                }
            }
        }
        s_int
    }

    pub fn verify_stokes_balance(
        region: &HashSet<usize>,
        all_nodes: &HashSet<usize>,
        flux_matrix: &HashMap<(usize, usize), f64>,
        divergence: &HashMap<usize, f64>,
        tolerance: f64,
    ) -> (bool, f64, f64) {
        let s_div: f64 = region.iter().map(|i| divergence.get(i).unwrap_or(&0.0)).sum();
        let outside: HashSet<_> = all_nodes.difference(region).copied().collect();

        let mut s_boundary = 0.0;
        for &i in region {
            for &j in &outside {
                if let Some(&phi) = flux_matrix.get(&(i, j)) {
                    s_boundary += phi;
                }
            }
        }

        let discrepancy = (s_div - s_boundary).abs();
        (discrepancy <= tolerance, s_div, s_boundary)
    }
}
