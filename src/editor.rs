use anyhow::{Context, Result};
use edit::edit;
use std::env;

/// エディタを起動してユーザーの入力を取得する
pub fn capture_input_from_editor() -> Result<String> {
    // 空のテキストでエディタを起動
    let template = r#"# Forestry: ここにディレクトリ構造を記述してください
# '#' で始まる行は無視されます
    "#;
    // 1. $EDITOR が設定されているか確認
    // 2. 設定されていなければ、一時的に "vi" をデフォルトに設定して edit を実行
    if env::var("EDITOR").is_err() {
        // システムにviがあることを期待して環境変数を一時的にセット
        unsafe {
            env::set_var("EDITOR", "vi");
        }
    }

    let edited_context =
        edit(template).context("エディタの起動または一時ファイルの保存に失敗しました")?;

    Ok(edited_context)
}
