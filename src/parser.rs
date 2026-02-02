use crate::node::{Node, NodeType};
use std::path::PathBuf;

pub fn parse_input(input: &str) -> Vec<Node> {
    let mut nodes = Vec::new();
    let mut stack: Vec<(usize, PathBuf)> = Vec::new(); // (depth, current_path)
    let mut indent_unit = None; // 1レベルあたりのインデント幅を保持

    for line in input.lines() {
        // 1. クリーニングとスキップ判定
        if line.trim().is_empty() || line.trim().starts_with('#') {
            continue;
        }

        // 罫線の除去
        let clean_line = line
            .replace('│', "")
            .replace('├', "")
            .replace('└', "")
            .replace('─', "");

        let trimmed = clean_line.trim_start();
        let indent_len = clean_line.len() - trimmed.len();

        // インデント幅の自動推定ロジック
        if indent_unit.is_none() && indent_len > 0 {
            indent_unit = Some(indent_len);
        }

        // 1レベルあたりの幅がわかれば割る、わからなければ0 ( トップレベル )
        let depth = match indent_unit {
            Some(unit) if unit > 0 => indent_len / unit,
            _ => 0,
        };

        let is_dir = trimmed.ends_with('/');

        // シンボリックリンクの判定
        let node_type = if trimmed.contains("->") {
            let parts: Vec<&str> = trimmed.split("->").map(|s| s.trim()).collect();
            let name = parts[0].to_string();
            let target = parts.get(1).unwrap_or(&"").to_string();
            (name, NodeType::Symlink { target })
        } else {
            let name = trimmed.trim_end_matches('/').to_string();
            let n_type = if is_dir {
                NodeType::Directory
            } else {
                NodeType::File
            };
            (name, n_type)
        };

        let (name, node_type) = node_type;

        // 2. パス解決ロジック (ディレクトリスタック)
        while let Some((stack_depth, _)) = stack.last() {
            if *stack_depth >= depth {
                stack.pop();
            } else {
                break;
            }
        }

        let mut current_path = if let Some((_, parent_path)) = stack.last() {
            parent_path.clone()
        } else {
            PathBuf::new()
        };
        current_path.push(&name);

        let mut node = Node::new(name.clone(), depth, node_type);
        node.path = current_path.clone();

        if let NodeType::Directory = node.node_type {
            stack.push((depth, current_path));
        }

        nodes.push(node);
    }
    nodes
}
