//! Read-only, driver-scoped checks of literal labels and their references.
//! This is a static diagnostic, not TeX/Typst compilation or an AF verdict.
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use regex::Regex;
use serde::Serialize;
use sha2::{Digest, Sha256};
use typst_syntax::{parse, SyntaxKind, SyntaxNode};

use crate::models::diagnostics::{RuleAnchor, Severity};

#[derive(Debug, Clone)]
pub struct LabelHygieneOptions {
    pub external_labels: BTreeSet<String>,
    /// Literal prefix added by the selected document's Main-reference wrappers.
    /// None preserves arguments, as in AOD; Some("main:") matches Matter I.
    pub main_reference_prefix: Option<String>,
    /// Explicit repository-relative paths; inspected as names, never executed.
    pub additional_paths: Vec<String>,
    pub check_names: bool,
}

impl Default for LabelHygieneOptions {
    fn default() -> Self {
        Self {
            external_labels: BTreeSet::new(),
            main_reference_prefix: None,
            additional_paths: Vec::new(),
            check_names: true,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct SourceLocation {
    pub file: String,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct LabelDefinition {
    pub name: String,
    #[serde(flatten)]
    pub location: SourceLocation,
}

#[derive(Debug, Clone, Serialize)]
pub struct SourceFile {
    pub path: String,
    pub sha256: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct CoverageGap {
    pub file: String,
    pub line: usize,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct HygieneDiagnostic {
    pub kind: String,
    pub severity: Severity,
    pub rule: RuleAnchor,
    pub file: String,
    pub line: usize,
    pub column: usize,
    pub message: String,
    pub related: Vec<SourceLocation>,
}

#[derive(Debug, Clone, Serialize)]
pub struct LabelHygieneReport {
    pub schema_version: &'static str,
    pub entry: String,
    pub status: String,
    pub files: Vec<SourceFile>,
    pub labels: Vec<LabelDefinition>,
    pub diagnostics: Vec<HygieneDiagnostic>,
    pub coverage_gaps: Vec<CoverageGap>,
    pub name_checks_enabled: bool,
    pub external_label_count: usize,
    pub main_reference_prefix: Option<String>,
    pub limitations: Vec<&'static str>,
}

impl LabelHygieneReport {
    pub fn exit_code(&self) -> i32 {
        if self
            .diagnostics
            .iter()
            .any(|d| matches!(d.severity, Severity::Error | Severity::Fatal))
        {
            1
        } else if !self.coverage_gaps.is_empty() {
            2
        } else {
            0
        }
    }
}

struct Reference {
    target: String,
    location: SourceLocation,
    typst: bool,
}

#[derive(Clone, Copy)]
enum ItemKind {
    Label,
    Reference,
    MainReference,
    Include,
    SelectedInclude,
    IncludeSelection,
    Heading,
    Gap,
}
struct Item {
    kind: ItemKind,
    value: String,
    offset: usize,
}

fn location(file: &Path, source: &str, offset: usize) -> SourceLocation {
    let prefix = &source[..offset];
    SourceLocation {
        file: file.to_string_lossy().into_owned(),
        line: prefix.bytes().filter(|&b| b == b'\n').count() + 1,
        column: prefix.rsplit('\n').next().unwrap_or("").chars().count() + 1,
    }
}

fn issue(
    kind: &str,
    severity: Severity,
    rule: RuleAnchor,
    at: &SourceLocation,
    message: String,
) -> HygieneDiagnostic {
    HygieneDiagnostic {
        kind: kind.into(),
        severity,
        rule,
        file: at.file.clone(),
        line: at.line,
        column: at.column,
        message,
        related: Vec::new(),
    }
}

fn markers(text: &str) -> Vec<&str> {
    text.split(|c: char| !c.is_alphanumeric())
        .filter(|token| {
            let lower = token.to_ascii_lowercase();
            matches!(lower.as_str(), "m1" | "m2" | "m3")
                || (lower.len() == 3
                    && lower.starts_with('r')
                    && lower.as_bytes()[1..].iter().all(u8::is_ascii_digit))
        })
        .collect()
}

fn escaped(text: &str, offset: usize) -> bool {
    text.as_bytes()[..offset]
        .iter()
        .rev()
        .take_while(|&&b| b == b'\\')
        .count()
        % 2
        == 1
}

fn blank(bytes: &mut [u8], start: usize, end: usize) {
    for b in &mut bytes[start..end] {
        if *b != b'\n' && *b != b'\r' {
            *b = b' ';
        }
    }
}

fn group_end(text: &str, start: usize, open: u8, close: u8) -> Option<usize> {
    if text.as_bytes().get(start) != Some(&open) {
        return None;
    }
    let mut depth = 0;
    for (i, &b) in text.as_bytes().iter().enumerate().skip(start) {
        if escaped(text, i) {
            continue;
        }
        if b == open {
            depth += 1;
        }
        if b == close {
            depth -= 1;
            if depth == 0 {
                return Some(i + 1);
            }
        }
    }
    None
}

fn skip_space(text: &str, mut i: usize) -> usize {
    while let Some(c) = text[i..].chars().next() {
        if !c.is_whitespace() {
            break;
        }
        i += c.len_utf8();
    }
    i
}

fn tex_items(source: &str) -> Vec<Item> {
    let mut bytes = source.as_bytes().to_vec();
    let mut items = Vec::new();
    // Mask in source order: a fake environment opener in a comment or \verb
    // must not hide later active source. Byte offsets and newlines stay intact.
    let environments = Regex::new(r"^\\begin\{(verbatim\*?|lstlisting|minted)\}").unwrap();
    let inline = Regex::new(r"^\\verb\*?([^\w\s])").unwrap();
    let mut i = 0;
    while i < source.len() {
        let tail = &source[i..];
        let mut end = None;
        if tail.starts_with('%') && !escaped(source, i) {
            end = Some(i + tail.find('\n').unwrap_or(tail.len()));
        } else if tail.starts_with('\\') && !escaped(source, i) {
            if let Some(cap) = inline.captures(tail) {
                let body = i + cap.get(0).unwrap().end();
                let line_end = body + source[body..].find('\n').unwrap_or(source.len() - body);
                end = Some(match source[body..line_end].find(&cap[1]) {
                    Some(n) => body + n + cap[1].len(),
                    None => {
                        items.push(Item {
                            kind: ItemKind::Gap,
                            value: "Unclosed inline verbatim command.".into(),
                            offset: i,
                        });
                        line_end
                    }
                });
            } else if let Some(cap) = environments.captures(tail) {
                let body = i + cap.get(0).unwrap().end();
                let closing = format!("\\end{{{}}}", &cap[1]);
                end = Some(match source[body..].find(&closing) {
                    Some(n) => body + n + closing.len(),
                    None => {
                        items.push(Item {
                            kind: ItemKind::Gap,
                            value: "Unclosed verbatim environment.".into(),
                            offset: i,
                        });
                        source.len()
                    }
                });
            }
        }
        if let Some(end) = end {
            blank(&mut bytes, i, end);
            i = end;
        } else {
            i += tail.chars().next().unwrap().len_utf8();
        }
    }
    // Definitions are not invocations. Do not invent labels from macro bodies.
    let definitions = Regex::new(r"\\(newcommand|renewcommand|providecommand|DeclareRobustCommand|newenvironment|renewenvironment|(?:New|Renew|Provide|Declare)(?:Expandable)?Document(?:Command|Environment)|def|gdef|edef|xdef)\b\*?").unwrap();
    let masked = String::from_utf8(bytes.clone()).unwrap();
    let mut consumed = 0;
    for cap in definitions.captures_iter(&masked) {
        let m = cap.get(0).unwrap();
        if m.start() < consumed || escaped(&masked, m.start()) {
            continue;
        }
        let command = &cap[1];
        let mut pos = skip_space(&masked, m.end());
        if matches!(command, "def" | "gdef" | "edef" | "xdef") {
            pos = masked[pos..]
                .find('{')
                .map(|n| pos + n)
                .unwrap_or(masked.len());
        } else {
            if masked.as_bytes().get(pos) == Some(&b'{') {
                pos = group_end(&masked, pos, b'{', b'}').unwrap_or(masked.len());
            } else {
                if masked.as_bytes().get(pos) == Some(&b'\\') {
                    pos += 1;
                }
                while masked
                    .as_bytes()
                    .get(pos)
                    .is_some_and(|b| b.is_ascii_alphabetic() || *b == b'@')
                {
                    pos += 1;
                }
            }
            loop {
                pos = skip_space(&masked, pos);
                if masked.as_bytes().get(pos) != Some(&b'[') {
                    break;
                }
                pos = group_end(&masked, pos, b'[', b']').unwrap_or(masked.len());
            }
        }
        if command.contains("Document") {
            // xparse-style definitions have a mandatory argument specification.
            pos = group_end(&masked, pos, b'{', b'}').unwrap_or(masked.len());
            pos = skip_space(&masked, pos);
        }
        let mut end = group_end(&masked, pos, b'{', b'}').unwrap_or(masked.len());
        if command.to_ascii_lowercase().ends_with("environment") {
            end = group_end(&masked, skip_space(&masked, end), b'{', b'}').unwrap_or(masked.len());
        }
        items.push(Item {
            kind: ItemKind::Gap,
            value: "Macro definitions are masked; expansion is not evaluated.".into(),
            offset: m.start(),
        });
        blank(&mut bytes, m.start(), end);
        consumed = end;
    }
    // Conditional content cannot supply a proved active anchor in a static scan.
    let masked = String::from_utf8(bytes.clone()).unwrap();
    let conditions = Regex::new(r"\\(if[A-Za-z@]*|else|fi)\b").unwrap();
    let mut stack = Vec::new();
    for cap in conditions.captures_iter(&masked) {
        let m = cap.get(0).unwrap();
        if escaped(&masked, m.start()) {
            continue;
        }
        if cap[1].starts_with("if") {
            stack.push(m.start());
        } else if &cap[1] == "fi" {
            if let Some(begin) = stack.pop() {
                if stack.is_empty() {
                    blank(&mut bytes, begin, m.end());
                    items.push(Item {
                        kind: ItemKind::Gap,
                        value: "Conditional TeX content is not evaluated.".into(),
                        offset: begin,
                    });
                }
            }
        }
    }
    if let Some(&begin) = stack.first() {
        blank(&mut bytes, begin, masked.len());
        items.push(Item {
            kind: ItemKind::Gap,
            value: "Unclosed or unsupported conditional TeX content.".into(),
            offset: begin,
        });
    }
    let masked = String::from_utf8(bytes.clone()).unwrap();
    let conditional_commands = Regex::new(r"\\(?:IfFileExists|InputIfFileExists)\b").unwrap();
    for m in conditional_commands.find_iter(&masked) {
        let mut end = m.end();
        for _ in 0..3 {
            end = group_end(&masked, skip_space(&masked, end), b'{', b'}').unwrap_or(masked.len());
        }
        blank(&mut bytes, m.start(), end);
        items.push(Item {
            kind: ItemKind::Gap,
            value: "Conditional file inclusion is not evaluated.".into(),
            offset: m.start(),
        });
    }
    let masked = String::from_utf8(bytes.clone()).unwrap();
    // Do not let trailing source supply anchors after an explicit stop. TeX's
    // expansion and end-of-line details still require the document build.
    let stops = Regex::new(r"\\(?:endinput\b|end\s*\{document\})").unwrap();
    if let Some(m) = stops
        .find_iter(&masked)
        .find(|m| !escaped(&masked, m.start()))
    {
        blank(&mut bytes, m.start(), masked.len());
        if m.as_str() == "\\endinput" {
            items.push(Item {
                kind: ItemKind::Gap,
                value:
                    "End-of-input behavior requires compilation; trailing content is not inspected."
                        .into(),
                offset: m.start(),
            });
        }
    }
    let masked = String::from_utf8(bytes).unwrap();
    let command = Regex::new(r"\\(?P<cmd>input|include|label|ref|eqref|autoref|cref|Cref|secref|seref|appref|apref|eqnref|figref|tabref|axref|nref|mainequation|mainsection|mainappendix|maintable|section|subsection|subsubsection|chapter|title)\b\*?\s*(?:\[[^\]]*\]\s*){0,2}\{(?P<arg>[^{}]*)\}").unwrap();
    for cap in command.captures_iter(&masked) {
        let m = cap.get(0).unwrap();
        if escaped(&masked, m.start()) {
            continue;
        }
        let name = &cap["cmd"];
        let arg = cap.name("arg").unwrap();
        let value = arg.as_str();
        if value.contains(['\\', '#']) {
            items.push(Item {
                kind: ItemKind::Gap,
                value: "Dynamic command operand is not resolved.".into(),
                offset: m.start(),
            });
            continue;
        }
        let kind = match name {
            "input" => ItemKind::Include,
            "include" => ItemKind::SelectedInclude,
            "label" => ItemKind::Label,
            "section" | "subsection" | "subsubsection" | "chapter" | "title" => ItemKind::Heading,
            "mainequation" | "mainsection" | "mainappendix" | "maintable" => {
                ItemKind::MainReference
            }
            _ => ItemKind::Reference,
        };
        if matches!(name, "cref" | "Cref") {
            for value in value.split(',') {
                items.push(Item {
                    kind: ItemKind::Reference,
                    value: value.trim().into(),
                    offset: m.start(),
                });
            }
        } else {
            items.push(Item {
                kind,
                value: value.into(),
                offset: m.start(),
            });
        }
    }
    let starts = Regex::new(r"\\(?:input|include|label|ref|eqref|autoref|cref|Cref|secref|seref|appref|apref|eqnref|figref|tabref|axref|nref|mainequation|mainsection|mainappendix|maintable|section|subsection|subsubsection|chapter|title)\b").unwrap();
    for m in starts.find_iter(&masked) {
        if escaped(&masked, m.start()) {
            continue;
        }
        if command
            .find_at(&masked, m.start())
            .is_none_or(|found| found.start() != m.start())
        {
            items.push(Item {
                kind: ItemKind::Gap,
                value: "Nested or unsupported label/reference/include/heading operand.".into(),
                offset: m.start(),
            });
        }
    }
    for m in Regex::new(r"\\(?:externaldocument|includeonly|endinput|catcode|csname|expandafter)\b")
        .unwrap()
        .find_iter(&masked)
    {
        items.push(Item {
            kind: if m.as_str() == "\\includeonly" {
                ItemKind::IncludeSelection
            } else {
                ItemKind::Gap
            },
            value: "External or dynamic TeX behavior requires explicit context or compilation."
                .into(),
            offset: m.start(),
        });
    }
    items.sort_by_key(|i| i.offset);
    items
}

fn typst_items(source: &str) -> Vec<Item> {
    fn heading_text(node: &SyntaxNode, text: &mut String) {
        match node.kind() {
            SyntaxKind::Text | SyntaxKind::Space => text.push_str(node.text()),
            SyntaxKind::Label
            | SyntaxKind::Ref
            | SyntaxKind::LineComment
            | SyntaxKind::BlockComment => {}
            _ => {
                for child in node.children() {
                    heading_text(child, text);
                }
            }
        }
    }
    fn walk(node: &SyntaxNode, offset: usize, items: &mut Vec<Item>) {
        let raw = node.clone().into_text();
        match node.kind() {
            SyntaxKind::Label => items.push(Item {
                kind: ItemKind::Label,
                value: raw
                    .trim()
                    .trim_start_matches('<')
                    .trim_end_matches('>')
                    .into(),
                offset,
            }),
            SyntaxKind::Ref => {
                let target = raw
                    .split('[')
                    .next()
                    .unwrap_or("")
                    .trim()
                    .trim_start_matches('@');
                items.push(Item {
                    kind: ItemKind::Reference,
                    value: target.into(),
                    offset,
                });
            }
            SyntaxKind::Heading => {
                let mut value = String::new();
                heading_text(node, &mut value);
                items.push(Item {
                    kind: ItemKind::Heading,
                    value,
                    offset,
                });
            }
            _ => {}
        }
        // Includes/imports are inspected as AST code, not text in raw examples.
        if matches!(
            node.kind(),
            SyntaxKind::ModuleInclude | SyntaxKind::ModuleImport
        ) {
            let literal = Regex::new(r##"^#?(?:include|import)\s+\"([^\"]+)\"\s*$"##).unwrap();
            if node.kind() == SyntaxKind::ModuleInclude {
                if let Some(cap) = literal.captures(raw.trim()) {
                    items.push(Item {
                        kind: ItemKind::Include,
                        value: cap[1].into(),
                        offset,
                    });
                } else {
                    items.push(Item {
                        kind: ItemKind::Gap,
                        value: "Dynamic Typst include is not resolved.".into(),
                        offset,
                    });
                }
            } else {
                items.push(Item {
                    kind: ItemKind::Gap,
                    value: "Typst module imports/exports are not evaluated.".into(),
                    offset,
                });
            }
            return;
        }
        if matches!(
            node.kind(),
            SyntaxKind::Raw | SyntaxKind::LineComment | SyntaxKind::BlockComment
        ) {
            return;
        }
        if matches!(
            node.kind(),
            SyntaxKind::Conditional
                | SyntaxKind::Closure
                | SyntaxKind::LetBinding
                | SyntaxKind::ForLoop
                | SyntaxKind::WhileLoop
                | SyntaxKind::FuncCall
                | SyntaxKind::CodeBlock
                | SyntaxKind::Contextual
                | SyntaxKind::ShowRule
                | SyntaxKind::SetRule
        ) {
            items.push(Item {
                kind: ItemKind::Gap,
                value: "Dynamic Typst code is not evaluated.".into(),
                offset,
            });
            return;
        }
        if node.kind() == SyntaxKind::Error {
            items.push(Item {
                kind: ItemKind::Gap,
                value: "Typst syntax error; compile before relying on this scan.".into(),
                offset,
            });
        }
        let mut child_offset = offset;
        for child in node.children() {
            walk(child, child_offset, items);
            child_offset += child.len();
        }
    }
    let mut items = Vec::new();
    walk(&parse(source), 0, &mut items);
    items
}

struct Scanner {
    base: PathBuf,
    options: LabelHygieneOptions,
    report: LabelHygieneReport,
    refs: Vec<Reference>,
    read_files: BTreeMap<String, String>,
    visits: usize,
    restricted_includes: bool,
}

impl Scanner {
    fn gap(&mut self, at: &SourceLocation, reason: &str) {
        if !self
            .report
            .coverage_gaps
            .iter()
            .any(|g| g.file == at.file && g.reason == reason)
        {
            self.report.coverage_gaps.push(CoverageGap {
                file: at.file.clone(),
                line: at.line,
                reason: reason.into(),
            });
        }
    }

    fn names(&mut self, kind: &str, value: &str, at: &SourceLocation) {
        if !self.options.check_names {
            return;
        }
        let found = markers(value);
        // Matter I's documented data namespace is a volume/application name.
        let found: Vec<_> = found
            .into_iter()
            .filter(|m| {
                !(m.eq_ignore_ascii_case("m1")
                    && kind == "workflow_path"
                    && (value.starts_with("manual2/data/m1/")
                        || (self.base.file_name().is_some_and(|n| n == "manual2")
                            && value.starts_with("data/m1/"))))
            })
            .collect();
        if !found.is_empty() {
            self.report.diagnostics.push(issue(kind, Severity::Warning, RuleAnchor::Sec7NotationConsistent, at,
                format!("Project marker(s) {} in '{}'. Prefer a scientific subject name for new material; preserve or deliberately migrate existing compatibility names.", found.join(", "), value)));
        }
    }

    fn visit(&mut self, path: PathBuf, stack: &mut Vec<PathBuf>, caller: Option<SourceLocation>) {
        let at = caller.unwrap_or(SourceLocation {
            file: path.to_string_lossy().into_owned(),
            line: 1,
            column: 1,
        });
        let path = match fs::canonicalize(&path) {
            Ok(p) => p,
            Err(e) => {
                self.report.diagnostics.push(issue(
                    "missing_include",
                    Severity::Error,
                    RuleAnchor::Sec7PreciseAnchors,
                    &at,
                    format!("Cannot read include '{}': {}", path.display(), e),
                ));
                return;
            }
        };
        if stack.contains(&path) {
            self.report.diagnostics.push(issue(
                "include_cycle",
                Severity::Error,
                RuleAnchor::Sec7PreciseAnchors,
                &at,
                format!("Include cycle through '{}'.", path.display()),
            ));
            return;
        }
        if self.visits >= 512 || stack.len() >= 64 {
            self.gap(&at, "Include scan limit reached.");
            return;
        }
        let source = match fs::read_to_string(&path) {
            Ok(s) => s,
            Err(e) => {
                self.report.diagnostics.push(issue(
                    "missing_include",
                    Severity::Error,
                    RuleAnchor::Sec7PreciseAnchors,
                    &at,
                    format!("Cannot read UTF-8 source: {}", e),
                ));
                return;
            }
        };
        let typst = match path.extension().and_then(|e| e.to_str()) {
            Some("typ") => true,
            Some("tex") => false,
            _ => {
                self.gap(&at, "Only .tex and .typ includes are inspected.");
                return;
            }
        };
        self.visits += 1;
        let key = path.to_string_lossy().into_owned();
        let hash = format!("{:x}", Sha256::digest(source.as_bytes()));
        if self
            .read_files
            .get(&key)
            .is_some_and(|previous| previous != &hash)
        {
            self.report.diagnostics.push(issue(
                "source_changed",
                Severity::Error,
                RuleAnchor::Sec7PreciseAnchors,
                &at,
                format!(
                    "Source '{}' changed between includes; rerun on a stable checkout.",
                    path.display()
                ),
            ));
            return;
        }
        if !self.read_files.contains_key(&key) {
            let relative = path
                .strip_prefix(&self.base)
                .unwrap_or_else(|_| Path::new(path.file_name().unwrap()));
            self.names(
                "workflow_path",
                &relative.to_string_lossy(),
                &location(&path, &source, 0),
            );
        }
        self.read_files.insert(key, hash);
        stack.push(path.clone());
        for item in if typst {
            typst_items(&source)
        } else {
            tex_items(&source)
        } {
            let at = location(&path, &source, item.offset);
            match item.kind {
                ItemKind::Gap => self.gap(&at, &item.value),
                ItemKind::Heading => self.names("workflow_heading", &item.value, &at),
                ItemKind::IncludeSelection => {
                    self.restricted_includes = true;
                    self.gap(&at, "TeX includeonly selection and auxiliary state are not evaluated; subsequent include commands are skipped.");
                }
                ItemKind::Include | ItemKind::SelectedInclude => {
                    if matches!(item.kind, ItemKind::SelectedInclude) && self.restricted_includes {
                        self.gap(
                            &at,
                            "Include skipped because includeonly selection requires compilation.",
                        );
                        continue;
                    }
                    let mut relative = PathBuf::from(item.value.trim());
                    if typst && relative.is_absolute() {
                        self.gap(&at, "Typst root-relative include requires the compilation project root; it is not an operating-system absolute path.");
                        continue;
                    }
                    if !typst && relative.extension().is_none() {
                        relative.set_extension("tex");
                    }
                    let base = if typst {
                        path.parent().unwrap()
                    } else {
                        &self.base
                    };
                    self.visit(base.join(relative), stack, Some(at));
                }
                ItemKind::Label => {
                    if item.value.trim().is_empty() {
                        self.report.diagnostics.push(issue(
                            "empty_label",
                            Severity::Error,
                            RuleAnchor::Sec7UniqueLabels,
                            &at,
                            "Label identifier is empty.".into(),
                        ));
                    } else {
                        if item.value.chars().any(char::is_whitespace) {
                            self.report.diagnostics.push(issue("label_whitespace", Severity::Warning, RuleAnchor::Sec7NotationConsistent, &at, format!("Whitespace in label '{}'; use a stable, unambiguous identifier.", item.value)));
                        }
                        self.names("workflow_label", &item.value, &at);
                        self.report.labels.push(LabelDefinition {
                            name: item.value,
                            location: at,
                        });
                    }
                }
                ItemKind::Reference | ItemKind::MainReference => {
                    let target = if matches!(item.kind, ItemKind::MainReference) {
                        format!(
                            "{}{}",
                            self.options.main_reference_prefix.as_deref().unwrap_or(""),
                            item.value
                        )
                    } else {
                        item.value
                    };
                    self.refs.push(Reference {
                        target,
                        location: at,
                        typst,
                    });
                }
            }
        }
        stack.pop();
    }

    fn finish(mut self) -> LabelHygieneReport {
        let mut labels: BTreeMap<String, Vec<SourceLocation>> = BTreeMap::new();
        for label in &self.report.labels {
            labels
                .entry(label.name.clone())
                .or_default()
                .push(label.location.clone());
        }
        for (name, locations) in &labels {
            if locations.len() > 1 || self.options.external_labels.contains(name) {
                let at = locations.last().unwrap();
                let mut d = issue(
                    "duplicate_label",
                    Severity::Error,
                    RuleAnchor::Sec7UniqueLabels,
                    at,
                    format!(
                        "Label '{}' has multiple definitions in this document/import context.",
                        name
                    ),
                );
                d.related = locations[..locations.len() - 1].to_vec();
                self.report.diagnostics.push(d);
            }
        }
        for reference in std::mem::take(&mut self.refs) {
            let local = labels.get(&reference.target);
            if local.is_some_and(|l| l.len() > 1) {
                self.report.diagnostics.push(issue(
                    "ambiguous_reference",
                    Severity::Error,
                    RuleAnchor::Sec7PreciseAnchors,
                    &reference.location,
                    format!(
                        "Reference '{}' has more than one active definition.",
                        reference.target
                    ),
                ));
            } else if local.is_none() && !self.options.external_labels.contains(&reference.target) {
                if reference.typst {
                    self.report.diagnostics.push(issue("unresolved_typst_reference", Severity::Warning, RuleAnchor::Sec7PreciseAnchors, &reference.location,
                        format!("Unresolved Typst reference or bibliography citation '{}'; supply its declared context or compile.", reference.target)));
                    self.gap(
                        &reference.location,
                        "Typst bibliography and dynamic reference resolution are not evaluated.",
                    );
                } else {
                    let partial = !self.report.coverage_gaps.is_empty();
                    self.report.diagnostics.push(issue("unresolved_reference", if partial { Severity::Warning } else { Severity::Error }, RuleAnchor::Sec7PreciseAnchors, &reference.location,
                        format!("No literal definition or supplied external label for '{}'.{}", reference.target,
                            if partial { " Coverage is partial; verify expansion/imports with the document build." } else { "" })));
                }
            }
        }
        self.report.files = self
            .read_files
            .into_iter()
            .map(|(path, sha256)| SourceFile { path, sha256 })
            .collect();
        self.report.status = match self.report.exit_code() {
            1 => "ISSUES_FOUND",
            2 => "PARTIAL",
            _ if self.report.diagnostics.is_empty() => "LITERAL_SCAN_CLEAR",
            _ => "WARNINGS",
        }
        .into();
        self.report
    }
}

pub fn check_labels(
    entry: &Path,
    options: LabelHygieneOptions,
) -> Result<LabelHygieneReport, String> {
    let entry = fs::canonicalize(entry).map_err(|e| format!("Cannot open entry: {}", e))?;
    if !matches!(
        entry.extension().and_then(|s| s.to_str()),
        Some("tex") | Some("typ")
    ) {
        return Err("Entry must be a .tex or .typ document driver.".into());
    }
    if options.external_labels.iter().any(|s| s.trim().is_empty()) {
        return Err("External label names must be nonempty.".into());
    }
    let base = entry.parent().unwrap().to_path_buf();
    let report = LabelHygieneReport { schema_version: "1.0", entry: entry.to_string_lossy().into_owned(), status: String::new(),
        files: Vec::new(), labels: Vec::new(), diagnostics: Vec::new(), coverage_gaps: Vec::new(),
        name_checks_enabled: options.check_names, external_label_count: options.external_labels.len(), main_reference_prefix: options.main_reference_prefix.clone(),
        limitations: vec!["Literal source checks only; no TeX/Typst execution or mathematical certification.",
                         "LaTeX includes resolve from the driver directory; Typst includes from the including file.",
                "Custom macro expansion, conditionals and module imports require compilation.",
                         "Typst references can name bibliography entries; unresolved targets are qualified warnings.",
                         "Project-name warnings are advisory; no identifiers or source files are rewritten."] };
    let mut scanner = Scanner {
        base,
        options,
        report,
        refs: Vec::new(),
        read_files: BTreeMap::new(),
        visits: 0,
        restricted_includes: false,
    };
    for name in scanner.options.additional_paths.clone() {
        if Path::new(&name).is_absolute()
            || Path::new(&name)
                .components()
                .any(|c| matches!(c, std::path::Component::ParentDir))
        {
            return Err("Additional paths must be repository-relative without '..'.".into());
        }
        scanner.names(
            "workflow_path",
            &name,
            &SourceLocation {
                file: name.clone(),
                line: 1,
                column: 1,
            },
        );
    }
    scanner.visit(entry, &mut Vec::new(), None);
    Ok(scanner.finish())
}
