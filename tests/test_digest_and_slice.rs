use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use aod_lint::models::manifest::DocumentTier;
use aod_lint::passes::digest::AodDigestPass;
use aod_lint::passes::slice::AodSlicePass;

fn make_temp_project(name: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let dir = std::env::temp_dir().join(format!("aod_lint_test_{}_{}", name, nanos));
    fs::create_dir_all(dir.join("sections")).unwrap();
    fs::create_dir_all(dir.join("manual")).unwrap();
    dir
}

#[test]
fn test_digest_manifest_frozen_guard_and_slice() {
    let dir = make_temp_project("digest_slice");

    fs::write(
        dir.join("main.typ"),
        "= Alpha Omega Dynamics\n#include \"sections/01_null.typ\"\n",
    )
    .unwrap();

    fs::write(
        dir.join("sections/01_null.typ"),
        "== Null Sameness <sec:null>\nAxiom 1: N = infinity.\nMonon closure @eq:monon[Eq. 14].\n$ Phi = 1_C $ <eq:monon>\n",
    )
    .unwrap();

    fs::write(
        dir.join("manual/01_dec_ledger.typ"),
        "== D.E.C. Ledger Example <sec:dec-demo>\nUsing @eq:monon[Eq. 14], Choice D1 encodes SADAR.\n",
    )
    .unwrap();

    fs::write(
        dir.join("manual/02_unrelated.typ"),
        "== Unrelated Note <sec:unrelated>\nIsolated note with no shared references.\n",
    )
    .unwrap();

    let digest = AodDigestPass::new(&dir).with_frozen_main(true);
    let baseline = digest.run().unwrap();

    assert_eq!(baseline.files.len(), 4);
    assert!(baseline.firewall_violations.is_empty());
    assert!(baseline.include_cycles.is_empty());

    let sec_null = baseline
        .files
        .iter()
        .find(|f| f.path == "sections/01_null.typ")
        .unwrap();
    assert_eq!(sec_null.tier, DocumentTier::Main);
    assert!(sec_null.frozen);
    assert!(sec_null.motifs.contains(&"Monon".to_string()));

    let man_dec = baseline
        .files
        .iter()
        .find(|f| f.path == "manual/01_dec_ledger.typ")
        .unwrap();
    assert_eq!(man_dec.tier, DocumentTier::Manual);
    assert!(!man_dec.frozen);
    assert!(man_dec.motifs.contains(&"DEC".to_string()));
    assert!(man_dec.motifs.contains(&"SADAR".to_string()));

    // Slice from a diff touching only manual/01_dec_ledger.typ
    let diff = "\
diff --git a/manual/01_dec_ledger.typ b/manual/01_dec_ledger.typ
--- a/manual/01_dec_ledger.typ
+++ b/manual/01_dec_ledger.typ
@@ -1,2 +1,2 @@
 == D.E.C. Ledger Example <sec:dec-demo>
-Using @eq:monon[Eq. 14], Choice D1 encodes SADAR.
+Using @eq:monon[Eq. 14], Choice D1 encodes SADAR and Monon.
";
    let slicer = AodSlicePass::new(&baseline, 1);
    let bundle = slicer.run_from_diff(diff);
    assert!(bundle.frozen_main_violations.is_empty());
    assert!(bundle
        .impacted_files
        .contains(&"manual/01_dec_ledger.typ".to_string()));
    assert!(bundle
        .impacted_files
        .contains(&"sections/01_null.typ".to_string()));
    assert!(!bundle
        .impacted_files
        .contains(&"manual/02_unrelated.typ".to_string()));

    // Mutate frozen Main file and verify frozen SHA-256 guard triggers
    fs::write(
        dir.join("sections/01_null.typ"),
        "== Null Sameness <sec:null>\nMutated frozen main!\n",
    )
    .unwrap();
    let updated = digest.run().unwrap();
    let frozen_errs = AodDigestPass::verify_frozen_baseline(&baseline, &updated);
    assert_eq!(frozen_errs.len(), 1);
    assert!(frozen_errs[0].contains("sections/01_null.typ"));

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_main_to_manual_firewall_violation() {
    let dir = make_temp_project("firewall");

    fs::write(
        dir.join("sections/01_bad.typ"),
        "== Bad Main <sec:bad>\nSee @sec:demo[Manual] for proof.\n",
    )
    .unwrap();
    fs::write(
        dir.join("manual/demo.typ"),
        "== Manual Demo <sec:demo>\nWorked example.\n",
    )
    .unwrap();

    let manifest = AodDigestPass::new(&dir).run().unwrap();
    assert_eq!(manifest.firewall_violations.len(), 1);
    assert_eq!(
        manifest.firewall_violations[0].origin_file,
        "sections/01_bad.typ"
    );

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_latex_digest_slice_and_check() {
    use aod_lint::models::diagnostics::RuleAnchor;
    use aod_lint::passes::latex::LatexPass;

    let dir = make_temp_project("latex_support");

    fs::write(
        dir.join("main.tex"),
        "\\documentclass{article}\n\\input{sections/01_null.tex}\n",
    )
    .unwrap();

    fs::write(
        dir.join("sections/01_null.tex"),
        "\\section{Null Sameness}\\label{sec:null}\nMonon closure \\eqnref{eq:monon}.\n\\begin{equation}\\label{eq:monon}\\Phi = 1_C\\end{equation}\n",
    )
    .unwrap();

    fs::write(
        dir.join("manual/01_dec.tex"),
        "\\section{D.E.C. Ledger}\\label{sec:dec}\nUsing \\mainequation{eq:monon}, \\SADARop{} is evaluated.\nSee \\cite{poyau2025} for background.\n",
    )
    .unwrap();

    let manifest = AodDigestPass::new(&dir).run().unwrap();
    assert_eq!(manifest.files.len(), 3);
    assert!(manifest.firewall_violations.is_empty());
    assert!(manifest.include_cycles.is_empty());

    let sec_null = manifest
        .files
        .iter()
        .find(|f| f.path == "sections/01_null.tex")
        .unwrap();
    assert_eq!(sec_null.tier, DocumentTier::Main);
    assert!(!sec_null.frozen);
    assert!(sec_null.motifs.contains(&"Monon".to_string()));

    let man_dec = manifest
        .files
        .iter()
        .find(|f| f.path == "manual/01_dec.tex")
        .unwrap();
    assert_eq!(man_dec.tier, DocumentTier::Manual);
    assert!(!man_dec.frozen);
    assert!(man_dec.motifs.contains(&"SADAR".to_string()));
    assert!(man_dec.motifs.contains(&"DEC".to_string()));

    let bundle =
        AodSlicePass::new(&manifest, 1).run_from_files(vec!["manual/01_dec.tex".to_string()]);
    assert!(bundle.frozen_main_violations.is_empty());
    assert!(bundle
        .impacted_files
        .contains(&"sections/01_null.tex".to_string()));

    let man_src = fs::read_to_string(dir.join("manual/01_dec.tex")).unwrap();
    let diags = LatexPass::new("manual/01_dec.tex", &man_src, false).run();
    assert_eq!(diags.len(), 1);
    assert_eq!(diags[0].rule, RuleAnchor::Sec7PreciseAnchors);

    let _ = fs::remove_dir_all(&dir);
}
