use crate::graph::Graph;
use crate::report::Report;
use aho_corasick::AhoCorasick;
use std::collections::HashSet;

pub fn check_cross_refs(graph: &Graph, report: &mut Report) {
    let mut concepts: Vec<String> = Vec::new();

    for page in &graph.pages {
        if let Some(stem) = std::path::Path::new(page)
            .file_stem()
            .and_then(|s| s.to_str())
        {
            concepts.push(stem.to_lowercase());
        }
    }

    let ac = AhoCorasick::new(&concepts).unwrap();

    for (page, content) in &graph.contents {
        let linked = graph.links.get(page).cloned().unwrap_or_default();
        let content_lower = content.to_lowercase();

        let mut found: HashSet<String> = HashSet::new();

        for mat in ac.find_iter(&content_lower) {
            let concept = &concepts[mat.pattern()];
            found.insert(concept.clone());
        }

        for concept in found {
            if !linked.contains(&concept) {
                report.add(format!(
                    "Missing cross-ref in {} -> [[{}]]",
                    page, concept
                ));
            }
        }
    }
}
