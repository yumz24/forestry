use crate::node::{Node, NodeType};
use crate::config::Config;
use anyhow::{Context, Result};
use log::{info, warn};
use std::fs;
use std::os::unix::fs as unix_fs;
use std::io::Write;

/// Nodeのリストから実際のディレクトリとファイルを生成する
pub fn generate(nodes: &[Node], config: &Config) -> Result<()> {
    for node in nodes {
        let path_str = node.path.to_string_lossy();

        // 無視設定のチェック
        if config.is_ignored(&path_str) {
            info!("IGNORED: {}", path_str);
            continue;
        }

        match &node.node_type {
            NodeType::Directory => {
                fs::create_dir_all(&node.path).with_context(|| {
                    format!("ディレクトリの作成に失敗しました: {:?}", node.path)
                })?;
                info!("CREATED: {}/", node.path.display());
            }
            NodeType::File => {
                if node.path.exists() && !config.overwrite {
                    warn!("SKIPPED (exists): {}", path_str);
                    continue;
                }

                // 親ディレクトリを念の為作成
                if let Some(parent) = node.path.parent() {
                    fs::create_dir_all(parent)?;
                }

                let mut file = fs::File::create(&node.path).with_context(|| {
                    format!("ファイルの作成に失敗しました: {:?}", node.path)
                })?;
    
                if let Some(template_map) = &config.templates {
                    // 拡張子を取得（ttt.rs なら "rs"）
                    if let Some(ext) = node.path.extension().and_then(|e| e.to_str()) {
                        if let Some(raw_template) = template_map.get(ext) {
                            // 変数展開 (${DATE}など)
                            let resolved = config.resolve_template(raw_template);
                            
                            // 確実に書き込みを実行
                            file.write_all(resolved.as_bytes()).with_context(|| {
                                format!("テンプレートの書き込みに失敗しました: {:?}", node.path)
                            })?;
                            
                            // 念のためディスクに同期
                            file.sync_all()?;
                            info!("TEMPLATE APPLIED: {}", path_str);
                        }
                    }
                }

                info!("CREATED: {}", node.path.display());
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
