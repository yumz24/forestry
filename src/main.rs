mod cli;
mod config;
mod editor;
mod generator;
mod node;
mod parser;

use anyhow::Result;
use clap::Parser;
use cli::Args;
use config::Config;
use dialoguer::Confirm;
use log::{debug, info, warn};

fn main() -> Result<()> {
    let args = Args::parse();

    // ロガーの初期化(RUST_LOG環境変数または引数のlog_levelを使用)
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(&args.log_level))
        .format_timestamp(None)
        .init();

    let _config = Config::load();

    let input = editor::capture_input_from_editor()?;

    if input.trim().is_empty() {
        println!("入力が空だったため、終了します");
        return Ok(());
    }

    // パース実行
    let nodes = parser::parse_input(&input);

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

    if !args.yes {
        let confirmation = Confirm::new()
            .with_prompt("ファイルを作成してもよろしいですか?")
            .default(true)
            .interact()?;

        if !confirmation {
            info!("キャンセルされました。");
            return Ok(());
        }
    }

    generator::generate(&nodes)?;

    info!("\nすべての処理が完了しました。🌲");
    Ok(())
}
