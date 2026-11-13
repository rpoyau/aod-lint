use regex::Regex;
use sha2::{Digest, Sha256};
use std::collections::{BTreeSet, HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use typst_syntax::{parse, SyntaxKind, SyntaxNode};

use crate::models::manifest::{
    AodManifest, DocumentTier, EdgeKind, EdgeRecord, FileRecord, FirewallViolation, HeadingRecord,
    LabelKind, LabelRecord, RefRecord,
};

const KNOWN_MOTIFS: &[(&str, &str)] = &[
    ("Monon", r"\bMonon\b"),
    ("Dion", r"\bDion\b"),
    ("Duon", r"\bDuon\b"),
    ("Duad", r"\bDuad\b"),
    ("Dimonon", r"\bDimonon\b"),
    ("Ditrion", r"\bDitrion\b"),
    ("Tritrion", r"\bTritrion\b"),
    ("Tetratrion", r"\bTetratrion\b"),
    ("RingGate", r"(?i)\bRing\s+Gate\b"),
    ("SADAR", r"\bSADAR\b|\\SADARop\b"),
    ("ADAR", r"\bADAR\b"),
    ("Awareness", r"\bAwareness\b|\bSELF\s*/\s*BETWEEN\s*/\s*NOT-SELF\b"),
    ("DEC", r"\bD\.E\.C\."),
    ("Stokes", r"\bStokes\b"),
    ("CurlingCurl", r"(?i)\bcurling[- ]curls?\b"),
    ("Hinge", r"(?i)\bhinge\b|\b0_H\b"),
];

pub struct AodDigestPass {
    root: PathBuf,
}

impl AodDigestPass {
    pub fn new<P: AsRef<Path>>(root: P) -> Self {
        Self {
            root: root.as_ref().to_path_buf(),
        }
    }

    pub fn run(&self) -> Result<AodManifest, String> {
        let base_dir = if self.root.is_file() {
            self.root
                .parent()
                .map(Path::to_path_buf)
                .unwrap_or_else(|| PathBuf::from("."))
        } else {
            self.root.clone()
        };

        let mut source_files = Vec::new();
        collect_source_files(&base_dir, &base_dir, &mut source_files)?;
        source_files.sort();

        let typ_include_re = Regex::new(r#"#(?:include|import)\s+"([^"]+\.typ)""#).unwrap();
        let typ_main_ref_re =
            Regex::new(r#"#main-(?:ref|eq|sec|cor|app|tab|choice|con|ax)\(\s*"([^"]+)""#).unwrap();
        let tex_input_re = Regex::new(r"\\(?:input|include)\{([^}]+)\}").unwrap();
        let tex_heading_re = Regex::new(r"\\(sub)*section\*?(?:\[[^\]]*\])?\{([^}]+)\}").unwrap();
        let tex_label_re = Regex::new(r"\\label\{([^}]+)\}").unwrap();
        let tex_ref_re = Regex::new(
            r"\\(?:ref|eqref|secref|eqnref|appref|tabref|figref|mainequation|mainsection|mainappendix|maintable)\{([^}]+)\}",
        )
        .unwrap();
        let tex_cite_re = Regex::new(r"\\cite(\[[^\]]*\])?\{([^}]+)\}").unwrap();

        let choice_re = Regex::new(r"\b(Choice\s+[A-Z0-9]+|Axiom\s+[A-Z0-9]+|Def(?:inition)?\s+[0-9]+)\b").unwrap();
        let motif_res: Vec<(&str, Regex)> = KNOWN_MOTIFS
            .iter()
            .map(|(name, pat)| (*name, Regex::new(pat).unwrap()))
            .collect();

        let mut files = Vec::new();
        for rel_path in &source_files {
            let full_path = base_dir.join(rel_path);
            let bytes = fs::read(&full_path)
                .map_err(|e| format!("Failed to read {}: {}", full_path.display(), e))?;
            let sha256 = sha256_hex(&bytes);
            let content = String::from_utf8_lossy(&bytes).to_string();
            let line_count = content.lines().count();
            let tier = classify_tier(rel_path);
            let frozen = matches!(tier, DocumentTier::Main | DocumentTier::Appendix);

            let mut includes = Vec::new();
            let mut headings = Vec::new();
            let mut labels = Vec::new();
            let mut references = Vec::new();

            if rel_path.ends_with(".tex") {
                extract_latex_items(
                    rel_path,
                    &content,
                    &tex_input_re,
                    &tex_heading_re,
                    &tex_label_re,
                    &tex_ref_re,
                    &tex_cite_re,
                    &mut includes,
                    &mut headings,
                    &mut labels,
                    &mut references,
                );
            } else {
                for cap in typ_include_re.captures_iter(&content) {
                    let raw_inc = &cap[1];
                    let resolved = resolve_relative_include(rel_path, raw_inc);
                    if !includes.contains(&resolved) {
                        includes.push(resolved);
                    }
                }

                for (idx, raw_line) in content.lines().enumerate() {
                    let line = idx + 1;
                    let code = raw_line.split("//").next().unwrap_or("");
                    for cap in typ_main_ref_re.captures_iter(code) {
                        let target = cap[1].trim().to_string();
                        if !target.is_empty() {
                            references.push(RefRecord {
                                target,
                                file: rel_path.clone(),
                                line,
                                has_anchor: true,
                            });
                        }
                    }
                }

                let root_ast = parse(&content);
                extract_ast_items(
                    &root_ast,
                    0,
                    rel_path,
                    &content,
                    &mut headings,
                    &mut labels,
                    &mut references,
                );
            }

            let mut distinctions_set = BTreeSet::new();
            for cap in choice_re.captures_iter(&content) {
                distinctions_set.insert(cap[1].to_string());
            }

            let mut motifs_set = BTreeSet::new();
            for (name, re) in &motif_res {
                if re.is_match(&content) {
                    motifs_set.insert((*name).to_string());
                }
            }

            files.push(FileRecord {
                path: rel_path.clone(),
                tier,
                frozen,
                sha256,
                line_count,
                includes,
                headings,
                labels,
                references,
                distinctions: distinctions_set.into_iter().collect(),
                motifs: motifs_set.into_iter().collect(),
            });
        }

        let mut label_to_file: HashMap<String, (String, DocumentTier)> = HashMap::new();
        for f in &files {
            for lbl in &f.labels {
                label_to_file.insert(lbl.name.clone(), (f.path.clone(), f.tier));
            }
        }

        let mut edges = Vec::new();
        let mut firewall_violations = Vec::new();
        let mut include_adj: HashMap<String, Vec<String>> = HashMap::new();

        for f in &files {
            for inc in &f.includes {
                edges.push(EdgeRecord {
                    origin: f.path.clone(),
                    target: inc.clone(),
                    kind: EdgeKind::Include,
                });
                include_adj
                    .entry(f.path.clone())
                    .or_default()
                    .push(inc.clone());
            }

            for r in &f.references {
                let resolved_target = label_to_file.get(&r.target).or_else(|| {
                    r.target
                        .strip_prefix("main:")
                        .and_then(|stripped| label_to_file.get(stripped))
                });
                if let Some((target_file, target_tier)) = resolved_target {
                    edges.push(EdgeRecord {
                        origin: f.path.clone(),
                        target: target_file.clone(),
                        kind: EdgeKind::Reference,
                    });

                    if matches!(f.tier, DocumentTier::Main | DocumentTier::Appendix)
                        && *target_tier == DocumentTier::Manual
                    {
                        firewall_violations.push(FirewallViolation {
                            rule: "AOD Main/Manual Separation".to_string(),
                            origin_file: f.path.clone(),
                            target: format!("@{} ({})", r.target, target_file),
                            line: r.line,
                            message: format!(
                                "Frozen {:?} file '{}' references Manual label '@{}' in '{}'. Main must not depend on Manual.",
                                f.tier, f.path, r.target, target_file
                            ),
                        });
                    }
                }
            }

            for m in &f.motifs {
                edges.push(EdgeRecord {
                    origin: f.path.clone(),
                    target: format!("motif:{}", m),
                    kind: EdgeKind::MotifDependency,
                });
            }
        }

        let include_cycles = detect_cycles(&include_adj);

        Ok(AodManifest {
            schema_version: "1.0".to_string(),
            root: ".".to_string(),
            files,
            edges,
            firewall_violations,
            include_cycles,
        })
    }

    pub fn verify_frozen_baseline(
        baseline: &AodManifest,
        current: &AodManifest,
    ) -> Vec<String> {
        let baseline_hashes: HashMap<&str, &str> = baseline
            .files
            .iter()
            .map(|f| (f.path.as_str(), f.sha256.as_str()))
            .collect();
        let current_hashes: HashMap<&str, &str> = current
            .files
            .iter()
            .map(|f| (f.path.as_str(), f.sha256.as_str()))
            .collect();

        let mut violations = Vec::new();
        for base_file in &baseline.files {
            if !base_file.frozen {
                continue;
            }
            match current_hashes.get(base_file.path.as_str()) {
                None => violations.push(format!(
                    "Frozen {:?} file '{}' was deleted or moved (expected sha256 {}).",
                    base_file.tier, base_file.path, base_file.sha256
                )),
                Some(&cur_hash) if cur_hash != base_file.sha256 => violations.push(format!(
                    "Frozen {:?} file '{}' was mutated (baseline sha256 {} != current {}).",
                    base_file.tier, base_file.path, base_file.sha256, cur_hash
                )),
                _ => {}
            }
        }
        for cur_file in &current.files {
            if cur_file.frozen && !baseline_hashes.contains_key(cur_file.path.as_str()) {
                violations.push(format!(
                    "New frozen {:?} file '{}' was added (sha256 {}).",
                    cur_file.tier, cur_file.path, cur_file.sha256
                ));
            }
        }
        violations
    }
}

pub fn collect_source_files(base: &Path, dir: &Path, out: &mut Vec<String>) -> Result<(), String> {
    let entries = fs::read_dir(dir)
        .map_err(|e| format!("Failed to read directory {}: {}", dir.display(), e))?;
    for entry in entries {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        let file_name = entry.file_name().to_string_lossy().to_string();
        if file_name.starts_with('.') || file_name == "target" || file_name == "dist" {
            continue;
        }
        if path.is_dir() {
            collect_source_files(base, &path, out)?;
        } else if matches!(
            path.extension().and_then(|e| e.to_str()),
            Some("typ") | Some("tex")
        ) {
            let rel = path
                .strip_prefix(base)
                .unwrap_or(&path)
                .to_string_lossy()
                .replace('\\', "/");
            out.push(rel);
        }
    }
    Ok(())
}

fn classify_tier(rel_path: &str) -> DocumentTier {
    if rel_path.starts_with("manual/") || rel_path == "manual.typ" || rel_path == "manual.tex" {
        DocumentTier::Manual
    } else if rel_path.starts_with("appendices/") {
        DocumentTier::Appendix
    } else if rel_path == "main.typ"
        || rel_path == "main.tex"
        || rel_path == "preamble.typ"
        || rel_path == "preamble.tex"
        || rel_path.starts_with("sections/")
    {
        DocumentTier::Main
    } else {
        DocumentTier::Other
    }
}

#[allow(clippy::too_many_arguments)]
fn extract_latex_items(
    file_path: &str,
    content: &str,
    input_re: &Regex,
    heading_re: &Regex,
    label_re: &Regex,
    ref_re: &Regex,
    cite_re: &Regex,
    includes: &mut Vec<String>,
    headings: &mut Vec<HeadingRecord>,
    labels: &mut Vec<LabelRecord>,
    references: &mut Vec<RefRecord>,
) {
    use crate::passes::latex::strip_latex_comment;

    for (idx, raw_line) in content.lines().enumerate() {
        let line = idx + 1;
        let code = strip_latex_comment(raw_line);
        if code.trim().is_empty() {
            continue;
        }

        for cap in input_re.captures_iter(code) {
            let mut raw_inc = cap[1].trim().to_string();
            if !raw_inc.ends_with(".tex") {
                raw_inc.push_str(".tex");
            }
            let resolved = resolve_relative_include(file_path, &raw_inc);
            if !includes.contains(&resolved) {
                includes.push(resolved);
            }
        }

        for cap in heading_re.captures_iter(code) {
            let full = cap.get(0).unwrap().as_str();
            let level = if full.starts_with("\\subsubsection") {
                3
            } else if full.starts_with("\\subsection") {
                2
            } else {
                1
            };
            headings.push(HeadingRecord {
                level,
                title: cap[2].trim().to_string(),
                line,
            });
        }

        for cap in label_re.captures_iter(code) {
            let name = cap[1].trim().to_string();
            if !name.is_empty() {
                labels.push(LabelRecord {
                    kind: classify_label(&name),
                    name,
                    file: file_path.to_string(),
                    line,
                });
            }
        }

        for cap in ref_re.captures_iter(code) {
            for target in cap[1].split(',') {
                let target = target.trim().to_string();
                if !target.is_empty() && !target.starts_with('#') {
                    references.push(RefRecord {
                        target,
                        file: file_path.to_string(),
                        line,
                        has_anchor: true,
                    });
                }
            }
        }

        for cap in cite_re.captures_iter(code) {
            let has_anchor = cap.get(1).is_some();
            for key in cap[2].split(',') {
                let key = key.trim().to_string();
                if !key.is_empty() {
                    references.push(RefRecord {
                        target: key,
                        file: file_path.to_string(),
                        line,
                        has_anchor,
                    });
                }
            }
        }
    }
}

fn resolve_relative_include(origin_rel: &str, include_rel: &str) -> String {
    let origin_parent = Path::new(origin_rel)
        .parent()
        .unwrap_or_else(|| Path::new(""));
    let joined = origin_parent.join(include_rel);
    let mut parts = Vec::new();
    for comp in joined.components() {
        match comp {
            std::path::Component::CurDir => {}
            std::path::Component::ParentDir => {
                parts.pop();
            }
            std::path::Component::Normal(s) => {
                parts.push(s.to_string_lossy().to_string());
            }
            _ => {}
        }
    }
    parts.join("/")
}

fn sha256_hex(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    let digest = hasher.finalize();
    digest.iter().map(|b| format!("{:02x}", b)).collect()
}

fn offset_to_line(source: &str, offset: usize) -> usize {
    let clamped = offset.min(source.len());
    source[..clamped].bytes().filter(|&b| b == b'\n').count() + 1
}

fn classify_label(name: &str) -> LabelKind {
    if name.starts_with("eq:") || name.starts_with("eq-") {
        LabelKind::Equation
    } else if name.starts_with("sec:") || name.starts_with("sec-") {
        LabelKind::Section
    } else if name.starts_with("def:") || name.starts_with("def-") {
        LabelKind::Definition
    } else if name.starts_with("ax:") || name.starts_with("ax-") {
        LabelKind::Axiom
    } else if name.starts_with("ch:") || name.starts_with("choice-") {
        LabelKind::Choice
    } else if name.starts_with("cor:") || name.starts_with("cor-") {
        LabelKind::Corollary
    } else if name.starts_with("motif:") || name.starts_with("motif-") {
        LabelKind::Motif
    } else {
        LabelKind::Other
    }
}

fn extract_ast_items(
    node: &SyntaxNode,
    offset: usize,
    file_path: &str,
    source: &str,
    headings: &mut Vec<HeadingRecord>,
    labels: &mut Vec<LabelRecord>,
    references: &mut Vec<RefRecord>,
) {
    let line = offset_to_line(source, offset);
    match node.kind() {
        SyntaxKind::Heading => {
            let full = node.clone().into_text();
            let trimmed = full.trim();
            let level = trimmed.chars().take_while(|&c| c == '=').count().max(1);
            let title = trimmed.trim_start_matches('=').trim().to_string();
            headings.push(HeadingRecord { level, title, line });
        }
        SyntaxKind::Label => {
            let raw = node.clone().into_text();
            let name = raw
                .trim()
                .trim_start_matches('<')
                .trim_end_matches('>')
                .to_string();
            if !name.is_empty() {
                labels.push(LabelRecord {
                    kind: classify_label(&name),
                    name,
                    file: file_path.to_string(),
                    line,
                });
            }
        }
        SyntaxKind::Ref => {
            let raw = node.clone().into_text();
            let has_anchor = raw.contains('[');
            let target_part = raw.split('[').next().unwrap_or("").trim();
            let target = target_part.trim_start_matches('@').to_string();
            if !target.is_empty() {
                references.push(RefRecord {
                    target,
                    file: file_path.to_string(),
                    line,
                    has_anchor,
                });
            }
        }
        _ => {}
    }

    let mut child_offset = offset;
    for child in node.children() {
        extract_ast_items(
            child,
            child_offset,
            file_path,
            source,
            headings,
            labels,
            references,
        );
        child_offset += child.len();
    }
}

fn detect_cycles(adj: &HashMap<String, Vec<String>>) -> Vec<Vec<String>> {
    let mut visited = HashSet::new();
    let mut in_stack = HashSet::new();
    let mut stack = Vec::new();
    let mut cycles = Vec::new();

    let mut keys: Vec<&String> = adj.keys().collect();
    keys.sort();

    for k in keys {
        if !visited.contains(k.as_str()) {
            dfs_cycle(
                k,
                adj,
                &mut visited,
                &mut in_stack,
                &mut stack,
                &mut cycles,
            );
        }
    }
    cycles
}

fn dfs_cycle(
    node: &str,
    adj: &HashMap<String, Vec<String>>,
    visited: &mut HashSet<String>,
    in_stack: &mut HashSet<String>,
    stack: &mut Vec<String>,
    cycles: &mut Vec<Vec<String>>,
) {
    visited.insert(node.to_string());
    in_stack.insert(node.to_string());
    stack.push(node.to_string());

    if let Some(neighbors) = adj.get(node) {
        for next in neighbors {
            if !visited.contains(next.as_str()) {
                dfs_cycle(next, adj, visited, in_stack, stack, cycles);
            } else if in_stack.contains(next.as_str()) {
                if let Some(pos) = stack.iter().position(|x| x == next) {
                    let mut cycle = stack[pos..].to_vec();
                    cycle.push(next.clone());
                    cycles.push(cycle);
                }
            }
        }
    }

    stack.pop();
    in_stack.remove(node);
}
