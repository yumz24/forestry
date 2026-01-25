use crate::node::{Node, NodeType};

pub fn parse_input(input: &str) -> Vec<Node> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty() && !line.trim().starts_with("#"))
        .filter_map(|line| {
            // 1. 罫線文字の除去
            let clean_line = line
                .replace('│', "")
                .replace('├', "")
                .replace('└', "")
                .replace('─', "");
            // 2. インデントの深さの計算(スペース2つを1単位として仮定 - TODO: 自動指定に強化するかも
            let trimmed = clean_line.trim_start();
            let indent_size = clean_line.len() - trimmed.len();
            let depth = indent_size / 2;

            // 3. ディレクトリかファイルかの判定
            let is_dir = trimmed.ends_with('/');
            let name = trimmed.trim_end_matches('/').to_string();
            let node_type = if is_dir {
                NodeType::Directory
            } else {
                NodeType::File
            };

            Some(Node::new(name, depth, node_type))
        })
        .collect()
}
