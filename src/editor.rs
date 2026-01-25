use anyhow::{Context, Result};
use edit::edit;

/// エディタを起動してユーザーの入力を取得する
pub fn capture_input_from_editor() -> Result<String> {
    // 空のテキストでエディタを起動
    let template = "# Forestry: ここにディレクトリ構造を記述してください\n# '#' で始まる行は無視されます\n";
    
    let edited_context = edit(template)
        .context("エディタの起動または一時ファイルの保存に失敗しました")?;

    Ok(edited_context)
}
