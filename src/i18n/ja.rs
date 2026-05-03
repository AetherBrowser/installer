use std::collections::HashMap;
pub fn strings(app_name: &str) -> HashMap<&'static str, String> {

  HashMap::from([
    ("app_title", format!("{} インストーラー", app_name)),
    ("loading", "読み込み中...".into()),
    ("license_title", "使用許諾契約".into()),
    ("license_subtitle", "続行する前に以下の重要な情報をお読みください".into()),
    ("license_body", "以下の使用許諾契約をよくお読みください。インストールを続\
                     行するには、この契約の条件に同意する必要があります。".into()),
    ("accept", "同意する".into()),
    ("decline", "拒否する".into()),
    ("downloading", "ダウンロード中...".into()),
    ("verifying", "確認中...".into()),
    ("download_ok", "ダウンロード完了".into()),
    ("finished_title", "インストール完了".into()),
    ("failed_title", "インストール失敗".into()),
    ("close", "閉じる".into()),
    ("trying", "接続中".into()),
    ("err_checksum", "チェックサムエラー".into()),
    ("err_all_failed", "すべてのサーバーが失敗しました".into()),
    ("err_no_length", "サイズ不明".into()),
  ])
}
