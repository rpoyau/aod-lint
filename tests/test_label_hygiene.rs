use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use aod_lint::models::diagnostics::Severity;
use aod_lint::passes::labels::{check_labels, LabelHygieneOptions};
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
            "aod_label_hygiene_{}_{}_{}_{}",
            name,
            std::process::id(),
            tick,
            serial
        ));
        fs::create_dir_all(&root).unwrap();
        let mut originals = BTreeMap::new();
        for (relative, text) in files {
            let path = root.join(relative);
            fs::create_dir_all(path.parent().unwrap()).unwrap();
            fs::write(&path, text).unwrap();
            originals.insert((*relative).to_owned(), text.as_bytes().to_vec());
        }
        Self { root, originals }
    }

    fn entry(&self, relative: &str) -> PathBuf {
        self.root.join(relative)
    }

    fn assert_unchanged(&self) {
        for (relative, expected) in &self.originals {
            assert_eq!(fs::read(self.root.join(relative)).unwrap(), *expected);
        }
        fn files_under(root: &Path, directory: &Path, out: &mut BTreeSet<String>) {
            for entry in fs::read_dir(directory).unwrap() {
                let path = entry.unwrap().path();
                if path.is_dir() {
                    files_under(root, &path, out);
                } else {
                    out.insert(
                        path.strip_prefix(root)
                            .unwrap()
                            .to_string_lossy()
                            .replace('\\', "/"),
                    );
                }
            }
        }
        let mut actual = BTreeSet::new();
        files_under(&self.root, &self.root, &mut actual);
        assert_eq!(actual, self.originals.keys().cloned().collect());
    }
}

impl Drop for Project {
    fn drop(&mut self) {
        // Only the unique directory created by this fixture is removed.
        let _ = fs::remove_dir_all(&self.root);
    }
}

#[test]
fn commented_verbatim_opener_does_not_hide_live_labels() {
    let project = Project::new(
        "comment_fake_verbatim",
        &[(
            "main.tex",
            "% \\begin{verbatim}\n\\label{eq:balance}\n\\ref{eq:balance}\n",
        )],
    );
    let report = check_labels(&project.entry("main.tex"), LabelHygieneOptions::default()).unwrap();
    assert_eq!(report.labels.len(), 1);
    assert_eq!(report.labels[0].name, "eq:balance");
    assert!(report.diagnostics.is_empty(), "{:?}", report.diagnostics);
    assert!(
        report.coverage_gaps.is_empty(),
        "{:?}",
        report.coverage_gaps
    );
    assert_eq!(report.exit_code(), 0);
    project.assert_unchanged();
}

#[test]
fn nested_heading_is_partial_instead_of_falsely_clear() {
    let project = Project::new(
        "nested_heading",
        &[("main.tex", "\\section{\\textbf{M3} comparisons}\n")],
    );
    let report = check_labels(&project.entry("main.tex"), LabelHygieneOptions::default()).unwrap();
    assert!(!report.coverage_gaps.is_empty(), "{:?}", report);
    assert_eq!(report.status, "PARTIAL");
    assert_eq!(report.exit_code(), 2);
    project.assert_unchanged();
}

#[test]
fn typst_project_root_include_is_partial_without_assuming_operating_system_root() {
    let project = Project::new(
        "typst_project_root",
        &[
            ("main.typ", "#include \"/section.typ\"\n@eq:balance\n"),
            ("section.typ", "Balance <eq:balance>\n"),
        ],
    );
    let report = check_labels(&project.entry("main.typ"), LabelHygieneOptions::default()).unwrap();
    assert!(!report.coverage_gaps.is_empty(), "{:?}", report);
    assert!(
        !report
            .diagnostics
            .iter()
            .any(|d| d.kind == "missing_include"),
        "{:?}",
        report.diagnostics
    );
    assert!(
        report.labels.is_empty(),
        "an undeclared compiler project root cannot establish the sibling label"
    );
    assert_eq!(report.files.len(), 1);
    assert_eq!(report.status, "PARTIAL");
    assert_eq!(report.exit_code(), 2);
    project.assert_unchanged();
}

#[test]
fn inline_verb_environment_example_does_not_hide_live_labels() {
    let project = Project::new(
        "inline_fake_verbatim",
        &[(
            "main.tex",
            "\\verb|\\begin{verbatim}|\n\\label{eq:balance}\n\\ref{eq:balance}\n",
        )],
    );
    let report = check_labels(&project.entry("main.tex"), LabelHygieneOptions::default()).unwrap();
    assert_eq!(report.labels.len(), 1);
    assert_eq!(report.labels[0].name, "eq:balance");
    assert!(report.diagnostics.is_empty(), "{:?}", report.diagnostics);
    assert!(
        report.coverage_gaps.is_empty(),
        "{:?}",
        report.coverage_gaps
    );
    assert_eq!(report.exit_code(), 0);
    project.assert_unchanged();
}

#[test]
fn unclosed_verbatim_is_partial_and_does_not_define_example_labels() {
    let project = Project::new(
        "unclosed_verbatim",
        &[("main.tex", "\\begin{verbatim}\n\\label{eq:example}\n")],
    );
    let report = check_labels(&project.entry("main.tex"), LabelHygieneOptions::default()).unwrap();
    assert!(report.labels.is_empty());
    assert!(!report.coverage_gaps.is_empty());
    assert_eq!(report.status, "PARTIAL");
    assert_eq!(report.exit_code(), 2);
    project.assert_unchanged();
}

#[test]
fn typst_heading_warning_uses_visible_text_not_attached_label() {
    let project = Project::new(
        "heading_label",
        &[(
            "main.typ",
            "= Native comparison <sec:m3-example>\n= M3 comparison <sec:comparison>\n",
        )],
    );
    let report = check_labels(&project.entry("main.typ"), LabelHygieneOptions::default()).unwrap();
    let headings: Vec<_> = report
        .diagnostics
        .iter()
        .filter(|d| d.kind == "workflow_heading")
        .collect();
    let labels: Vec<_> = report
        .diagnostics
        .iter()
        .filter(|d| d.kind == "workflow_label")
        .collect();
    assert_eq!(headings.len(), 1, "{:?}", report.diagnostics);
    assert_eq!(headings[0].line, 2);
    assert_eq!(labels.len(), 1, "{:?}", report.diagnostics);
    assert_eq!(labels[0].line, 1);
    assert_eq!(report.exit_code(), 0);
    project.assert_unchanged();
}

#[test]
fn forward_reference_resolves_in_included_file_and_inputs_stay_unchanged() {
    let project = Project::new(
        "forward",
        &[
            (
                "main.tex",
                "See \\eqref{eq:balance}.\n\\input{sections/law}\n",
            ),
            ("sections/law.tex", "\\label{eq:balance}\nx=x\n"),
        ],
    );
    let report = check_labels(&project.entry("main.tex"), LabelHygieneOptions::default()).unwrap();
    assert!(report.diagnostics.is_empty(), "{:?}", report.diagnostics);
    assert_eq!(report.exit_code(), 0);
    assert_eq!(report.files.len(), 2);
    assert_eq!(report.labels.len(), 1);
    assert_eq!(report.labels[0].name, "eq:balance");
    let source = report
        .files
        .iter()
        .find(|f| f.path.ends_with("sections/law.tex"))
        .unwrap();
    let expected = format!("{:x}", Sha256::digest(b"\\label{eq:balance}\nx=x\n"));
    assert_eq!(source.sha256, expected);
    project.assert_unchanged();
}

#[test]
fn duplicate_labels_across_includes_report_both_locations() {
    let project = Project::new(
        "duplicate",
        &[
            (
                "main.tex",
                "\\input{sections/a}\n\\input{sections/b}\n\\eqref{eq:balance}\n",
            ),
            ("sections/a.tex", "First\n\\label{eq:balance}\n"),
            ("sections/b.tex", "Second\n\\label{eq:balance}\n"),
        ],
    );
    let report = check_labels(&project.entry("main.tex"), LabelHygieneOptions::default()).unwrap();
    let duplicate = report
        .diagnostics
        .iter()
        .find(|d| d.kind == "duplicate_label")
        .expect("duplicate definitions must not overwrite each other");
    assert_eq!(duplicate.severity, Severity::Error);
    assert_eq!(duplicate.line, 2);
    let mut locations = BTreeSet::from([duplicate.file.clone()]);
    locations.extend(duplicate.related.iter().map(|s| s.file.clone()));
    assert!(locations.iter().any(|p| p.ends_with("sections/a.tex")));
    assert!(locations.iter().any(|p| p.ends_with("sections/b.tex")));
    assert!(report
        .diagnostics
        .iter()
        .any(|d| d.kind == "ambiguous_reference"));
    assert_eq!(report.exit_code(), 1);
    project.assert_unchanged();
}

#[test]
fn repeated_input_is_not_mistaken_for_an_include_cycle_or_ignored() {
    let project = Project::new(
        "repeat",
        &[
            ("main.tex", "\\input{part}\n\\input{part}\n"),
            ("part.tex", "\\label{sec:part}\n"),
        ],
    );
    let report = check_labels(&project.entry("main.tex"), LabelHygieneOptions::default()).unwrap();
    assert!(report
        .diagnostics
        .iter()
        .any(|d| d.kind == "duplicate_label"));
    assert!(!report.diagnostics.iter().any(|d| d.kind == "include_cycle"));
}

#[test]
fn independent_document_drivers_have_separate_label_scopes() {
    let project = Project::new(
        "separate",
        &[
            ("main.tex", "\\label{sec:setup}\n\\ref{sec:setup}\n"),
            ("manual/main.tex", "\\label{sec:setup}\n\\ref{sec:setup}\n"),
        ],
    );
    for entry in ["main.tex", "manual/main.tex"] {
        let report = check_labels(&project.entry(entry), LabelHygieneOptions::default()).unwrap();
        assert_eq!(report.labels.len(), 1);
        assert!(!report
            .diagnostics
            .iter()
            .any(|d| d.kind == "duplicate_label"));
        assert_eq!(report.exit_code(), 0);
    }
}

#[test]
fn inactive_file_cannot_satisfy_a_rendered_reference() {
    let project = Project::new(
        "inactive",
        &[
            ("main.tex", "\\eqref{eq:orphan}\n"),
            ("archive/orphan.tex", "\\label{eq:orphan}\n"),
        ],
    );
    let report = check_labels(&project.entry("main.tex"), LabelHygieneOptions::default()).unwrap();
    assert!(report
        .diagnostics
        .iter()
        .any(|d| { d.kind == "unresolved_reference" && d.severity == Severity::Error }));
    assert!(report.labels.is_empty());
    assert!(!report.files.iter().any(|f| f.path.ends_with("orphan.tex")));
    assert_eq!(report.exit_code(), 1);
}

#[test]
fn latex_include_paths_resolve_from_the_driver_directory() {
    let project = Project::new(
        "tex_paths",
        &[
            (
                "manual/main.tex",
                "\\input{sections/first}\n\\ref{eq:last}\n",
            ),
            ("manual/sections/first.tex", "\\input{sections/second}\n"),
            ("manual/sections/second.tex", "\\label{eq:last}\n"),
        ],
    );
    let report = check_labels(
        &project.entry("manual/main.tex"),
        LabelHygieneOptions::default(),
    )
    .unwrap();
    assert_eq!(report.files.len(), 3);
    assert_eq!(report.exit_code(), 0, "{:?}", report.diagnostics);
}

#[test]
fn comments_verbatim_and_macro_definitions_do_not_create_live_labels() {
    let source = r"% \label{eq:ghost} \ref{eq:absent}
\begin{verbatim}
\label{eq:ghost}
\ref{eq:absent}
\end{verbatim}
\verb|\label{eq:ghost}|
\newcommand{\unused}{\label{eq:ghost}\ref{eq:absent}}
\label{eq:real}
\ref{eq:real}
";
    let project = Project::new("masked", &[("main.tex", source)]);
    let report = check_labels(&project.entry("main.tex"), LabelHygieneOptions::default()).unwrap();
    assert_eq!(
        report
            .labels
            .iter()
            .map(|x| x.name.as_str())
            .collect::<Vec<_>>(),
        vec!["eq:real"]
    );
    assert!(!report
        .diagnostics
        .iter()
        .any(|d| { matches!(d.kind.as_str(), "duplicate_label" | "unresolved_reference") }));
    assert!(
        !report.coverage_gaps.is_empty(),
        "macro expansion is not evaluated"
    );
    assert_eq!(report.exit_code(), 2);
    project.assert_unchanged();
}

#[test]
fn dynamic_and_conditional_labels_are_partial_not_fabricated_definitions() {
    let source = r"\label{\computedkey}
\ref{\computedkey}
\iffalse
\input{absent}
\label{eq:inactive}
\fi
\IfFileExists{also-absent.tex}{\input{also-absent}}{}
";
    let project = Project::new("dynamic", &[("main.tex", source)]);
    let report = check_labels(&project.entry("main.tex"), LabelHygieneOptions::default()).unwrap();
    assert!(!report.coverage_gaps.is_empty());
    assert!(report.labels.is_empty());
    assert!(!report
        .diagnostics
        .iter()
        .any(|d| d.kind == "missing_include"));
    assert_eq!(report.exit_code(), 2);
}

#[test]
fn latex_bibliography_citations_are_not_label_references() {
    let project = Project::new(
        "citations",
        &[(
            "main.tex",
            "\\cite{paper}\n\\citep[Theorem 2]{other}\n\\citet{third}\n",
        )],
    );
    let report = check_labels(&project.entry("main.tex"), LabelHygieneOptions::default()).unwrap();
    assert!(!report
        .diagnostics
        .iter()
        .any(|d| d.kind == "unresolved_reference"));
    assert_eq!(report.exit_code(), 0);
}

fn external_options(prefix: Option<&str>) -> LabelHygieneOptions {
    LabelHygieneOptions {
        external_labels: BTreeSet::from(["main:eq:balance".to_owned()]),
        main_reference_prefix: prefix.map(str::to_owned),
        ..LabelHygieneOptions::default()
    }
}

#[test]
fn aod_wrapper_preserves_its_explicit_external_prefix() {
    let project = Project::new(
        "aod_external",
        &[("main.tex", "\\mainequation{main:eq:balance}\n")],
    );
    let report = check_labels(&project.entry("main.tex"), external_options(None)).unwrap();
    assert!(!report
        .diagnostics
        .iter()
        .any(|d| d.kind == "unresolved_reference"));
    assert_eq!(report.exit_code(), 0);
}

#[test]
fn matter_wrapper_prefix_is_explicit_and_not_guessed() {
    let project = Project::new(
        "matter_external",
        &[("main.tex", "\\mainequation{eq:balance}\n")],
    );
    let missing = check_labels(&project.entry("main.tex"), external_options(None)).unwrap();
    assert!(missing
        .diagnostics
        .iter()
        .any(|d| d.kind == "unresolved_reference"));
    let bound = check_labels(&project.entry("main.tex"), external_options(Some("main:"))).unwrap();
    assert!(!bound
        .diagnostics
        .iter()
        .any(|d| d.kind == "unresolved_reference"));
    assert_eq!(bound.exit_code(), 0);
}

#[test]
fn configured_wrapper_does_not_hide_an_accidental_double_prefix() {
    let project = Project::new(
        "double_prefix",
        &[("main.tex", "\\mainequation{main:eq:balance}\n")],
    );
    let report = check_labels(&project.entry("main.tex"), external_options(Some("main:"))).unwrap();
    assert!(report
        .diagnostics
        .iter()
        .any(|d| d.kind == "unresolved_reference"));
    assert_eq!(report.exit_code(), 1);
}

#[test]
fn typst_unicode_references_and_relative_includes_do_not_panic() {
    let project = Project::new(
        "typst_unicode",
        &[
            ("main.typ", "= Résumé <sec:resume>\nÉquilibre @sec:resume — λλλλλλλλλλλλλλλλλλλλλλλλλλλλλλλ\n#include \"sections/first.typ\"\n"),
            ("sections/first.typ", "#include \"second.typ\"\nVoir @eq:balance.\n"),
            ("sections/second.typ", "$ x = x $ <eq:balance>\n"),
        ],
    );
    let report = check_labels(&project.entry("main.typ"), LabelHygieneOptions::default()).unwrap();
    assert_eq!(report.files.len(), 3);
    assert!(report.labels.iter().any(|x| x.name == "sec:resume"));
    assert!(report.labels.iter().any(|x| x.name == "eq:balance"));
    assert!(!report.diagnostics.iter().any(|d| {
        matches!(
            d.kind.as_str(),
            "unresolved_reference" | "unresolved_typst_reference"
        )
    }));
    assert_eq!(report.exit_code(), 0, "{:?}", report.diagnostics);
    project.assert_unchanged();
}

#[test]
fn unresolved_typst_citation_or_reference_is_not_called_a_broken_anchor() {
    let project = Project::new("typst_cite", &[("main.typ", "See @external-paper.\n")]);
    let report = check_labels(&project.entry("main.typ"), LabelHygieneOptions::default()).unwrap();
    let unknown = report
        .diagnostics
        .iter()
        .find(|d| d.kind == "unresolved_typst_reference")
        .unwrap();
    assert_eq!(unknown.severity, Severity::Warning);
    assert!(!report.coverage_gaps.is_empty());
    assert!(!report
        .diagnostics
        .iter()
        .any(|d| d.severity == Severity::Error));
    assert_eq!(report.exit_code(), 2);
}

#[test]
fn typst_comments_raw_examples_and_unexecuted_bindings_are_not_live_labels() {
    let source = "// <eq:ghost> @eq:absent\n/* <eq:ghost> */\n`<eq:ghost>`\n#let unused = [<eq:ghost> @eq:absent]\n= Real <sec:real>\n@sec:real\n";
    let project = Project::new("typst_masked", &[("main.typ", source)]);
    let report = check_labels(&project.entry("main.typ"), LabelHygieneOptions::default()).unwrap();
    assert_eq!(
        report
            .labels
            .iter()
            .map(|x| x.name.as_str())
            .collect::<Vec<_>>(),
        vec!["sec:real"]
    );
    assert!(!report.diagnostics.iter().any(|d| {
        matches!(
            d.kind.as_str(),
            "duplicate_label" | "unresolved_typst_reference"
        )
    }));
    assert!(!report.coverage_gaps.is_empty());
    assert_eq!(report.exit_code(), 2);
}

#[test]
fn workflow_markers_warn_without_flagging_m31_or_rewriting_input() {
    let source = "\\section{M3 comparisons}\n\\label{eq:m3-cells}\n\\label{sec:M2-awareness}\n\\label{eq:R02-replay}\n\\label{eq:M31-rotation}\n";
    let project = Project::new("names", &[("main.tex", source)]);
    let options = LabelHygieneOptions {
        additional_paths: vec![
            "manual/data/m3/input.json".into(),
            "scripts/aod_m3_temporal.py".into(),
            "manual2/sections/07-survival.tex".into(),
            "manual/data/M31/velocities.json".into(),
        ],
        ..LabelHygieneOptions::default()
    };
    let report = check_labels(&project.entry("main.tex"), options).unwrap();
    assert_eq!(
        report
            .diagnostics
            .iter()
            .filter(|d| d.kind == "workflow_label")
            .count(),
        3
    );
    assert_eq!(
        report
            .diagnostics
            .iter()
            .filter(|d| d.kind == "workflow_path")
            .count(),
        2
    );
    assert!(report
        .diagnostics
        .iter()
        .any(|d| d.kind == "workflow_heading"));
    assert!(report
        .diagnostics
        .iter()
        .all(|d| d.severity == Severity::Warning));
    assert_eq!(
        report.files.len(),
        1,
        "additional paths are names, not files to execute/read"
    );
    assert_eq!(report.exit_code(), 0);
    project.assert_unchanged();
}

#[test]
fn naming_warnings_can_be_disabled_without_disabling_duplicate_checks() {
    let project = Project::new(
        "no_names",
        &[(
            "main.tex",
            "\\section{M3 status}\n\\label{eq:m3-example}\n\\label{eq:m3-example}\n",
        )],
    );
    let options = LabelHygieneOptions {
        check_names: false,
        ..LabelHygieneOptions::default()
    };
    let report = check_labels(&project.entry("main.tex"), options).unwrap();
    assert!(!report
        .diagnostics
        .iter()
        .any(|d| d.kind.starts_with("workflow_")));
    assert!(report
        .diagnostics
        .iter()
        .any(|d| d.kind == "duplicate_label"));
    assert_eq!(report.exit_code(), 1);
}

#[test]
fn whitespace_is_diagnosed_without_merging_distinct_literal_label_keys() {
    let project = Project::new(
        "whitespace",
        &[(
            "main.tex",
            "\\label{eq:spaced}\n\\label{ eq:spaced }\n\\ref{ eq:spaced }\n",
        )],
    );
    let report = check_labels(&project.entry("main.tex"), LabelHygieneOptions::default()).unwrap();
    assert!(report
        .diagnostics
        .iter()
        .any(|d| d.kind == "label_whitespace"));
    assert!(!report
        .diagnostics
        .iter()
        .any(|d| d.kind == "duplicate_label"));
    assert!(report.labels.iter().any(|x| x.name == " eq:spaced "));
}

#[test]
fn empty_label_is_reported() {
    let project = Project::new("empty", &[("main.tex", "\\label{}\n")]);
    let report = check_labels(&project.entry("main.tex"), LabelHygieneOptions::default()).unwrap();
    assert!(report.diagnostics.iter().any(|d| d.kind == "empty_label"));
    assert_eq!(report.exit_code(), 1);
}

#[test]
fn missing_includes_and_include_cycles_are_reported() {
    let project = Project::new(
        "includes",
        &[("main.tex", "\\input{missing}\n\\input{main}\n")],
    );
    let report = check_labels(&project.entry("main.tex"), LabelHygieneOptions::default()).unwrap();
    assert!(report
        .diagnostics
        .iter()
        .any(|d| d.kind == "missing_include"));
    assert!(report.diagnostics.iter().any(|d| d.kind == "include_cycle"));
    assert_eq!(report.exit_code(), 1);
}
