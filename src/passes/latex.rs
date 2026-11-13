use regex::Regex;

use crate::models::diagnostics::{Diagnostic, RuleAnchor, Severity};

pub struct LatexPass<'a> {
    file_path: &'a str,
    source_text: &'a str,
    strict_mode: bool,
    diagnostics: Vec<Diagnostic>,
    cite_re: Regex,
    vague_range_re: Regex,
    midstream_import_re: Regex,
    manifold_import_re: Regex,
}

impl<'a> LatexPass<'a> {
    pub fn new(file_path: &'a str, source_text: &'a str, strict_mode: bool) -> Self {
        Self {
            file_path,
            source_text,
            strict_mode,
            diagnostics: Vec::new(),
            cite_re: Regex::new(r"\\cite(\[[^\]]*\])?\{([^}]+)\}").unwrap(),
            vague_range_re: Regex::new(
                r"(?i)\b(see|refer to|in)\s+(§§?\s*\d+\s*[-–—]\s*\d+|sections?\s+\d+\s*[-–—]\s*\d+|the literature)\b",
            )
            .unwrap(),
            midstream_import_re: Regex::new(
                r"(?i)\b(by|via|using)\s+([A-Z][a-zA-Z0-9_\-\s]+(law|equation|theorem|principle))\b",
            )
            .unwrap(),
            manifold_import_re: Regex::new(
                r"(?i)\b(smooth\s+manifold|continuous\s+manifold|hilbert\s+space)\b",
            )
            .unwrap(),
        }
    }

    pub fn run(mut self) -> Vec<Diagnostic> {
        for (idx, raw_line) in self.source_text.lines().enumerate() {
            let line_num = idx + 1;
            let code_line = strip_latex_comment(raw_line);
            if code_line.trim().is_empty() {
                continue;
            }

            for cap in self.cite_re.captures_iter(code_line) {
                if cap.get(1).is_none() {
                    let full = cap.get(0).unwrap();
                    let keys = &cap[2];
                    self.diagnostics.push(Diagnostic {
                        severity: Severity::Warning,
                        rule: RuleAnchor::Sec7PreciseAnchors,
                        file: self.file_path.to_string(),
                        line: line_num,
                        column: full.start() + 1,
                        message: format!(
                            "Citation '\\cite{{{}}}' lacks a precise anchor. Please honor the academic social contract & AF §7.2 by specifying the exact theorem, section, or page.",
                            keys
                        ),
                        suggested_fix: Some(format!("\\cite[Theorem X, p. Y]{{{}}}", keys)),
                        hoisted: false,
                    });
                }
            }

            if let Some(mat) = self.vague_range_re.find(code_line) {
                self.diagnostics.push(Diagnostic {
                    severity: Severity::Warning,
                    rule: RuleAnchor::Sec7PreciseAnchors,
                    file: self.file_path.to_string(),
                    line: line_num,
                    column: mat.start() + 1,
                    message: format!(
                        "Broad reference range '{}'. AF §7.2 requires pointing to a single precise anchor.",
                        mat.as_str()
                    ),
                    suggested_fix: Some("Replace range with exact \\eqnref{...} or \\secref{...} anchor.".into()),
                    hoisted: false,
                });
            }

            if let Some(mat) = self.midstream_import_re.find(code_line) {
                let sev = if self.strict_mode {
                    Severity::Error
                } else {
                    Severity::Warning
                };
                self.diagnostics.push(Diagnostic {
                    severity: sev,
                    rule: RuleAnchor::AF4NoMidstreamImports,
                    file: self.file_path.to_string(),
                    line: line_num,
                    column: mat.start() + 1,
                    message: format!("Midstream premise import '{}' detected.", mat.as_str()),
                    suggested_fix: Some(
                        "Hoisted to Dependency Manifest. Move upfront to Section 2 (Modelling Choices) to resolve warning.".into(),
                    ),
                    hoisted: !self.strict_mode,
                });
            }

            if let Some(mat) = self.manifold_import_re.find(code_line) {
                let sev = if self.strict_mode {
                    Severity::Error
                } else {
                    Severity::Warning
                };
                self.diagnostics.push(Diagnostic {
                    severity: sev,
                    rule: RuleAnchor::RF3NoAbsolutes,
                    file: self.file_path.to_string(),
                    line: line_num,
                    column: mat.start() + 1,
                    message: format!(
                        "CHECK YOUR IMPORTS: Heavy ontological import '{}' carries uncountable ghost states (RF2) and UV singularity liabilities.",
                        mat.as_str()
                    ),
                    suggested_fix: Some(
                        "Verify if a finite distinction set V with AFC Discrete Stokes suffices.".into(),
                    ),
                    hoisted: !self.strict_mode,
                });
            }
        }

        self.diagnostics
    }
}

pub fn strip_latex_comment(line: &str) -> &str {
    let bytes = line.as_bytes();
    for (i, &b) in bytes.iter().enumerate() {
        if b == b'%' && (i == 0 || bytes[i - 1] != b'\\') {
            return &line[..i];
        }
    }
    line
}
