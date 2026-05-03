use std::collections::HashMap;
pub fn strings(app_name: &str) -> HashMap<&'static str, String> {

  HashMap::from([
    ("app_title", format!("{} 安装程序", app_name)),
    ("loading", "加载中...".into()),
    ("license_title", "许可协议".into()),
    ("license_subtitle", "在继续之前，请阅读以下重要信息".into()),
    ("license_body",
     "请仔细阅读以下许可协议。在继续安装之前，您必须接受本协议的条款。".into()),
    ("accept", "接受".into()),
    ("decline", "拒绝".into()),
    ("downloading", "下载中...".into()),
    ("verifying", "验证中...".into()),
    ("download_ok", "下载完成".into()),
    ("finished_title", "安装完成".into()),
    ("failed_title", "安装失败".into()),
    ("close", "关闭".into()),
    ("trying", "正在连接".into()),
    ("err_checksum", "校验和错误".into()),
    ("err_all_failed", "所有服务器均失败".into()),
    ("err_no_length", "未知大小".into()),
  ])
}
