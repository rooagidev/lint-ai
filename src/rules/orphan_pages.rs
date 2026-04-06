use crate::graph::Graph;
use crate::report::Report;
use std::collections::HashSet;

pub fn check_orphans(graph: &Graph, report: &mut Report) {
    let mut referenced: HashSet<String> = HashSet::new();

    for links in graph.links.values() {
        for link in links {
            referenced.insert(link.clone());
        }
    }

    for page in &graph.pages {
        let file_stem = std::path::Path::new(page)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("");

        if !referenced.contains(file_stem) {
            report.add(format!("Orphan page: {}", page));
        }
    }
}
