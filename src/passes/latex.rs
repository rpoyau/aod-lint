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
        let lit_spans = compute_literature_note_spans(self.source_text);
        let mut byte_cursor = 0usize;

        for (idx, raw_line) in self.source_text.split('\n').enumerate() {
            let line_num = idx + 1;
            let line_trimmed_cr = raw_line.strip_suffix('\r').unwrap_or(raw_line);
            let code_line = strip_latex_comment(line_trimmed_cr);

            if !code_line.trim().is_empty() {
                for cap in self.cite_re.captures_iter(code_line) {
                    let full = cap.get(0).unwrap();
                    let cite_abs_offset = byte_cursor + full.start();
                    let in_lit_note = lit_spans
                        .iter()
                        .any(|&(s, e)| cite_abs_offset >= s && cite_abs_offset < e);
                    let has_anchor = cap.get(1).is_some();
                    let keys = &cap[2];

                    if !in_lit_note {
                        let msg = if has_anchor {
                            format!(
                                "Inline citation '{}' in body prose. General Mechanics style localizes external references inside \\aodliteraturenote{{...}} at the end of a section/block.",
                                full.as_str()
                            )
                        } else {
                            format!(
                                "Inline citation '\\cite{{{}}}' in body prose lacks a precise anchor (AF §7.2) and sits outside \\aodliteraturenote{{...}} (General Mechanics style).",
                                keys
                            )
                        };
                        self.diagnostics.push(Diagnostic {
                            severity: Severity::Warning,
                            rule: RuleAnchor::Sec7PreciseAnchors,
                            file: self.file_path.to_string(),
                            line: line_num,
                            column: full.start() + 1,
                            message: msg,
                            suggested_fix: Some(format!(
                                "Move to \\aodliteraturenote{{...~\\cite[Theorem/Table X, p. Y]{{{}}}}}.",
                                keys
                            )),
                            hoisted: false,
                        });
                    } else if !has_anchor {
                        self.diagnostics.push(Diagnostic {
                            severity: Severity::Info,
                            rule: RuleAnchor::Sec7PreciseAnchors,
                            file: self.file_path.to_string(),
                            line: line_num,
                            column: full.start() + 1,
                            message: format!(
                                "Literature-note citation '\\cite{{{}}}' has no bracketed clause/page anchor (AF §7.2).",
                                keys
                            ),
                            suggested_fix: Some(format!("\\cite[Sec./Table X]{{{}}}", keys)),
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
                        suggested_fix: Some(
                            "Replace range with exact \\eqnref{...} or \\secref{...} anchor.".into(),
                        ),
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
                            "Hoisted to Dependency Manifest. Move upfront to Section 2 (Modelling Choices) to resolve warning."
                                .into(),
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

            byte_cursor += raw_line.len() + 1;
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

fn compute_literature_note_spans(source: &str) -> Vec<(usize, usize)> {
    let marker = "\\aodliteraturenote";
    let mut spans = Vec::new();
    let mut search_from = 0usize;
    let bytes = source.as_bytes();

    while let Some(rel_idx) = source[search_from..].find(marker) {
        let start = search_from + rel_idx;
        let mut pos = start + marker.len();
        while pos < bytes.len() && bytes[pos].is_ascii_whitespace() {
            pos += 1;
        }
        if pos < bytes.len() && bytes[pos] == b'{' {
            let mut depth = 0usize;
            let mut i = pos;
            while i < bytes.len() {
                if bytes[i] == b'{' && (i == 0 || bytes[i - 1] != b'\\') {
                    depth += 1;
                } else if bytes[i] == b'}' && (i == 0 || bytes[i - 1] != b'\\') {
                    depth = depth.saturating_sub(1);
                    if depth == 0 {
                        spans.push((start, i + 1));
                        search_from = i + 1;
                        break;
                    }
                }
                i += 1;
            }
            if depth != 0 {
                spans.push((start, bytes.len()));
                break;
            }
        } else {
            search_from = pos;
        }
    }
    spans
}
