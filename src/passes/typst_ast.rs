use regex::Regex;
use typst_syntax::{parse, SyntaxKind, SyntaxNode};
use crate::models::diagnostics::{Diagnostic, RuleAnchor, Severity};

const INTERNAL_LABEL_PREFIXES: &[&str] = &[
    "ax:", "ax-",
    "def:", "def-",
    "ch:", "ch-", "choice:", "choice-",
    "con:", "con-",
    "cor:", "cor-",
    "eq:", "eq-",
    "sec:", "sec-",
    "app:", "app-",
    "tab:", "tab-",
    "fig:", "fig-",
    "motif:", "motif-",
    "rule:", "rule-",
    "thm:", "thm-",
    "lem:", "lem-",
    "prop:", "prop-",
    "main:", "main-",
];

pub struct TypstAstPass<'a> {
    file_path: &'a str,
    source_text: &'a str,
    strict_mode: bool,
    diagnostics: Vec<Diagnostic>,
    vague_range_re: Regex,
    midstream_import_re: Regex,
    manifold_import_re: Regex,
}

impl<'a> TypstAstPass<'a> {
    pub fn new(file_path: &'a str, source_text: &'a str, strict_mode: bool) -> Self {
        Self {
            file_path,
            source_text,
            strict_mode,
            diagnostics: Vec::new(),
            vague_range_re: Regex::new(
                r"(?i)\b(see|refer to|in)\s+(§§?\s*\d+\s*[-–—]\s*\d+|sections?\s+\d+\s*[-–—]\s*\d+|the literature)\b"
            ).unwrap(),
            midstream_import_re: Regex::new(
                r"(?i)\b(by|via|using)\s+([A-Z][a-zA-Z0-9_\-\s]+(law|equation|theorem|principle))\b"
            ).unwrap(),
            manifold_import_re: Regex::new(
                r"(?i)\b(smooth\s+manifold|continuous\s+manifold|hilbert\s+space)\b"
            ).unwrap(),
        }
    }

    pub fn run(mut self) -> Vec<Diagnostic> {
        let root: SyntaxNode = parse(self.source_text);
        self.traverse(&root, 0, false);
        self.diagnostics
    }

    fn traverse(&mut self, node: &SyntaxNode, offset: usize, in_lit_note: bool) {
        if node.kind() == SyntaxKind::Raw {
            return;
        }
        let (line, col) = self.offset_to_line_col(offset);
        let now_in_lit_note = in_lit_note || is_literature_note_call(node);

        if node.kind() == SyntaxKind::Ref {
            let raw = node.clone().into_text();
            let has_supplement = raw.contains('[');
            let target = raw
                .split('[')
                .next()
                .unwrap_or("")
                .trim()
                .trim_start_matches('@');

            if !target.is_empty() && !is_internal_label(target) {
                if !now_in_lit_note {
                    let msg = if has_supplement {
                        format!(
                            "Inline bibliography citation '@{}' disrupts derivation flow; General Mechanics style isolates external citations in #literature-note[...].",
                            target
                        )
                    } else {
                        format!(
                            "Inline citation '@{}' lacks a precise anchor and sits outside #literature-note[...]. Honor AF §7.2 by citing exact section/theorem inside #literature-note[...].",
                            target
                        )
                    };
                    self.diagnostics.push(Diagnostic {
                        severity: Severity::Warning,
                        rule: RuleAnchor::Sec7PreciseAnchors,
                        file: self.file_path.to_string(),
                        line,
                        column: col,
                        message: msg,
                        suggested_fix: Some(format!(
                            "#literature-note[Topic: @{}[§X, p. Y].]",
                            target
                        )),
                        hoisted: false,
                    });
                } else if !has_supplement {
                    self.diagnostics.push(Diagnostic {
                        severity: Severity::Info,
                        rule: RuleAnchor::Sec7PreciseAnchors,
                        file: self.file_path.to_string(),
                        line,
                        column: col,
                        message: format!(
                            "Literature-note citation '@{}' lacks an explicit [§/Theorem/p.] anchor (AF §7.2).",
                            target
                        ),
                        suggested_fix: Some(format!("@{}[§X, p. Y]", target)),
                        hoisted: false,
                    });
                }
            }
        }

        if node.kind() == SyntaxKind::Text {
            let text = node.text();

            if let Some(mat) = self.vague_range_re.find(text) {
                self.diagnostics.push(Diagnostic {
                    severity: Severity::Warning,
                    rule: RuleAnchor::Sec7PreciseAnchors,
                    file: self.file_path.to_string(),
                    line,
                    column: col + mat.start(),
                    message: format!(
                        "Broad reference range '{}'. AF §7.2 requires pointing to a single precise anchor.",
                        mat.as_str()
                    ),
                    suggested_fix: Some("Replace range with exact label anchor (e.g., @eq:stokes).".into()),
                    hoisted: false,
                });
            }

            if let Some(mat) = self.midstream_import_re.find(text) {
                let sev = if self.strict_mode { Severity::Error } else { Severity::Warning };
                self.diagnostics.push(Diagnostic {
                    severity: sev,
                    rule: RuleAnchor::AF4NoMidstreamImports,
                    file: self.file_path.to_string(),
                    line,
                    column: col + mat.start(),
                    message: format!("Midstream premise import '{}' detected.", mat.as_str()),
                    suggested_fix: Some("Hoisted to Dependency Manifest. Move upfront to Modelling Choices to resolve.".into()),
                    hoisted: !self.strict_mode,
                });
            }

            if let Some(mat) = self.manifold_import_re.find(text) {
                let sev = if self.strict_mode { Severity::Error } else { Severity::Warning };
                self.diagnostics.push(Diagnostic {
                    severity: sev,
                    rule: RuleAnchor::RF3NoAbsolutes,
                    file: self.file_path.to_string(),
                    line,
                    column: col + mat.start(),
                    message: format!(
                        "CHECK YOUR IMPORTS: Heavy ontological import '{}' carries uncountable ghost states (RF2) and UV singularity liabilities.",
                        mat.as_str()
                    ),
                    suggested_fix: Some("Verify if a finite distinction set V with AFC Discrete Stokes suffices.".into()),
                    hoisted: !self.strict_mode,
                });
            }
        }

        let mut child_offset = offset;
        for child in node.children() {
            self.traverse(child, child_offset, now_in_lit_note);
            child_offset += child.len();
        }
    }

    fn offset_to_line_col(&self, offset: usize) -> (usize, usize) {
        let prefix = &self.source_text[..offset.min(self.source_text.len())];
        let line = prefix.lines().count().max(1);
        let col = prefix.lines().last().map(|l| l.len() + 1).unwrap_or(1);
        (line, col)
    }
}

fn is_internal_label(target: &str) -> bool {
    INTERNAL_LABEL_PREFIXES
        .iter()
        .any(|prefix| target.starts_with(prefix))
}

fn is_literature_note_call(node: &SyntaxNode) -> bool {
    if node.kind() != SyntaxKind::FuncCall {
        return false;
    }
    if let Some(first) = node.children().next() {
        let ident = first.clone().into_text();
        let trimmed = ident.trim();
        return trimmed == "literature-note" || trimmed == "aodliteraturenote";
    }
    false
}
