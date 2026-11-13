use regex::Regex;
use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};
use std::fs;
use std::path::Path;

use crate::models::manifest::{AodManifest, ContextBundle, DiffHunk, FileRecord, SliceExcerpt};

pub struct AodSlicePass<'a> {
    manifest: &'a AodManifest,
    hops: usize,
    context_lines: usize,
}

impl<'a> AodSlicePass<'a> {
    pub fn new(manifest: &'a AodManifest, hops: usize) -> Self {
        Self {
            manifest,
            hops,
            context_lines: 12,
        }
    }

    pub fn run_from_diff(&self, diff_text: &str) -> ContextBundle {
        let hunks = parse_unified_diff(diff_text);
        let mut touched_set = BTreeSet::new();
        for h in &hunks {
            touched_set.insert(h.file.clone());
        }
        let touched_files: Vec<String> = touched_set.into_iter().collect();
        self.build_bundle(touched_files, &hunks)
    }

    pub fn run_from_files(&self, files: Vec<String>) -> ContextBundle {
        self.build_bundle(files, &[])
    }

    fn build_bundle(&self, touched_files: Vec<String>, hunks: &[DiffHunk]) -> ContextBundle {
        let file_map: HashMap<&str, &FileRecord> = self
            .manifest
            .files
            .iter()
            .map(|f| (f.path.as_str(), f))
            .collect();

        let mut frozen_main_violations = Vec::new();
        for tf in &touched_files {
            if let Some(rec) = file_map.get(tf.as_str()) {
                if rec.frozen {
                    frozen_main_violations.push(format!(
                        "Frozen {:?} file '{}' was modified in PR diff (sha256 {}).",
                        rec.tier, rec.path, rec.sha256
                    ));
                }
            }
        }

        // Build bidirectional adjacency between file nodes (and via motif nodes).
        let mut adj: HashMap<String, BTreeSet<String>> = HashMap::new();
        for edge in &self.manifest.edges {
            adj.entry(edge.origin.clone())
                .or_default()
                .insert(edge.target.clone());
            adj.entry(edge.target.clone())
                .or_default()
                .insert(edge.origin.clone());
        }

        // BFS up to `self.hops` from touched files.
        let mut visited: HashSet<String> = HashSet::new();
        let mut queue: VecDeque<(String, usize)> = VecDeque::new();
        for tf in &touched_files {
            if visited.insert(tf.clone()) {
                queue.push_back((tf.clone(), 0));
            }
        }

        while let Some((node, depth)) = queue.pop_front() {
            if depth >= self.hops {
                continue;
            }
            if let Some(neighbors) = adj.get(&node) {
                for next in neighbors {
                    if visited.insert(next.clone()) {
                        queue.push_back((next.clone(), depth + 1));
                    }
                }
            }
        }

        let mut impacted_files_set = BTreeSet::new();
        let mut impacted_motifs_set = BTreeSet::new();
        for item in &visited {
            if let Some(motif) = item.strip_prefix("motif:") {
                impacted_motifs_set.insert(motif.to_string());
            } else if file_map.contains_key(item.as_str()) {
                impacted_files_set.insert(item.clone());
            }
        }

        let mut impacted_labels_set = BTreeSet::new();
        for path in &impacted_files_set {
            if let Some(rec) = file_map.get(path.as_str()) {
                for lbl in &rec.labels {
                    impacted_labels_set.insert(lbl.name.clone());
                }
                for m in &rec.motifs {
                    impacted_motifs_set.insert(m.clone());
                }
            }
        }

        let touched_lookup: HashSet<&str> = touched_files.iter().map(|s| s.as_str()).collect();
        let mut excerpts = Vec::new();
        let root_path = Path::new(&self.manifest.root);

        for path in &impacted_files_set {
            let Some(rec) = file_map.get(path.as_str()) else {
                continue;
            };
            let full_path = root_path.join(path);
            let Ok(content) = fs::read_to_string(&full_path) else {
                continue;
            };
            let lines: Vec<&str> = content.lines().collect();
            if lines.is_empty() {
                continue;
            }

            if touched_lookup.contains(path.as_str()) {
                let file_hunks: Vec<&DiffHunk> = hunks.iter().filter(|h| h.file == *path).collect();
                if file_hunks.is_empty() {
                    let end = lines.len().min(60);
                    excerpts.push(SliceExcerpt {
                        file: path.clone(),
                        tier: rec.tier,
                        frozen: rec.frozen,
                        reason: "touched file".to_string(),
                        start_line: 1,
                        end_line: end,
                        content: lines[..end].join("\n"),
                    });
                } else {
                    for h in file_hunks {
                        let start = h.start_line.saturating_sub(self.context_lines).max(1);
                        let end = (h.start_line + h.line_count + self.context_lines).min(lines.len());
                        if start <= end {
                            excerpts.push(SliceExcerpt {
                                file: path.clone(),
                                tier: rec.tier,
                                frozen: rec.frozen,
                                reason: format!("diff hunk at line {}", h.start_line),
                                start_line: start,
                                end_line: end,
                                content: lines[start - 1..end].join("\n"),
                            });
                        }
                    }
                }
            } else {
                let end = lines.len().min(40);
                excerpts.push(SliceExcerpt {
                    file: path.clone(),
                    tier: rec.tier,
                    frozen: rec.frozen,
                    reason: format!("reachable within {} hop(s)", self.hops),
                    start_line: 1,
                    end_line: end,
                    content: lines[..end].join("\n"),
                });
            }
        }

        ContextBundle {
            touched_files,
            frozen_main_violations,
            impacted_files: impacted_files_set.into_iter().collect(),
            impacted_labels: impacted_labels_set.into_iter().collect(),
            impacted_motifs: impacted_motifs_set.into_iter().collect(),
            hops: self.hops,
            excerpts,
        }
    }
}

pub fn parse_unified_diff(diff_text: &str) -> Vec<DiffHunk> {
    let hunk_re = Regex::new(r"^@@\s+-\d+(?:,\d+)?\s+\+(\d+)(?:,(\d+))?\s+@@").unwrap();
    let mut hunks = Vec::new();
    let mut current_file: Option<String> = None;

    for line in diff_text.lines() {
        if let Some(rest) = line.strip_prefix("+++ b/") {
            current_file = Some(rest.trim().to_string());
        } else if let Some(rest) = line.strip_prefix("+++ ") {
            let trimmed = rest.trim();
            if trimmed != "/dev/null" {
                current_file = Some(trimmed.trim_start_matches("b/").to_string());
            }
        } else if let Some(cap) = hunk_re.captures(line) {
            if let Some(ref file) = current_file {
                let start_line: usize = cap[1].parse().unwrap_or(1).max(1);
                let line_count: usize = cap
                    .get(2)
                    .and_then(|m| m.as_str().parse().ok())
                    .unwrap_or(1);
                hunks.push(DiffHunk {
                    file: file.clone(),
                    start_line,
                    line_count,
                });
            }
        }
    }

    hunks
}
