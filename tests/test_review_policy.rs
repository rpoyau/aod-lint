use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

use aod_lint::models::manifest::DocumentTier;
use aod_lint::models::rubric::{Assessment, Section8Rubric};
use aod_lint::passes::digest::AodDigestPass;
use aod_lint::passes::slice::AodSlicePass;
use serde_json::Value;

static SERIAL: AtomicU64 = AtomicU64::new(0);

struct Project(PathBuf);
impl Project {
    fn new() -> Self {
        let root = std::env::temp_dir().join(format!(
            "aod_review_policy_{}_{}",
            std::process::id(),
            SERIAL.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir_all(&root).unwrap();
        Self(root)
    }
    fn write(&self, name: &str, text: &str) {
        let path = self.0.join(name);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, text).unwrap();
    }
    fn cli(&self, args: &[&str]) -> std::process::Output {
        Command::new(env!("CARGO_BIN_EXE_aod-lint"))
            .args(args)
            .current_dir(&self.0)
            .output()
            .unwrap()
    }
}
impl Drop for Project {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

#[test]
fn semantic_rubric_starts_unassessed_without_a_pass_mask() {
    let rubric = Section8Rubric::new();
    assert_eq!(rubric.questions.len(), 8);
    assert!(rubric
        .questions
        .iter()
        .all(|q| q.status == Assessment::NotChecked));
    let json = serde_json::to_value(rubric).unwrap();
    assert!(json.get("converged").is_none());
    assert!(json.get("rubric_bits").is_none());
}

#[test]
fn empty_lexical_scan_does_not_claim_semantic_certification() {
    let p = Project::new();
    p.write(
        "main.tex",
        "A statement whose proof has not been inspected.\n",
    );
    let output = p.cli(&["check", "main.tex", "--json"]);
    assert!(output.status.success());
    let report: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(report["schema_version"], "2.0");
    assert_eq!(report["coverage"], "static_source");
    assert_eq!(report["semantic_review"], "not_checked");
    assert_eq!(report["hoisting"], "not_performed");
    assert!(report.get("converged").is_none());
    assert!(report.get("rubric_bits").is_none());
    assert!(report.get("hoisted_dependencies").is_none());
    assert_eq!(report["rubric"]["questions"].as_array().unwrap().len(), 8);
    assert!(report["rubric"]["questions"]
        .as_array()
        .unwrap()
        .iter()
        .all(|q| q["status"] == "not_checked"));
    let human = p.cli(&["check", "main.tex"]);
    let text = String::from_utf8(human.stdout).unwrap();
    assert!(!text.contains("100%"));
    assert!(text.contains("not assessed"));
}

#[test]
fn strict_keyword_candidates_are_not_reported_as_semantic_proofs_or_hoisting() {
    let p = Project::new();
    for ext in ["tex", "typ"] {
        let file = format!("main.{ext}");
        p.write(&file, "A smooth manifold is part of the declared model.\n");
        let out = p.cli(&["check", &file, "--strict", "--json"]);
        assert_eq!(out.status.code(), Some(1));
        let report: Value = serde_json::from_slice(&out.stdout).unwrap();
        assert_eq!(report["semantic_review"], "not_checked");
        assert_eq!(report["candidate_dependencies"], 1);
        assert_eq!(report["diagnostics"][0]["dependency_candidate"], true);
        assert!(report["diagnostics"][0].get("hoisted").is_none());
        assert!(report["diagnostics"][0]["message"]
            .as_str()
            .unwrap()
            .contains("term alone does not establish"));
    }
}

#[test]
fn default_main_ownership_does_not_freeze_authorized_edits() {
    let p = Project::new();
    p.write("main.tex", "\\input{sections/model}\n");
    p.write("sections/model.tex", "\\label{eq:model}\n");
    p.write("appendices/proof.tex", "\\label{eq:proof}\n");
    let baseline = AodDigestPass::new(&p.0).run().unwrap();
    assert!(baseline.files.iter().all(|f| !f.frozen));
    p.write(
        "sections/model.tex",
        "\\label{eq:model}\nAuthorized correction.\n",
    );
    let after = AodDigestPass::new(&p.0).run().unwrap();
    assert!(AodDigestPass::verify_frozen_baseline(&baseline, &after).is_empty());
    let slice = AodSlicePass::new(&after, 1).run_from_files(vec!["sections/model.tex".into()]);
    assert!(slice.frozen_main_violations.is_empty());
    assert_eq!(slice.excerpts[0].tier, DocumentTier::Main);
}

#[test]
fn explicit_freeze_keeps_baseline_and_slice_guards() {
    let p = Project::new();
    p.write("main.tex", "\\input{sections/model}\n");
    p.write("sections/model.tex", "\\label{eq:model}\n");
    p.write("appendices/proof.tex", "\\label{eq:proof}\n");
    p.write("manual2/main.tex", "\\label{sec:application}\n");
    let baseline = AodDigestPass::new(&p.0)
        .with_frozen_main(true)
        .run()
        .unwrap();
    assert!(baseline
        .files
        .iter()
        .filter(|f| f.tier != DocumentTier::Manual)
        .all(|f| f.frozen));
    assert!(
        !baseline
            .files
            .iter()
            .find(|f| f.tier == DocumentTier::Manual)
            .unwrap()
            .frozen
    );
    p.write("sections/model.tex", "\\label{eq:model}\nChanged.\n");
    let after = AodDigestPass::new(&p.0).run().unwrap();
    assert_eq!(
        AodDigestPass::verify_frozen_baseline(&baseline, &after).len(),
        1
    );
    let slice = AodSlicePass::new(&baseline, 1).run_from_files(vec!["sections/model.tex".into()]);
    assert_eq!(slice.frozen_main_violations.len(), 1);
    fs::remove_file(p.0.join("appendices/proof.tex")).unwrap();
    let after = AodDigestPass::new(&p.0).run().unwrap();
    assert_eq!(
        AodDigestPass::verify_frozen_baseline(&baseline, &after).len(),
        2
    );
}

#[test]
fn manual_two_firewall_is_independent_of_preservation_policy() {
    let p = Project::new();
    p.write("main.tex", "\\ref{sec:application}\n");
    p.write("manual2/main.tex", "\\label{sec:application}\n");
    for freeze in [false, true] {
        let manifest = AodDigestPass::new(&p.0)
            .with_frozen_main(freeze)
            .run()
            .unwrap();
        assert_eq!(manifest.firewall_violations.len(), 1);
        assert_eq!(manifest.firewall_violations[0].origin_file, "main.tex");
        let manual = manifest
            .files
            .iter()
            .find(|f| f.path == "manual2/main.tex")
            .unwrap();
        assert_eq!(manual.tier, DocumentTier::Manual);
        assert!(!manual.frozen);
    }
}

#[test]
fn cli_freeze_is_opt_in_and_existing_baseline_is_enforced_without_reselecting_it() {
    let p = Project::new();
    p.write("main.tex", "\\label{sec:model}\n");
    let normal = p.cli(&["digest", ".", "--json"]);
    assert!(normal.status.success());
    let manifest: Value = serde_json::from_slice(&normal.stdout).unwrap();
    assert_eq!(manifest["files"][0]["frozen"], false);
    let frozen = p.cli(&["digest", ".", "--freeze-main", "--output", "baseline.json"]);
    assert!(frozen.status.success());
    let manifest: Value =
        serde_json::from_slice(&fs::read(p.0.join("baseline.json")).unwrap()).unwrap();
    assert_eq!(manifest["files"][0]["frozen"], true);
    assert!(p
        .cli(&["digest", ".", "--baseline", "baseline.json", "--json"])
        .status
        .success());
    p.write("main.tex", "\\label{sec:model}\nChanged.\n");
    let result = p.cli(&["digest", ".", "--baseline", "baseline.json", "--json"]);
    assert_eq!(result.status.code(), Some(1));
    let result: Value = serde_json::from_slice(&result.stdout).unwrap();
    let violations = result["frozen_baseline_violations"].as_array().unwrap();
    assert_eq!(violations.len(), 1);
    assert!(violations[0].as_str().unwrap().contains("main.tex"));
}

#[test]
fn cross_document_duplicate_labels_are_partial_not_false_manual_dependencies() {
    let p = Project::new();
    p.write("main.tex", "\\ref{eq:shared}\n");
    p.write("sections/model.tex", "\\label{eq:shared}\n");
    p.write(
        "manual2/vendor/old/sections/model.tex",
        "\\label{eq:shared}\n",
    );
    let manifest = AodDigestPass::new(&p.0).run().unwrap();
    assert!(manifest.firewall_violations.is_empty());
    assert_eq!(manifest.coverage_gaps.len(), 1);
    assert_eq!(
        manifest
            .edges
            .iter()
            .filter(|e| e.origin == "main.tex")
            .count(),
        2
    );
    let output = p.cli(&["digest", ".", "--json"]);
    assert_eq!(output.status.code(), Some(2));
    let report: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(report["coverage_gaps"].as_array().unwrap().len(), 1);
}

#[test]
fn multiple_manual_candidates_do_not_hide_a_main_to_manual_dependency() {
    let p = Project::new();
    p.write("main.tex", "\\ref{eq:manual}\n");
    p.write("manual/main.tex", "\\label{eq:manual}\n");
    p.write("manual2/main.tex", "\\label{eq:manual}\n");
    let manifest = AodDigestPass::new(&p.0).run().unwrap();
    assert_eq!(manifest.firewall_violations.len(), 1);
    assert_eq!(manifest.coverage_gaps.len(), 1);
    assert_eq!(p.cli(&["digest", ".", "--json"]).status.code(), Some(1));
}
