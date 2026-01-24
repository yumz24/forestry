mod cli;
use clap::Parser;
use cli::Args;

fn main() {
    let args = Args::parse();

    println!("Preview mode: {}", args.preview);
    println!("Auto-confirm (yes): {}", args.yes);
    println!("Dry run: {}", args.dry_run);
}
