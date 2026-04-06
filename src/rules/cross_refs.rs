use crate::graph::Graph;
use crate::report::Report;
use aho_corasick::AhoCorasick;
use std::collections::HashSet;

fn is_word_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_' || c == '-'
}

fn has_word_boundaries(text: &str, start: usize, end: usize) -> bool {
    let left_ok = if start == 0 {
        true
    } else {
        text[..start]
            .chars()
            .next_back()
            .map(|c| !is_word_char(c))
            .unwrap_or(true)
    };

    let right_ok = if end >= text.len() {
        true
    } else {
        text[end..]
            .chars()
            .next()
            .map(|c| !is_word_char(c))
            .unwrap_or(true)
    };

    left_ok && right_ok
}

fn normalize_concept(s: &str) -> String {
    s.trim().to_lowercase().replace('_', " ").replace('-', " ")
}

pub fn check_cross_refs(graph: &Graph, report: &mut Report) {
    let mut concepts: Vec<String> = Vec::new();
    let mut raw_concepts: Vec<String> = Vec::new();

    for page in &graph.pages {
        if let Some(stem) = std::path::Path::new(page)
            .file_stem()
            .and_then(|s| s.to_str())
        {
            raw_concepts.push(stem.to_string());
            concepts.push(normalize_concept(stem));
        }
    }

    let ac = AhoCorasick::new(&concepts).unwrap();

    for (page, content) in &graph.contents {
        let linked: HashSet<String> = graph
            .links
            .get(page)
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .map(|s| normalize_concept(&s))
            .collect();

        let page_stem = std::path::Path::new(page)
            .file_stem()
            .and_then(|s| s.to_str())
            .map(normalize_concept)
            .unwrap_or_default();

        let content_lower = normalize_concept(content);
        let mut found: HashSet<String> = HashSet::new();

        for mat in ac.find_iter(&content_lower) {
            let start = mat.start();
            let end = mat.end();
            if !has_word_boundaries(&content_lower, start, end) {
                continue;
            }

            let concept = &concepts[mat.pattern()];
            found.insert(concept.clone());
        }

        for concept in found {
            if concept == page_stem {
                continue;
            }

            if !linked.contains(&concept) {
                report.add(format!(
                    "Missing cross-ref in {} -> [[{}]]",
                    page, concept
                ));
            }
        }
    }
}
