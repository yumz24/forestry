use crate::node::{Node, NodeType};
use std::path::PathBuf;

pub fn parse_input(input: &str) -> Vec<Node> {
    let lines: Vec<&str> = input
        .lines()
        .filter(|l| !l.trim().is_empty() && !l.trim().starts_with('#'))
        .collect();

    let mut nodes: Vec<Node> = Vec::new();
    // (インデントの実際の長さ, 深さレベル) を管理するスタック
    let mut indent_levels = vec![0]; 
    let mut path_stack: Vec<(usize, PathBuf)> = Vec::new();

    let mut iter = lines.iter().enumerate().peekable();

    while let Some((_, line)) = iter.next() {
        let clean_line = line
            .replace('│', "").replace('├', "").replace('└', "").replace('─', "");
        
        // 実際のインデントの「長さ」を取得
        let raw_indent_len = clean_line.len() - clean_line.trim_start().len();
        let temp_content = clean_line.trim();

        // --- インデント深さの判定 (スタック比較方式) ---
        if raw_indent_len > *indent_levels.last().unwrap() {
            indent_levels.push(raw_indent_len);
        } else {
            while raw_indent_len < *indent_levels.last().unwrap() && indent_levels.len() > 1 {
                indent_levels.pop();
            }
        }
        let depth = indent_levels.len() - 1;

        // --- ディレクトリ判定と名前抽出 ---
        let (content, is_explicit_dir) =
            if !temp_content.contains("->") && temp_content.contains('/') {
                let base = temp_content.split('/').next().unwrap_or("").trim();
                (base.to_string(), true)
            } else {
                (temp_content.to_string(), temp_content.ends_with('/'))
            };

        let mut has_children = false;
        if let Some((_, next_line)) = iter.peek() {
            let next_clean = next_line.replace('│', "").replace('├', "").replace('└', "").replace('─', "");
            let next_raw_indent = next_clean.len() - next_clean.trim_start().len();
            if next_raw_indent > raw_indent_len {
                has_children = true;
            }
        }

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
            (content, n_type)
        };

        // --- パス解決 ---
        while let Some((d, _)) = path_stack.last() {
            if *d >= depth {
                path_stack.pop();
            } else {
                break;
            }
        }

        let mut full_path = if let Some((_, parent_path)) = path_stack.last() {
            parent_path.clone()
        } else {
            PathBuf::new()
        };
        full_path.push(&name);

        if let NodeType::Directory = node_type {
            path_stack.push((depth, full_path.clone()));
        }

        nodes.push(Node {
            name,
            depth,
            node_type,
            path: full_path,
        });
    }
    nodes
}
