mod cli;
mod editor;

use clap::Parser;
use cli::Args;
use anyhow::Result;

fn main() -> Result<()> {
    let _args = Args::parse();

    let input = editor::capture_input_from_editor()?;

    if input.trim().is_empty() {
        println!("入力が空だったため、処理を中断します。");
    } else {
        println!("入力を受け取りました");
    }

    Ok(())
}
