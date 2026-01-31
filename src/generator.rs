use crate::node::{Node, NodeType};
use anyhow::{Context, Result};
use std::fs;
use std::os::unix::fs as unix_fs;
use log::info;

/// Nodeのリストから実際のディレクトリとファイルを生成する
pub fn generate(nodes: &[Node]) -> Result<()> {
    for node in nodes {
        match &node.node_type {
            NodeType::Directory => {
                fs::create_dir_all(&node.path).with_context(|| {
                    format!("ディレクトリの作成に失敗しました: {:?}", node.path)
                })?;
                info!("CREATED: {}/", node.path.display());
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
                    info!("CREATED: {}", node.path.display());
                } else {
                    info!("EXISTS (SKIPPED): {}", node.path.display());
                }
            }
            NodeType::Symlink { target } => {
                if !node.path.exists() {
                    unix_fs::symlink(target, &node.path).with_context(|| {
                        format!("リンク作成失敗: {:?} -> {}", node.path, target)
                    })?;
                    info!("CREATED LINK: {} -> {}", node.path.display(), target);
                }
            }
        }
    }
    Ok(())
}
