use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use aod_lint::passes::labels::{check_labels, LabelHygieneOptions};

static NEXT_FIXTURE: AtomicU64 = AtomicU64::new(0);

struct Project {
    root: PathBuf,
    originals: Vec<(PathBuf, Vec<u8>)>,
}

impl Project {
    fn new(name: &str, files: &[(&str, &str)]) -> Self {
        let stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let serial = NEXT_FIXTURE.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "aod_inactive_labels_{}_{}_{}_{}",
            name,
            std::process::id(),
            stamp,
            serial
        ));
        fs::create_dir_all(&root).unwrap();
        let mut originals = Vec::new();
        for (relative, text) in files {
            let path = root.join(relative);
            fs::create_dir_all(path.parent().unwrap()).unwrap();
            fs::write(&path, text).unwrap();
            originals.push((path, text.as_bytes().to_vec()));
        }
        Self { root, originals }
    }

    fn entry(&self, name: &str) -> PathBuf {
        self.root.join(name)
    }

    fn assert_unchanged(&self) {
        for (path, bytes) in &self.originals {
            assert_eq!(&fs::read(path).unwrap(), bytes);
        }
    }
}

impl Drop for Project {
    fn drop(&mut self) {
        // Only this test's uniquely created temporary directory is removed.
        let _ = fs::remove_dir_all(&self.root);
    }
}

fn check(entry: &Path) -> aod_lint::passes::labels::LabelHygieneReport {
    check_labels(entry, LabelHygieneOptions::default()).unwrap()
}

#[test]
fn unused_xparse_command_does_not_supply_a_live_label() {
    let source = r"\NewDocumentCommand{\unused}{m}{\label{eq:ghost}}
\ref{eq:ghost}
";
    let project = Project::new("xparse", &[("main.tex", source)]);
    let report = check(&project.entry("main.tex"));
    assert!(!report.labels.iter().any(|label| label.name == "eq:ghost"));
    assert!(report.diagnostics.iter().any(|diagnostic| {
        diagnostic.kind == "unresolved_reference" && diagnostic.message.contains("eq:ghost")
    }));
    assert!(!report.coverage_gaps.is_empty());
    assert_eq!(report.exit_code(), 2);
    project.assert_unchanged();
}

#[test]
fn unexecuted_typst_loops_do_not_supply_live_labels() {
    for (name, inactive) in [
        ("for", "#for i in () [<eq:ghost>]"),
        ("while", "#while false [<eq:ghost>]"),
    ] {
        let source = format!("{inactive}\nSee @eq:ghost.\n");
        let project = Project::new(name, &[("main.typ", &source)]);
        let report = check(&project.entry("main.typ"));
        assert!(!report.labels.iter().any(|label| label.name == "eq:ghost"));
        assert!(report.diagnostics.iter().any(|diagnostic| {
            diagnostic.kind == "unresolved_typst_reference"
                && diagnostic.message.contains("eq:ghost")
        }));
        assert!(!report.coverage_gaps.is_empty());
        assert_eq!(report.exit_code(), 2);
        project.assert_unchanged();
    }
}

#[test]
fn labels_passed_to_functions_are_not_assumed_to_be_rendered() {
    for (name, call) in [
        ("literal_argument", "#repr(<eq:ghost>)"),
        ("content_argument", "#repr([<eq:ghost>])"),
    ] {
        let source = format!("{call}\nSee @eq:ghost.\n");
        let project = Project::new(name, &[("main.typ", &source)]);
        let report = check(&project.entry("main.typ"));
        assert!(!report.labels.iter().any(|label| label.name == "eq:ghost"));
        assert!(report.diagnostics.iter().any(|diagnostic| {
            diagnostic.kind == "unresolved_typst_reference"
                && diagnostic.message.contains("eq:ghost")
        }));
        assert!(!report.coverage_gaps.is_empty());
        assert_eq!(report.exit_code(), 2);
        project.assert_unchanged();
    }
}

#[test]
fn labels_after_endinput_do_not_resolve_live_references() {
    let source = r"\ref{eq:ghost}
\endinput
\label{eq:ghost}
";
    let project = Project::new("endinput", &[("main.tex", source)]);
    let report = check(&project.entry("main.tex"));
    assert!(!report.labels.iter().any(|label| label.name == "eq:ghost"));
    assert!(report.diagnostics.iter().any(|diagnostic| {
        diagnostic.kind == "unresolved_reference" && diagnostic.message.contains("eq:ghost")
    }));
    assert!(!report.coverage_gaps.is_empty());
    assert_eq!(report.exit_code(), 2);
    project.assert_unchanged();
}

#[test]
fn includeonly_does_not_create_a_false_duplicate_from_excluded_content() {
    let source = r"\includeonly{a}
\include{a}
\include{b}
\input{direct}
\ref{eq:direct}
";
    let project = Project::new(
        "includeonly",
        &[
            ("main.tex", source),
            ("a.tex", "\\label{eq:shared}\n"),
            ("b.tex", "\\label{eq:shared}\n"),
            ("direct.tex", "\\label{eq:direct}\n"),
        ],
    );
    let report = check(&project.entry("main.tex"));
    assert!(!report
        .diagnostics
        .iter()
        .any(|diagnostic| diagnostic.kind == "duplicate_label"));
    // Selection is deliberately not evaluated: both \include targets stay
    // outside the literal coverage, while unconditional \input still works.
    assert!(!report.labels.iter().any(|label| label.name == "eq:shared"));
    assert!(report.labels.iter().any(|label| label.name == "eq:direct"));
    assert_eq!(report.files.len(), 2);
    assert!(!report.coverage_gaps.is_empty());
    assert_eq!(report.exit_code(), 2);
    project.assert_unchanged();
}
