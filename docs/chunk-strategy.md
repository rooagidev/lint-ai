# Chunk Strategy Design

This document defines how Lint-AI chunks documents for Tier 1 indexing and query retrieval.

## Goals
- Keep chunks small enough for LLM context windows.
- Keep chunk IDs stable enough for incremental updates.
- Preserve document structure where possible.

## Available Strategies

### 1. `heading` (default)
Splits content by Markdown headings (`#` to `######`).

Rules:
- Text before first heading becomes a `(document)` chunk.
- Each heading section becomes one chunk.
- If no headings exist, the whole file is one chunk.

Best for:
- Well-structured Markdown docs.
- Section-aware retrieval and explanation.

### 2. `line`
Splits by fixed line windows.

Rules:
- Uses `--chunk-lines` as window size.
- Uses `--chunk-overlap` as overlap between windows.
- Produces line-ranged headings like `lines 1-40`.

Best for:
- Stable diffs and incremental updates.
- Documents with weak heading quality.

### 3. `hybrid`
Line-stable chunking + token-aware splitting.

Rules:
- First chunk by line windows (`--chunk-lines`, `--chunk-overlap`).
- Estimate token size for each line chunk.
- If chunk exceeds token budget, split further into smaller parts.
- Controlled by:
  - `--chunk-target-tokens` (preferred size)
  - `--chunk-max-tokens` (hard cap)

Best for:
- LLM context-fit retrieval with stable change tracking.
- Large documents where fixed line windows can be too large.

## CLI Flags
- `--chunk-strategy heading|line|hybrid`
- `--chunk-lines <N>` (default: `40`)
- `--chunk-overlap <N>` (default: `10`)
- `--chunk-target-tokens <N>` (default: `450`)
- `--chunk-max-tokens <N>` (default: `800`)

## Recommended Defaults
For general use:
- `--chunk-strategy hybrid`
- `--chunk-lines 40`
- `--chunk-overlap 10`
- `--chunk-target-tokens 450`
- `--chunk-max-tokens 800`

## Example Commands
Build index with hybrid chunking:

```bash
./target/debug/lint-ai --index /path/to/docs --chunk-strategy hybrid --chunk-lines 40 --chunk-overlap 10 --chunk-target-tokens 450 --chunk-max-tokens 800
```

Query with same chunk strategy:

```bash
./target/debug/lint-ai --query "linux install docker compose" /path/to/docs --chunk-strategy hybrid
```

## Notes on Caching
Query cache validity includes chunk settings. Changing strategy or chunk parameters triggers rebuild.

## Internal Chunk Schema
Each chunk stores:
- `chunk_id`
- `heading`
- `content`
- `key_entities`
- `important_terms`

Chunk IDs are generated as `doc_id::index`.
