use serde::{Deserialize, Serialize};

use crate::ids::stable_doc_id_from_source;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceDocument {
    pub doc_id: String,
    pub source: String,
    pub content: String,
    pub concept: String,
    #[serde(default)]
    pub group_id: Option<String>,
    #[serde(default)]
    pub headings: Vec<String>,
    #[serde(default)]
    pub links: Vec<String>,
    pub timestamp: Option<String>,
    pub doc_length: usize,
    pub author_agent: Option<String>,
}

impl SourceDocument {
    pub fn with_stable_doc_id_from_source(
        source: String,
        content: String,
        concept: String,
        group_id: Option<String>,
        headings: Vec<String>,
        links: Vec<String>,
        timestamp: Option<String>,
        author_agent: Option<String>,
    ) -> Self {
        let doc_id = stable_doc_id_from_source(&source);
        let doc_length = content.len();
        Self {
            doc_id,
            source,
            content,
            concept,
            group_id,
            headings,
            links,
            timestamp,
            doc_length,
            author_agent,
        }
    }
}
