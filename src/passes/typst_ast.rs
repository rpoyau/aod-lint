use crate::models::diagnostics::{Diagnostic, RuleAnchor, Severity};
use regex::Regex;
use std::collections::HashSet;
use typst_syntax::{ast, parse, SyntaxKind, SyntaxNode};

pub struct TypstAstPass<'a> {
    file_path: &'a str,
    source_text: &'a str,
    strict_mode: bool,
    diagnostics: Vec<Diagnostic>,
    vague_range_re: Regex,
    midstream_import_re: Regex,
    manifold_import_re: Regex,
    local_labels: HashSet<String>,
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
            local_labels: HashSet::new(),
        }
    }

    pub fn run(mut self) -> Vec<Diagnostic> {
        let root: SyntaxNode = parse(self.source_text);
        collect_labels(&root, &mut self.local_labels);
        self.traverse(&root, 0);
        self.diagnostics
    }

    fn traverse(&mut self, node: &SyntaxNode, offset: usize) {
        let (line, col) = self.offset_to_line_col(offset);

        if let Some(reference) = node.cast::<ast::Ref>() {
            let target = reference.target();
            if reference.supplement().is_none() && !self.local_labels.contains(target) {
                self.diagnostics.push(Diagnostic {
                    severity: Severity::Warning,
                    rule: RuleAnchor::Sec7PreciseAnchors,
                    file: self.file_path.to_string(),
                    line,
                    column: col,
                    message: format!(
                        "Reference '@{}' has no local literal label or supplement. It may be an imported label or a bibliography key; verify that context before adding a citation locator.",
                        target
                    ),
                    suggested_fix: Some(
                        "Check the document's imports and bibliography. For a bibliography citation, supply the relevant theorem, section, or page; a resolved label needs no redundant locator.".into()
                    ),
                    dependency_candidate: false,
                });
            }
        }

        if node.kind() == SyntaxKind::Text {
            let text = node.text();

            if let Some(mat) = self.vague_range_re.find(text) {
                let (match_line, match_col) = self.offset_to_line_col(offset + mat.start());
                self.diagnostics.push(Diagnostic {
                    severity: Severity::Warning,
                    rule: RuleAnchor::Sec7PreciseAnchors,
                    file: self.file_path.to_string(),
                    line: match_line,
                    column: match_col,
                    message: format!("Broad reference range '{}'. AF §7.2 requires pointing to a single precise anchor.", mat.as_str()),
                    suggested_fix: Some("Replace range with exact label anchor (e.g., @eq:stokes).".into()),
                    dependency_candidate: false,
                });
            }

            if let Some(mat) = self.midstream_import_re.find(text) {
                let sev = if self.strict_mode {
                    Severity::Error
                } else {
                    Severity::Warning
                };
                let (match_line, match_col) = self.offset_to_line_col(offset + mat.start());
                self.diagnostics.push(Diagnostic {
                    severity: sev,
                    rule: RuleAnchor::AF4NoMidstreamImports,
                    file: self.file_path.to_string(),
                    line: match_line,
                    column: match_col,
                    message: format!("Possible premise dependency '{}'; inspect its declaration and scope.", mat.as_str()),
                    suggested_fix: Some("Check whether this premise is already declared at its owning scope. This lexical scan does not reconstruct or hoist dependencies.".into()),
                    dependency_candidate: true,
                });
            }

            if let Some(mat) = self.manifold_import_re.find(text) {
                let sev = if self.strict_mode {
                    Severity::Error
                } else {
                    Severity::Warning
                };
                let (match_line, match_col) = self.offset_to_line_col(offset + mat.start());
                self.diagnostics.push(Diagnostic {
                    severity: sev,
                    rule: RuleAnchor::RF3NoAbsolutes,
                    file: self.file_path.to_string(),
                    line: match_line,
                    column: match_col,
                    message: format!(
                        "Representation term '{}' requires its declared domain and hypotheses to be checked; the term alone does not establish an undeclared import.",
                        mat.as_str()
                    ),
                    suggested_fix: Some("Review the actual model and source dependencies. Do not replace it with an AFC model without a separately declared construction.".into()),
                    dependency_candidate: true,
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
        // AST offsets and regex match boundaries are valid UTF-8 boundaries.
        let prefix = &self.source_text[..offset];
        let line = prefix.bytes().filter(|&b| b == b'\n').count() + 1;
        let col = prefix.rsplit('\n').next().unwrap_or("").chars().count() + 1;
        (line, col)
    }
}

fn collect_labels(node: &SyntaxNode, labels: &mut HashSet<String>) {
    if matches!(
        node.kind(),
        SyntaxKind::Conditional
            | SyntaxKind::LetBinding
            | SyntaxKind::Closure
            | SyntaxKind::ForLoop
            | SyntaxKind::WhileLoop
            | SyntaxKind::FuncCall
            | SyntaxKind::CodeBlock
            | SyntaxKind::Contextual
            | SyntaxKind::ShowRule
            | SyntaxKind::SetRule
    ) {
        return;
    }
    if let Some(label) = node.cast::<ast::Label>() {
        labels.insert(label.get().to_string());
    }
    for child in node.children() {
        collect_labels(child, labels);
    }
}
