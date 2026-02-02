use crate::node::{Node, NodeType};
use std::path::PathBuf;

pub fn parse_input(input: &str) -> Vec<Node> {
    let lines: Vec<&str> = input
        .lines()
        .filter(|l| !l.trim().is_empty() && !l.trim().starts_with('#'))
        .collect();

    let mut nodes: Vec<Node> = Vec::new();
    let mut indent_map: std::collections::HashMap<usize, usize> = std::collections::HashMap::new();
    let mut iter = lines.iter().enumerate().peekable();

    while let Some((_, line)) = iter.next() {
        let clean_line = line
            .replace('│', "")
            .replace('├', "")
            .replace('└', "")
            .replace('─', "");
        let raw_indent = clean_line.len() - clean_line.trim_start().len();

        // 1. 前後の一般的な空白を除去
        let temp_content = clean_line.trim();
        // 2. 「名前/ 空白」のようなケースに対応するため、最初のスラッシュで分割
        // (シンボリックリンク "->" が含まれない場合のみ適用)
        let (content, is_explicit_dir) =
            if !temp_content.contains("->") && temp_content.contains('/') {
                let base = temp_content.split('/').next().unwrap_or("").trim();
                (base.to_string(), true)
            } else {
                (temp_content.to_string(), temp_content.ends_with('/'))
            };

        // インデント正規化
        let mut sorted_indents: Vec<usize> = indent_map.keys().cloned().collect();
        sorted_indents.sort();
        let depth = if raw_indent == 0 {
            0
        } else if let Some(d) = indent_map.get(&raw_indent) {
            *d
        } else {
            let new_depth = sorted_indents.len();
            indent_map.insert(raw_indent, new_depth);
            new_depth
        };

        let mut has_children = false;
        if let Some((_, next_line)) = iter.peek() {
            let next_clean = next_line
                .replace('│', "")
                .replace('├', "")
                .replace('└', "")
                .replace('─', "");
            let next_raw_indent = next_clean.len() - next_clean.trim_start().len();
            if next_raw_indent > raw_indent {
                has_children = true;
            }
        }

        // 名前の確定と型判定
        let (name, node_type) = if content.contains("->") {
            let parts: Vec<&str> = content.split("->").map(|s| s.trim()).collect();
            let n = parts[0].trim_end_matches('/').trim().to_string();
            let t = parts.get(1).unwrap_or(&"").to_string();
            (n, NodeType::Symlink { target: t })
        } else {
            let n_type = if is_explicit_dir || has_children {
                NodeType::Directory
            } else {
                NodeType::File
            };
            (content.clone(), n_type)
        };

        let mut path = PathBuf::new();
        for prev in nodes.iter().rev() {
            if prev.depth < depth {
                path = prev.path.clone();
                break;
            }
        }
        path.push(&name);

        nodes.push(Node {
            name,
            depth,
            node_type,
            path,
        });
    }
    nodes
}
