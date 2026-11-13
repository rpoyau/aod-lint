use std::fs;
use std::path::PathBuf;

use aod_lint::models::manifest::{AodManifest, DocumentTier};
use aod_lint::models::rubric::Section8Rubric;
use aod_lint::passes::digest::{collect_source_files, AodDigestPass};
use aod_lint::passes::hygiene::RepoHygienePass;
use aod_lint::passes::slice::AodSlicePass;
use aod_lint::passes::typst_ast::TypstAstPass;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

#[test]
fn test_self_manuscript_strict_af_and_gm_compliance() {
    let root = repo_root();
    let mut rel_files = Vec::new();
    collect_source_files(&root, &root, &mut rel_files).unwrap();
    rel_files.sort();

    assert_eq!(rel_files.len(), 23, "Expected 13 Main/Appendix + 10 Manual .typ files");

    let mut all_diagnostics = Vec::new();
    for rel in &rel_files {
        let content = fs::read_to_string(root.join(rel)).unwrap();
        let diags = TypstAstPass::new(rel, &content, true).run();
        assert!(
            diags.is_empty(),
            "Expected zero strict diagnostics in '{}', got: {:?}",
            rel,
            diags
        );
        all_diagnostics.extend(diags);
    }

    let mut rubric = Section8Rubric::new();
    rubric.apply_diagnostics(&all_diagnostics);
    assert!(rubric.is_converged());
    assert_eq!(rubric.to_bitstring(), "11111111");
}

#[test]
fn test_self_manuscript_frozen_baseline_firewall_and_slicing() {
    let root = repo_root();
    let current = AodDigestPass::new(&root).run().unwrap();

    assert!(
        current.firewall_violations.is_empty(),
        "Unexpected Main -> Manual firewall violations: {:?}",
        current.firewall_violations
    );
    assert!(
        current.include_cycles.is_empty(),
        "Unexpected include cycles: {:?}",
        current.include_cycles
    );

    let frozen_count = current.files.iter().filter(|f| f.frozen).count();
    let manual_count = current
        .files
        .iter()
        .filter(|f| f.tier == DocumentTier::Manual)
        .count();
    assert_eq!(frozen_count, 13);
    assert_eq!(manual_count, 10);

    let baseline_str =
        fs::read_to_string(root.join("contracts/main_baseline_manifest.json")).unwrap();
    let baseline: AodManifest = serde_json::from_str(&baseline_str).unwrap();
    let frozen_diffs = AodDigestPass::verify_frozen_baseline(&baseline, &current);
    assert!(
        frozen_diffs.is_empty(),
        "Frozen Main/Appendix SHA-256 baseline drifted: {:?}",
        frozen_diffs
    );

    // Verify 1-hop blast-radius slice from Manual Stokes fixture resolves Main Section 4
    let bundle = AodSlicePass::new(&current, 1).run_from_files(vec![
        "manual/sections/02_dec_stokes_and_capacity_fixtures.typ".to_string(),
    ]);
    assert!(bundle.frozen_main_violations.is_empty());
    assert!(bundle
        .impacted_files
        .contains(&"sections/05_dec_stokes_and_blast_radius_slicing.typ".to_string()));
    assert!(!bundle
        .impacted_files
        .contains(&"manual/sections/04_hygiene_and_leakage_quarantine.typ".to_string()));
}

#[test]
fn test_self_repository_hygiene() {
    let root = repo_root();
    let diags = RepoHygienePass::new(&root, 1_048_576).run().unwrap();
    assert!(
        diags.is_empty(),
        "Repository failed self-hygiene check: {:?}",
        diags
    );
}
