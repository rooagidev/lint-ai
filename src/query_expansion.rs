use deunicode::deunicode;
use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::sync::OnceLock;

#[derive(Debug, Clone)]
pub struct ExpandedQuery {
    pub original_terms: Vec<String>,
    pub expanded_terms: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct Entry {
    term: String,
    related: Vec<Related>,
}

#[derive(Debug, Clone, Deserialize)]
struct Related {
    term: String,
    relation: String,
    confidence: f32,
}

#[derive(Debug, Default)]
struct LexicalStore {
    by_term: HashMap<String, Vec<Related>>,
}

static WORDNET_DATA: &str = include_str!("../data/lexical/wordnet_subset.json");
static CONCEPTNET_DATA: &str = include_str!("../data/lexical/conceptnet_subset.json");
static STORE: OnceLock<Option<LexicalStore>> = OnceLock::new();

const MAX_EXPANSIONS_PER_TERM: usize = 3;
const CONCEPTNET_MIN_CONFIDENCE: f32 = 0.82;

pub fn expand_query_terms(input_terms: &[String]) -> ExpandedQuery {
    let original_terms = input_terms
        .iter()
        .map(|t| normalize_for_index(t))
        .filter(|t| !t.is_empty())
        .collect::<Vec<_>>();

    let Some(store) = STORE.get_or_init(load_store).as_ref() else {
        return ExpandedQuery {
            original_terms,
            expanded_terms: Vec::new(),
        };
    };

    let original_set: HashSet<String> = original_terms.iter().cloned().collect();
    let mut expanded = Vec::new();

    for term in &original_terms {
        if let Some(related) = store.by_term.get(term) {
            let mut count = 0usize;
            for rel in related {
                if count >= MAX_EXPANSIONS_PER_TERM {
                    break;
                }
                let candidate = normalize_for_index(&rel.term);
                if candidate.is_empty() || original_set.contains(&candidate) || expanded.contains(&candidate) {
                    continue;
                }
                expanded.push(candidate);
                count += 1;
            }
        }
    }

    ExpandedQuery {
        original_terms,
        expanded_terms: expanded,
    }
}

fn load_store() -> Option<LexicalStore> {
    let mut store = LexicalStore::default();
    let wn: Vec<Entry> = serde_json::from_str(WORDNET_DATA).ok()?;
    let cn: Vec<Entry> = serde_json::from_str(CONCEPTNET_DATA).ok()?;

    for e in wn {
        let key = normalize_for_index(&e.term);
        if key.is_empty() {
            continue;
        }
        for rel in e.related {
            if !matches!(rel.relation.as_str(), "Synonym" | "SimilarTo") {
                continue;
            }
            store.by_term.entry(key.clone()).or_default().push(rel);
        }
    }

    for e in cn {
        let key = normalize_for_index(&e.term);
        if key.is_empty() {
            continue;
        }
        for rel in e.related {
            if !matches!(rel.relation.as_str(), "Synonym" | "SimilarTo" | "RelatedTo") {
                continue;
            }
            if rel.relation == "RelatedTo" && rel.confidence < CONCEPTNET_MIN_CONFIDENCE {
                continue;
            }
            store.by_term.entry(key.clone()).or_default().push(rel);
        }
    }

    for rels in store.by_term.values_mut() {
        rels.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap_or(std::cmp::Ordering::Equal));
    }
    Some(store)
}

pub fn normalize_for_index(input: &str) -> String {
    let lowered = deunicode(input).to_lowercase();
    let token_re = regex::Regex::new(r"[A-Za-z][A-Za-z0-9_-]{2,}").expect("valid regex");
    token_re
        .find_iter(&lowered)
        .map(|m| m.as_str())
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_basic() {
        assert_eq!(normalize_for_index("Virtual-Machine!"), "virtual-machine");
        assert_eq!(normalize_for_index("Café"), "cafe");
    }

    #[test]
    fn expansion_caps_and_dedups() {
        let terms = vec!["install".to_string(), "setup".to_string()];
        let out = expand_query_terms(&terms);
        assert!(out.expanded_terms.len() <= terms.len() * MAX_EXPANSIONS_PER_TERM);
        for t in &out.expanded_terms {
            assert!(!out.original_terms.contains(t));
        }
    }

    #[test]
    fn install_expands_to_setup() {
        let out = expand_query_terms(&["install".to_string()]);
        assert!(
            out.expanded_terms.iter().any(|t| t == "setup"),
            "expected setup expansion, got {:?}",
            out.expanded_terms
        );
    }
}
