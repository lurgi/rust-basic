// ========================================
// 과제 3: 문자열 검증과 추출 (20분)
// ========================================
// 이메일 주소에서 사용자 이름 부분을 추출하는 함수를 작성하세요.
//
// 요구사항:
// - 함수는 &str을 매개변수로 받습니다
// - '@' 기호가 있으면 그 앞부분을 반환합니다
// - '@' 기호가 없으면 None을 반환합니다
// - 반환 타입은 Option<&str>입니다
//
// 예시:
// - "user@example.com" → Some("user")
// - "invalid-email" → None
// - "@example.com" → Some("") (빈 문자열도 유효)
//
// 힌트:
// - s.find('@')로 '@'의 위치를 찾을 수 있습니다
// - find는 Option<usize>를 반환합니다
// - match나 if let으로 Option을 처리하세요
// - 슬라이스 [0..index]를 사용하세요

#[allow(dead_code)]
fn extract_username(_email: &str) -> Option<&str> {
    // TODO: 여기에 코드를 작성하세요
    let index = _email.find('@');
    if index.is_some() {
        return Some(&_email[0..index.unwrap()]);
    } else {
        return None;
    }
}

pub fn run() {
    println!("=== 과제 3: 문자열 검증과 추출 ===");
    let _email1 = "user@example.com";
    // TODO: extract_username 함수를 호출하고 결과를 출력하세요
    // 힌트: Option 출력은 {:?} 포맷을 사용하거나
    // match로 Some/None을 처리하세요
    println!("{:?}", extract_username(_email1));
    let _email2 = "invalid-email";
    // TODO: '@'가 없는 경우도 테스트하세요
    println!("{:?}", extract_username(_email2));
    let _email3 = "@example.com";
    // TODO: 사용자 이름이 빈 경우도 테스트하세요
    println!("{:?}", extract_username(_email3));
}
