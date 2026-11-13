use regex::Regex;
use typst_syntax::{parse, SyntaxKind, SyntaxNode};
use crate::models::diagnostics::{Diagnostic, RuleAnchor, Severity};

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
        self.traverse(&root, 0);
        self.diagnostics
    }

    fn traverse(&mut self, node: &SyntaxNode, offset: usize) {
        let (line, col) = self.offset_to_line_col(offset);

        if node.kind() == SyntaxKind::Ref {
            let text = node.text();
            let next_slice_end = (offset + text.len() + 24).min(self.source_text.len());
            let following = &self.source_text[offset + text.len()..next_slice_end];

            if !following.trim_start().starts_with('[') {
                self.diagnostics.push(Diagnostic {
                    severity: Severity::Warning,
                    rule: RuleAnchor::Sec7PreciseAnchors,
                    file: self.file_path.to_string(),
                    line,
                    column: col,
                    message: format!(
                        "Citation '{}' lacks a precise anchor. Please honor the academic social contract & AF §7.2 by specifying the exact theorem, section, or page.",
                        text
                    ),
                    suggested_fix: Some(format!("{}[Theorem X, p. Y]", text)),
                    hoisted: false,
                });
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
                    message: format!("Broad reference range '{}'. AF §7.2 requires pointing to a single precise anchor.", mat.as_str()),
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
                    suggested_fix: Some("Hoisted to Dependency Manifest. Move upfront to Section 2 (Modelling Choices) to resolve warning.".into()),
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
            self.traverse(child, child_offset);
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
