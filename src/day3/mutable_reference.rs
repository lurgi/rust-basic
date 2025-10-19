// ========================================
// 과제 2: 가변 참조 사용하기 (20분)
// ========================================
// 문자열의 모든 소문자를 대문자로 바꾸는 함수를 작성하세요.
//
// 요구사항:
// - 함수는 String의 가변 참조(&mut String)를 받습니다
// - 원본 문자열을 직접 수정합니다
// - 반환값은 없습니다
// - main에서 함수 호출 전후의 문자열을 출력합니다
//
// 힌트:
// - ch.is_lowercase()로 소문자인지 확인할 수 있습니다
// - ch.to_uppercase()로 대문자로 변환할 수 있습니다
// - String을 직접 수정하려면 clear()한 후 push()를 사용하거나
// - 새로운 String을 만들어서 원본과 교체할 수 있습니다

pub fn to_uppercase_inplace(s: &mut String) {
    // TODO: 여기에 코드를 작성하세요
    *s = s.to_uppercase();
}

pub fn run() {
    println!("=== 과제 2: 가변 참조 사용하기 ===");
    let mut text = String::from("hello rust");
    println!("변경 전: {}", text);
    // TODO: to_uppercase_inplace 함수를 호출하세요
    to_uppercase_inplace(&mut text);
    println!("변경 후: {}", text);
}
