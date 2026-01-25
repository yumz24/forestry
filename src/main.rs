mod cli;
mod editor;
mod node;
mod parser;

use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    let _args = cli::Args::parse();

    let input = editor::capture_input_from_editor()?;

    if input.trim().is_empty() {
        return Ok(());
    }

    // パース実行
    let nodes = parser::parse_input(&input);

    for node in nodes {
        println!(
            "{:?}: depth={}, name={}",
            node.node_type, node.depth, node.name
        );
    }

    Ok(())
}
