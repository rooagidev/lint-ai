fn normalize_component(input: &str) -> String {
    input
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .trim()
        .to_lowercase()
}

fn fnv1a64_hex(parts: &[&str]) -> String {
    let mut hash: u64 = 0xcbf29ce484222325;
    for part in parts {
        for byte in part.as_bytes() {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(0x100000001b3);
        }
        hash ^= u64::from(0xffu8);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("{hash:016x}")
}

pub fn stable_doc_id_from_source(source: &str) -> String {
    let canonical_source = normalize_component(source);
    format!("doc:{}", fnv1a64_hex(&[&canonical_source]))
}

pub fn stable_chunk_id(
    doc_id: &str,
    heading: &str,
    content: &str,
    start_line: usize,
    end_line: usize,
) -> String {
    let normalized_heading = normalize_component(heading);
    let normalized_content = normalize_component(content);
    format!(
        "chunk:{}",
        fnv1a64_hex(&[
            doc_id,
            &normalized_heading,
            &normalized_content,
            &start_line.to_string(),
            &end_line.to_string(),
        ])
    )
}
