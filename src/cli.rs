use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// 実行前に作成予定リストを表示する
    #[arg(short, long)]
    pub preview: bool,

    /// 確認をスキップして即座に実行する
    #[arg(short, long, alias = "f")]
    pub yes: bool,

    /// シミュレーション結果の表示のみを行い、ファイル作成は行わない
    #[arg(short = 'n', long)]
    pub dry_run: bool,

    /// 設定ファイルのパスを指定する
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<String>,

    /// ログレベルを指定する (error, warn, info, debug, trace)
    #[arg(short, long, default_value = "info")]
    pub log_level: String,
}
