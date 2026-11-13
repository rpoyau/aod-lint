use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use aod_lint::models::diagnostics::{RuleAnchor, Severity};
use aod_lint::models::rubric::Section8Rubric;
use aod_lint::passes::hygiene::RepoHygienePass;

fn make_temp_repo(name: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let dir = std::env::temp_dir().join(format!("aod_hygiene_test_{}_{}", name, nanos));
    fs::create_dir_all(dir.join("sections")).unwrap();
    fs::create_dir_all(dir.join("figures")).unwrap();
    dir
}

#[test]
fn test_clean_scientific_repo_passes_hygiene() {
    let dir = make_temp_repo("clean");
    fs::write(dir.join("main.typ"), "= Clean Paper\n").unwrap();
    fs::write(dir.join("figures/diagram.pdf"), b"%PDF-1.4 figure").unwrap();

    let diags = RepoHygienePass::new(&dir, 1_048_576).run().unwrap();
    assert!(diags.is_empty(), "Expected clean repo, got {:?}", diags);

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_agent_state_and_session_leakage_detected() {
    let dir = make_temp_repo("leaked");
    fs::create_dir_all(dir.join(".agents/skills")).unwrap();
    fs::write(dir.join(".agents/skills/SKILL.md"), "agent skill").unwrap();
    fs::write(dir.join("AGENTS.md"), "agent rules").unwrap();
    fs::create_dir_all(dir.join("manual/data/m3/historical_metrology")).unwrap();
    fs::write(
        dir.join("manual/data/m3/historical_metrology/original_replay_r01.zip"),
        b"PK\x03\x04fakezip",
    )
    .unwrap();
    let leaked_line = concat!(
        "See https://chatgpt.com/", "library?file_id=", "lib", "file_abc123&", "account_id=", "personal ",
        "and /workspace/", "scratch/27f6/run ",
        "and https://github.com/rpoyau/", "aod-", "skills/blob/main/plan.md\n"
    );
    fs::write(
        dir.join("manual/data/m3/historical_metrology/historical_replay_review.md"),
        leaked_line,
    )
    .unwrap();
    fs::write(dir.join("main.pdf"), b"%PDF-1.4 compiled root pdf").unwrap();
    fs::write(dir.join("BUILD.md"), "Implements MAP-21 and WP4.\n").unwrap();

    let diags = RepoHygienePass::new(&dir, 1_048_576).run().unwrap();
    assert!(diags.iter().any(|d| d.severity == Severity::Fatal
        && d.file == "AGENTS.md"
        && d.rule == RuleAnchor::RepoHygieneAgentLeakage));
    assert!(diags.iter().any(|d| d.severity == Severity::Fatal
        && d.message.contains(concat!("chatgpt.com/", "library"))));
    assert!(diags
        .iter()
        .any(|d| d.severity == Severity::Error && d.message.contains(concat!("/workspace/", "scratch/"))));
    assert!(diags
        .iter()
        .any(|d| d.severity == Severity::Error && d.message.contains(concat!("aod-", "skills/blob/"))));
    assert!(diags
        .iter()
        .any(|d| d.severity == Severity::Error && d.file.ends_with("original_replay_r01.zip")));
    assert!(diags
        .iter()
        .any(|d| d.severity == Severity::Error && d.file == "main.pdf"));
    assert!(diags
        .iter()
        .any(|d| d.severity == Severity::Warning && d.file == "BUILD.md"));

    let mut rubric = Section8Rubric::new();
    rubric.apply_diagnostics(&diags);
    assert!(!rubric.is_converged());
    assert_eq!(rubric.0 & Section8Rubric::C5_DEMOS_ISOLATED, 0);

    let _ = fs::remove_dir_all(&dir);
}
