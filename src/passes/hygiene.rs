use regex::Regex;
use std::fs;
use std::path::{Path, PathBuf};

use crate::models::diagnostics::{Diagnostic, RuleAnchor, Severity};

const MAX_TEXT_SCAN_BYTES: u64 = 2 * 1024 * 1024;

pub struct RepoHygienePass {
    root: PathBuf,
    max_blob_bytes: u64,
    agent_dir_re: Regex,
    agent_prompt_file_re: Regex,
    agent_dump_file_re: Regex,
    archive_re: Regex,
    build_artifact_re: Regex,
    private_session_re: Regex,
    sandbox_path_re: Regex,
    private_repo_re: Regex,
    ticket_id_re: Regex,
}

impl RepoHygienePass {
    pub fn new<P: AsRef<Path>>(root: P, max_blob_bytes: u64) -> Self {
        Self {
            root: root.as_ref().to_path_buf(),
            max_blob_bytes,
            agent_dir_re: Regex::new(r"(?i)(^|/)\.(agents|codex|claude|gemini|cursor)(/|$)")
                .unwrap(),
            agent_prompt_file_re: Regex::new(
                r"(?i)(^|/)(AGENTS\.md|CLAUDE\.md|CODEX_PROMPT[^/]*|NEXT_CODEX_REQUEST[^/]*|START_NEW_THREAD[^/]*)$",
            )
            .unwrap(),
            agent_dump_file_re: Regex::new(
                r"(?i)(^|/)(historical_replay_review\.md|review-\d+\.md|.*acceptance.*\.(csv|json)|.*handoff.*\.(json|md)|original-archive-verification\.json|original-output\.json|original-source-bindings\.json|prior_[^/]+\.json|evidence/pre-[^/]+/.*|evidence/v\d+-request/.*|vendor/aod_[^/]+/.*)$",
            )
            .unwrap(),
            archive_re: Regex::new(r"(?i)\.(zip|tar|tgz|tar\.gz|jsonl\.gz|json\.gz)$").unwrap(),
            build_artifact_re: Regex::new(r"(?i)\.(aux|fls|fdb_latexmk|synctex\.gz)$").unwrap(),
            private_session_re: Regex::new(
                r#"(?i)(chatgpt\.com/library\?[^\s\)"']*|libfile_[a-z0-9_]+|account_id=personal)"#,
            )
            .unwrap(),
            sandbox_path_re: Regex::new(
                r#"(/workspace/scratch/[^\s\)"']*|/home/antigravity/[^\s\)"']*|/Users/[A-Za-z0-9_.-]+/[^\s\)"']*)"#,
            )
            .unwrap(),
            private_repo_re: Regex::new(
                r#"(?i)(github\.com/rpoyau/aod-skills[^\s\)"']*|aod-skills/blob/[^\s\)"']*)"#,
            )
            .unwrap(),
            ticket_id_re: Regex::new(r"\b(MAP-\d{2}|WP[0-7])\b").unwrap(),
        }
    }

    pub fn run(&self) -> Result<Vec<Diagnostic>, String> {
        let mut rel_files = Vec::new();
        collect_repo_files(&self.root, &self.root, &mut rel_files)?;
        rel_files.sort();

        let mut diagnostics = Vec::new();

        for (rel_path, byte_len) in rel_files {
            // 1. Agent config / prompt / skill directories checked into scientific repo
            if self.agent_dir_re.is_match(&rel_path) || self.agent_prompt_file_re.is_match(&rel_path)
            {
                diagnostics.push(Diagnostic {
                    severity: Severity::Fatal,
                    rule: RuleAnchor::RepoHygieneAgentLeakage,
                    file: rel_path.clone(),
                    line: 1,
                    column: 1,
                    message: format!(
                        "Agent configuration/prompt artifact '{}' is checked into the repository.",
                        rel_path
                    ),
                    suggested_fix: Some(
                        "Remove from scientific repository; keep skills and agent prompts in rpoyau/aod-skills.".into(),
                    ),
                    hoisted: false,
                });
                continue;
            }

            // 2. Agent review narratives, acceptance CSVs, handoff manifests, cross-repo snapshots
            if self.agent_dump_file_re.is_match(&rel_path) {
                diagnostics.push(Diagnostic {
                    severity: Severity::Error,
                    rule: RuleAnchor::RepoHygieneAgentLeakage,
                    file: rel_path.clone(),
                    line: 1,
                    column: 1,
                    message: format!(
                        "Agent review/acceptance/handoff state artifact '{}' leaked into scientific source tree.",
                        rel_path
                    ),
                    suggested_fix: Some(
                        "Relocate project review and acceptance history to rpoyau/aod-skills/projects/aod/.".into(),
                    ),
                    hoisted: false,
                });
            }

            // 3. Checked-in archives & compressed trace dumps (.zip, .json.gz, .jsonl.gz)
            if self.archive_re.is_match(&rel_path) {
                diagnostics.push(Diagnostic {
                    severity: Severity::Error,
                    rule: RuleAnchor::RepoHygieneAgentLeakage,
                    file: rel_path.clone(),
                    line: 1,
                    column: 1,
                    message: format!(
                        "Checked-in archive or compressed agent trace bundle '{}' ({:.2} MB).",
                        rel_path,
                        byte_len as f64 / (1024.0 * 1024.0)
                    ),
                    suggested_fix: Some(
                        "Keep zip handoffs and bulk execution archives in detached CI/release artifacts, not git.".into(),
                    ),
                    hoisted: false,
                });
            }

            // 4. Checked-in generated PDFs, external source PDFs, or LaTeX build intermediates
            let is_pdf = rel_path.to_ascii_lowercase().ends_with(".pdf");
            let is_figure_pdf = is_pdf
                && (rel_path.starts_with("figures/")
                    || rel_path.contains("/figures/"));
            if (is_pdf && !is_figure_pdf) || self.build_artifact_re.is_match(&rel_path) {
                diagnostics.push(Diagnostic {
                    severity: Severity::Error,
                    rule: RuleAnchor::RepoHygieneAgentLeakage,
                    file: rel_path.clone(),
                    line: 1,
                    column: 1,
                    message: format!(
                        "Checked-in PDF or build artifact '{}' ({:.2} MB) violates source-only repository hygiene.",
                        rel_path,
                        byte_len as f64 / (1024.0 * 1024.0)
                    ),
                    suggested_fix: Some(
                        "Remove generated/bundled PDF from git, add to .gitignore, and cite external papers via refs.bib + SHA-256 receipt if needed.".into(),
                    ),
                    hoisted: false,
                });
            }

            // 5. Oversized state dumps (excluding figure assets and already-flagged archives/PDFs)
            let is_figure_asset = rel_path.starts_with("figures/")
                || rel_path.starts_with("figures_jpg/")
                || rel_path.contains("/figures/");
            if byte_len > self.max_blob_bytes
                && !is_figure_asset
                && !self.archive_re.is_match(&rel_path)
                && !is_pdf
            {
                diagnostics.push(Diagnostic {
                    severity: Severity::Warning,
                    rule: RuleAnchor::RepoHygieneAgentLeakage,
                    file: rel_path.clone(),
                    line: 1,
                    column: 1,
                    message: format!(
                        "Oversized tracked state blob '{}' ({:.2} MB exceeds {:.2} MB limit).",
                        rel_path,
                        byte_len as f64 / (1024.0 * 1024.0),
                        self.max_blob_bytes as f64 / (1024.0 * 1024.0)
                    ),
                    suggested_fix: Some(
                        "Reconstruct large deterministic execution outputs via --check scripts instead of committing multi-MB JSON dumps.".into(),
                    ),
                    hoisted: false,
                });
            }

            // 6. Content scan on text files up to MAX_TEXT_SCAN_BYTES
            if byte_len <= MAX_TEXT_SCAN_BYTES && is_scannable_text_file(&rel_path) {
                let full_path = self.root.join(&rel_path);
                if let Ok(bytes) = fs::read(&full_path) {
                    if let Ok(content) = std::str::from_utf8(&bytes) {
                        self.scan_text_contents(&rel_path, content, &mut diagnostics);
                    }
                }
            }
        }

        Ok(diagnostics)
    }

    fn scan_text_contents(
        &self,
        rel_path: &str,
        content: &str,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        let check_tickets = rel_path.ends_with(".tex")
            || rel_path.ends_with(".typ")
            || rel_path == "BUILD.md"
            || rel_path.starts_with("contracts/scientific/");

        for (idx, line) in content.lines().enumerate() {
            let line_num = idx + 1;

            if let Some(mat) = self.private_session_re.find(line) {
                diagnostics.push(Diagnostic {
                    severity: Severity::Fatal,
                    rule: RuleAnchor::RepoHygieneAgentLeakage,
                    file: rel_path.to_string(),
                    line: line_num,
                    column: mat.start() + 1,
                    message: format!(
                        "Private agent session / personal library identifier '{}' leaked into tracked file.",
                        mat.as_str()
                    ),
                    suggested_fix: Some(
                        "Purge private ChatGPT/agent session URLs and libfile IDs immediately.".into(),
                    ),
                    hoisted: false,
                });
            }

            if let Some(mat) = self.sandbox_path_re.find(line) {
                diagnostics.push(Diagnostic {
                    severity: Severity::Error,
                    rule: RuleAnchor::RepoHygieneAgentLeakage,
                    file: rel_path.to_string(),
                    line: line_num,
                    column: mat.start() + 1,
                    message: format!(
                        "Ephemeral agent container/host path '{}' leaked into tracked file.",
                        mat.as_str()
                    ),
                    suggested_fix: Some(
                        "Use repository-relative paths only; strip container scratch paths.".into(),
                    ),
                    hoisted: false,
                });
            }

            if let Some(mat) = self.private_repo_re.find(line) {
                diagnostics.push(Diagnostic {
                    severity: Severity::Error,
                    rule: RuleAnchor::RepoHygieneAgentLeakage,
                    file: rel_path.to_string(),
                    line: line_num,
                    column: mat.start() + 1,
                    message: format!(
                        "Private planning repository reference '{}' leaked into public scientific source.",
                        mat.as_str()
                    ),
                    suggested_fix: Some(
                        "Keep rpoyau/aod-skills planning references out of public scientific repositories.".into(),
                    ),
                    hoisted: false,
                });
            }

            if check_tickets {
                if let Some(mat) = self.ticket_id_re.find(line) {
                    diagnostics.push(Diagnostic {
                        severity: Severity::Warning,
                        rule: RuleAnchor::RepoHygieneAgentLeakage,
                        file: rel_path.to_string(),
                        line: line_num,
                        column: mat.start() + 1,
                        message: format!(
                            "Work-package / acceptance tracker code '{}' leaked into scientific manuscript or contract.",
                            mat.as_str()
                        ),
                        suggested_fix: Some(
                            "Replace internal milestone ticket IDs (MAP-xx / WPx) with self-contained mathematical names.".into(),
                        ),
                        hoisted: false,
                    });
                }
            }
        }
    }
}

fn collect_repo_files(
    base: &Path,
    dir: &Path,
    out: &mut Vec<(String, u64)>,
) -> Result<(), String> {
    let entries = fs::read_dir(dir)
        .map_err(|e| format!("Failed to read directory {}: {}", dir.display(), e))?;
    for entry in entries {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();
        if name == ".git" || name == "target" || name == "__pycache__" || name == ".pytest_cache" {
            continue;
        }
        let meta = entry.metadata().map_err(|e| e.to_string())?;
        if meta.is_dir() {
            collect_repo_files(base, &path, out)?;
        } else if meta.is_file() {
            let rel = path
                .strip_prefix(base)
                .unwrap_or(&path)
                .to_string_lossy()
                .replace('\\', "/");
            out.push((rel, meta.len()));
        }
    }
    Ok(())
}

fn is_scannable_text_file(rel_path: &str) -> bool {
    let lower = rel_path.to_ascii_lowercase();
    [
        ".tex", ".typ", ".md", ".json", ".csv", ".txt", ".py", ".rs", ".toml", ".bib", ".yaml",
        ".yml",
    ]
    .iter()
    .any(|ext| lower.ends_with(ext))
}
