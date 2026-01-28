use crate::node::{Node, NodeType};
use anyhow::{Context, Result};
use std::fs;

/// Nodeのリストから実際のディレクトリとファイルを生成する
pub fn generate(nodes: &[Node]) -> Result<()> {
    for node in nodes {
        match node.node_type {
            NodeType::Directory => {
                // ディレクトリの作成
                fs::create_dir_all(&node.path).with_context(|| {
                    format!("ディレクトリの作成に失敗しました: {:?}", node.path)
                })?;
                println!("CREATED: {}/", node.path.display());
            }
            NodeType::File => {
                // 親ディレクトリを念の為作成
                if let Some(parent) = node.path.parent() {
                    fs::create_dir_all(parent)?;
                }

                // ファイルが存在しない場合のみからファイルを作成
                if !node.path.exists() {
                    fs::File::create(&node.path).with_context(|| {
                        format!("ファイルの作成に失敗しました: {:?}", node.path)
                    })?;
                    println!("CREATED: {}", node.path.display());
                } else {
                    println!("EXISTS (SKIPPED): {}", node.path.display());
                }
            }
        }
    }
    Ok(())
}
