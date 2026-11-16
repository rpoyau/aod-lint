use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DocumentTier {
    Main,
    Manual,
    Appendix,
    Other,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LabelKind {
    Section,
    Equation,
    Definition,
    Axiom,
    Choice,
    Corollary,
    Motif,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeadingRecord {
    pub level: usize,
    pub title: String,
    pub line: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LabelRecord {
    pub name: String,
    pub file: String,
    pub line: usize,
    pub kind: LabelKind,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefRecord {
    pub target: String,
    pub file: String,
    pub line: usize,
    pub has_anchor: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileRecord {
    pub path: String,
    pub tier: DocumentTier,
    pub frozen: bool,
    pub sha256: String,
    pub line_count: usize,
    pub includes: Vec<String>,
    pub headings: Vec<HeadingRecord>,
    pub labels: Vec<LabelRecord>,
    pub references: Vec<RefRecord>,
    pub distinctions: Vec<String>,
    pub motifs: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EdgeKind {
    Include,
    Reference,
    MotifDependency,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgeRecord {
    pub origin: String,
    pub target: String,
    pub kind: EdgeKind,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirewallViolation {
    pub rule: String,
    pub origin_file: String,
    pub target: String,
    pub line: usize,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AodManifest {
    pub schema_version: String,
    pub root: String,
    pub files: Vec<FileRecord>,
    pub edges: Vec<EdgeRecord>,
    pub firewall_violations: Vec<FirewallViolation>,
    pub include_cycles: Vec<Vec<String>>,
    /// Results from an explicitly supplied preservation baseline, if any.
    #[serde(default)]
    pub frozen_baseline_violations: Vec<String>,
    #[serde(default)]
    pub coverage_gaps: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffHunk {
    pub file: String,
    pub start_line: usize,
    pub line_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SliceExcerpt {
    pub file: String,
    pub tier: DocumentTier,
    pub frozen: bool,
    pub reason: String,
    pub start_line: usize,
    pub end_line: usize,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextBundle {
    pub touched_files: Vec<String>,
    pub frozen_main_violations: Vec<String>,
    pub impacted_files: Vec<String>,
    pub impacted_labels: Vec<String>,
    pub impacted_motifs: Vec<String>,
    pub hops: usize,
    pub excerpts: Vec<SliceExcerpt>,
}
