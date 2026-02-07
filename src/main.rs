mod cli;
mod config;
mod editor;
mod generator;
mod node;
mod parser;

use anyhow::Result;
use clap::Parser;
use cli::Args;
use dialoguer::Confirm;
use log::{debug, info};

fn main() -> Result<()> {
    let args = Args::parse();

    // 設定ファイルの読み込み( 引数でパスが指定されていればそれを使用する )
    let config = config::Config::load(args.config.clone());

    // ログレベルの設定( 引数 > 設定ファイル > デフォルト"info" )
    let log_level = if args.log_level != "info" {
        args.log_level.clone()
    } else {
        config.log_level.clone().unwrap_or_else(|| "info".to_string())
    };

    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(&log_level))
        .format_timestamp(None)
        .init();

    let input = editor::capture_input_from_editor()?;

    if input.trim().is_empty() {
        println!("入力が空だったため、終了します");
        return Ok(());
    }

    // パース実行
    let nodes = parser::parse_input(&input);

    if nodes.is_empty() {
        info!("有効なディレクトリ構造が見つからなかったため、終了します。");
        return Ok(());
    }

    debug!("Parsed {} nodes", nodes.len());

    // プレビュー表示
    println!("\n以下の構成で作成を開始します");
    for node in &nodes {
        let prefix = " ".repeat(node.depth);
        let icon = match &node.node_type {
            node::NodeType::Directory => "📁",
            node::NodeType::File => "📄",
            node::NodeType::Symlink { .. } => "🔗",
        };
        println!("{} {} {}", prefix, icon, node.name);
    }

    if args.dry_run {
        info!("\n[Dry-run] 実際には作成されません。");
        return Ok(());
    }

    let skip_confirm = args.yes || config.default_yes.unwrap_or(false);

    if !skip_confirm {
        let confirmation = Confirm::new()
            .with_prompt("ファイルを作成してもよろしいですか?")
            .default(true)
            .interact()?;

        if !confirmation {
            info!("キャンセルされました。");
            return Ok(());
        }
    }

    generator::generate(&nodes, &config)?;

    info!("\nすべての処理が完了しました。🌲");
    Ok(())
}
