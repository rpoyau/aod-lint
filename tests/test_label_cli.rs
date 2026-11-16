use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;
use std::process::{Command, Output};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use serde_json::Value;
use sha2::{Digest, Sha256};

static NEXT_FIXTURE: AtomicU64 = AtomicU64::new(0);

struct Project {
    root: PathBuf,
    originals: BTreeMap<String, Vec<u8>>,
}

impl Project {
    fn new(name: &str, files: &[(&str, &str)]) -> Self {
        let tick = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let serial = NEXT_FIXTURE.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "aod_label_cli_{}_{}_{}_{}",
            name,
            std::process::id(),
            tick,
            serial
        ));
        fs::create_dir_all(&root).unwrap();
        let mut originals = BTreeMap::new();
        for (relative, source) in files {
            let path = root.join(relative);
            fs::create_dir_all(path.parent().unwrap()).unwrap();
            fs::write(path, source).unwrap();
            originals.insert((*relative).to_owned(), source.as_bytes().to_vec());
        }
        Self { root, originals }
    }

    fn command(&self) -> Command {
        let mut command = Command::new(env!("CARGO_BIN_EXE_aod-lint"));
        command
            .arg("labels")
            .arg(self.root.join("main.tex"))
            .arg("--json");
        command
    }

    fn assert_unchanged(&self) {
        for (relative, bytes) in &self.originals {
            assert_eq!(fs::read(self.root.join(relative)).unwrap(), *bytes);
        }
    }
}

impl Drop for Project {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

fn json_output(command: &mut Command, expected_code: i32) -> Value {
    let Output {
        status,
        stdout,
        stderr,
    } = command.output().unwrap();
    assert_eq!(
        status.code(),
        Some(expected_code),
        "stdout: {}\nstderr: {}",
        String::from_utf8_lossy(&stdout),
        String::from_utf8_lossy(&stderr)
    );
    assert!(stderr.is_empty(), "{}", String::from_utf8_lossy(&stderr));
    serde_json::from_slice(&stdout).expect("JSON mode must emit one valid JSON document")
}

fn kinds(report: &Value) -> Vec<&str> {
    report["diagnostics"]
        .as_array()
        .unwrap()
        .iter()
        .map(|d| d["kind"].as_str().unwrap())
        .collect()
}

#[test]
fn clear_scan_emits_json_and_zero_exit_status() {
    let project = Project::new(
        "clear",
        &[("main.tex", "\\label{eq:balance}\n\\ref{eq:balance}\n")],
    );
    let report = json_output(&mut project.command(), 0);
    assert_eq!(report["status"], "LITERAL_SCAN_CLEAR");
    assert!(kinds(&report).is_empty());
    assert_eq!(report["labels"][0]["name"], "eq:balance");
    assert_eq!(report["files"].as_array().unwrap().len(), 1);
    assert!(report["coverage_gaps"].as_array().unwrap().is_empty());
    project.assert_unchanged();
}

#[test]
fn broken_reference_emits_error_json_and_exit_one() {
    let project = Project::new("broken", &[("main.tex", "\\ref{eq:missing}\n")]);
    let report = json_output(&mut project.command(), 1);
    assert_eq!(report["status"], "ISSUES_FOUND");
    assert!(kinds(&report).contains(&"unresolved_reference"));
    project.assert_unchanged();
}

#[test]
fn unevaluated_dynamic_label_emits_partial_json_and_exit_two() {
    let project = Project::new("partial", &[("main.tex", "\\label{\\computedkey}\n")]);
    let report = json_output(&mut project.command(), 2);
    assert_eq!(report["status"], "PARTIAL");
    assert!(!report["coverage_gaps"].as_array().unwrap().is_empty());
    assert!(report["labels"].as_array().unwrap().is_empty());
    project.assert_unchanged();
}

#[test]
fn malformed_context_is_an_input_error_for_both_context_flags() {
    for (index, contents) in ["not json", "{}", "[42]", "[\"\"]", "[\"  \"]"]
        .iter()
        .enumerate()
    {
        let project = Project::new(
            &format!("invalid_context_{index}"),
            &[("main.tex", ""), ("context.json", contents)],
        );
        for flag in ["--external-labels", "--paths"] {
            let report = json_output(
                project
                    .command()
                    .arg(flag)
                    .arg(project.root.join("context.json")),
                1,
            );
            assert_eq!(report["status"], "INPUT_ERROR");
            assert!(!report["message"].as_str().unwrap().is_empty());
            assert!(report.get("labels").is_none());
        }
        project.assert_unchanged();
    }
}

#[test]
fn external_labels_bind_exact_context_bytes_and_enable_explicit_wrapper_prefix() {
    let context = "[\n  \"main:eq:balance\"\n]\n";
    let project = Project::new(
        "external",
        &[
            ("main.tex", "\\mainequation{eq:balance}\n"),
            ("external.json", context),
        ],
    );
    let context_path = project.root.join("external.json");
    let report = json_output(
        project
            .command()
            .arg("--external-labels")
            .arg(&context_path)
            .args(["--main-reference-prefix", "main:"]),
        0,
    );
    assert_eq!(report["status"], "LITERAL_SCAN_CLEAR");
    assert_eq!(report["external_label_count"], 1);
    assert_eq!(report["main_reference_prefix"], "main:");
    assert_eq!(
        report["external_labels_input"]["path"],
        context_path.to_str().unwrap()
    );
    assert_eq!(
        report["external_labels_input"]["sha256"],
        format!("{:x}", Sha256::digest(context.as_bytes()))
    );
    project.assert_unchanged();
}

#[test]
fn additional_paths_warn_without_loading_files_and_bind_exact_context_bytes() {
    let context = "[\n \"scripts/aod_m3_temporal.py\",\n \"manual/data/M31/input.json\"\n]\n";
    let project = Project::new(
        "paths",
        &[
            ("main.tex", "\\label{eq:balance}\n"),
            ("paths.json", context),
        ],
    );
    let context_path = project.root.join("paths.json");
    let report = json_output(project.command().arg("--paths").arg(&context_path), 0);
    assert_eq!(report["status"], "WARNINGS");
    assert_eq!(kinds(&report), ["workflow_path"]);
    assert_eq!(report["files"].as_array().unwrap().len(), 1);
    assert_eq!(
        report["additional_paths_input"]["path"],
        context_path.to_str().unwrap()
    );
    assert_eq!(
        report["additional_paths_input"]["sha256"],
        format!("{:x}", Sha256::digest(context.as_bytes()))
    );
    assert!(!project.root.join("scripts").exists());
    let quiet = json_output(
        project
            .command()
            .arg("--paths")
            .arg(&context_path)
            .arg("--no-name-warnings"),
        0,
    );
    assert_eq!(quiet["status"], "LITERAL_SCAN_CLEAR");
    assert_eq!(quiet["name_checks_enabled"], false);
    assert!(kinds(&quiet).is_empty());
    project.assert_unchanged();
}

#[test]
fn disabling_name_warnings_keeps_duplicate_label_errors() {
    let project = Project::new(
        "quiet_duplicate",
        &[(
            "main.tex",
            "\\label{eq:m3-balance}\n\\label{eq:m3-balance}\n",
        )],
    );
    let report = json_output(project.command().arg("--no-name-warnings"), 1);
    assert_eq!(report["status"], "ISSUES_FOUND");
    assert_eq!(report["name_checks_enabled"], false);
    assert!(kinds(&report).contains(&"duplicate_label"));
    assert!(!kinds(&report)
        .iter()
        .any(|kind| kind.starts_with("workflow_")));
    project.assert_unchanged();
}

#[test]
fn relative_context_inputs_are_bound_to_absolute_paths_for_later_review() {
    let project = Project::new(
        "relative_context",
        &[
            ("main.tex", "\\ref{main:eq:existing}\n"),
            ("external.json", "[\"main:eq:existing\"]\n"),
            ("paths.json", "[\"manual/example.tex\"]\n"),
        ],
    );
    let report = json_output(
        project.command().current_dir(&project.root).args([
            "--external-labels",
            "external.json",
            "--paths",
            "paths.json",
        ]),
        0,
    );
    for (key, file) in [
        ("external_labels_input", "external.json"),
        ("additional_paths_input", "paths.json"),
    ] {
        let bound = PathBuf::from(report[key]["path"].as_str().unwrap());
        assert!(bound.is_absolute());
        assert_eq!(bound, fs::canonicalize(project.root.join(file)).unwrap());
        assert_eq!(
            report[key]["sha256"],
            format!("{:x}", Sha256::digest(fs::read(bound).unwrap()))
        );
    }
    project.assert_unchanged();
}
