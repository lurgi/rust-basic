// ========================================
// 과제 2: 문자열 연결 함수 (20분)
// ========================================
// 두 개의 String을 받아서 연결한 새로운 String을 반환하는 함수를 작성하세요.
//
// 요구사항:
// - 함수는 두 개의 String을 매개변수로 받습니다
// - 두 문자열을 연결한 새로운 String을 반환합니다
// - 원본 문자열들의 소유권은 함수로 이동하며, 다시 돌려주지 않습니다
// - main에서 결과를 출력합니다
//
// 힌트:
// - format! 매크로를 사용할 수 있습니다: format!("{}{}", s1, s2)
// - 또는 String의 push_str 메서드를 사용할 수 있습니다

pub fn concatenate(mut s1: String, s2: String) -> String {
    // TODO: 여기에 코드를 작성하세요
    // format!("{}{}", s1, s2)
    s1.push_str(&s2);
    s1
}

pub fn run() {
    println!("=== 과제 2: 문자열 연결 함수 ===");
    let first = String::from("Hello, ");
    let second = String::from("Rust!");
    // TODO: concatenate 함수를 호출하고 결과를 출력하세요
    println!("연결된 문자열: {}", concatenate(first, second));
}
