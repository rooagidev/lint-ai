use anyhow::Result;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs;
use walkdir::WalkDir;

pub struct Graph {
    pub pages: HashSet<String>,
    pub links: HashMap<String, HashSet<String>>,
}

impl Graph {
    pub fn build(path: &str) -> Result<Self> {
        let mut pages = HashSet::new();
        let mut links: HashMap<String, HashSet<String>> = HashMap::new();

        let link_re = Regex::new(r"\[\[(.*?)\]\]")?;

        for entry in WalkDir::new(path) {
            let entry = entry?;
            if !entry.file_type().is_file() {
                continue;
            }

            let ext = entry.path().extension().and_then(|s| s.to_str()).unwrap_or("");
            if ext != "md" {
                continue;
            }

            let path_str = entry.path().display().to_string();
            pages.insert(path_str.clone());

            let content = fs::read_to_string(entry.path())?;
            let mut page_links = HashSet::new();

            for cap in link_re.captures_iter(&content) {
                page_links.insert(cap[1].trim().to_string());
            }

            links.insert(path_str, page_links);
        }

        Ok(Self { pages, links })
    }
}
