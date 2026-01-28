mod cli;
mod editor;
mod generator;
mod node;
mod parser;

use anyhow::Result;
use clap::Parser;
use cli::Args;
use dialoguer::Confirm;

fn main() -> Result<()> {
    let args = Args::parse();
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
        let icon = if let node::NodeType::Directory = node.node_type {
            "📁"
        } else {
            "📄"
        };
        println!("{} {} {}", prefix, icon, node.name);
    }

    if args.dry_run {
        println!("\n[Dry-run] 実際には作成されません。");
        return Ok(());
    }

    if !args.yes {
        let confirmation = Confirm::new()
            .with_prompt("ファイルを作成してもよろしいですか?")
            .default(true)
            .interact()?;

        if !confirmation {
            println!("キャンセルされました。");
            return Ok(());
        }
    }

    generator::generate(&nodes)?;

    println!("\nすべての処理が完了しました。🌲");
    Ok(())
}
