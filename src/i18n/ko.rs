use std::collections::HashMap;
pub fn strings(app_name: &str) -> HashMap<&'static str, String> {

  HashMap::from([
    ("app_title", format!("{} 설치 프로그램", app_name)),
    ("loading", "로딩 중...".into()),
    ("license_title", "라이선스 계약".into()),
    ("license_subtitle", "계속하기 전에 다음 중요 정보를 읽어 주세요".into()),
    ("license_body", "다음 라이선스 계약을 주의 깊게 읽어 주세요. 설치를 \
                     계속하려면 이 계약의 조건에 동의해야 합니다.".into()),
    ("accept", "동의".into()),
    ("decline", "거부".into()),
    ("downloading", "다운로드 중...".into()),
    ("verifying", "확인 중...".into()),
    ("download_ok", "다운로드 완료".into()),
    ("finished_title", "설치 완료".into()),
    ("failed_title", "설치 실패".into()),
    ("close", "닫기".into()),
    ("trying", "연결 중".into()),
    ("err_checksum", "체크섬 오류".into()),
    ("err_all_failed", "모든 서버 실패".into()),
    ("err_no_length", "알 수 없는 크기".into()),
  ])
}
