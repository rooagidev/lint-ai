use crate::graph::Graph;
use crate::report::Report;
use std::collections::HashSet;

pub fn check_orphans(graph: &Graph, report: &mut Report) {
    let mut referenced: HashSet<String> = HashSet::new();

    for page in &graph.pages {
        for link in &page.links {
            referenced.insert(link.clone());
        }
    }

    for page in &graph.pages {
        if !referenced.contains(&page.concept) {
            report.add(format!("Orphan page: {}", page.rel_path));
        }
    }
}
