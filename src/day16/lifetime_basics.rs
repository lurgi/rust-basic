// ========================================
// 과제 1: 기본 생명주기 함수
// ========================================
// 생명주기 어노테이션을 사용하여 참조를 반환하는 함수를 작성하세요.
//
// 요구사항:
// - longest<'a>(x: &'a str, y: &'a str) -> &'a str
//   * 두 문자열 중 더 긴 것을 반환
//
// - first<'a>(x: &'a str, y: &str) -> &'a str
//   * 항상 첫 번째 문자열을 반환
//   * y의 생명주기는 반환값과 무관
//
// - contains_substring<'a, 'b>(text: &'a str, pattern: &'b str) -> Option<&'a str>
//   * text에 pattern이 포함되어 있으면 Some(text)
//   * 없으면 None
//   * 힌트: text.contains(pattern) 사용

// TODO: longest 함수를 구현하세요
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// TODO: first 함수를 구현하세요
fn first<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

// TODO: contains_substring 함수를 구현하세요
fn contains_substring<'a>(text: &'a str, pattern: &str) -> Option<&'a str> {
    if text.contains(pattern) {
        Some(text)
    } else {
        None
    }
}

pub fn run() {
    println!("=== 과제 1: 기본 생명주기 함수 ===");

    let string1 = String::from("hello");
    let string2 = String::from("world!");

    let result = longest(&string1, &string2);
    println!("더 긴 문자열: {}", result);

    let result = first(&string1, &string2);
    println!("첫 번째 문자열: {}", result);

    match contains_substring(&string1, "ell") {
        Some(text) => println!("'ell'을 포함: {}", text),
        None => println!("'ell'을 포함하지 않음"),
    }

    match contains_substring(&string1, "xyz") {
        Some(text) => println!("'xyz'를 포함: {}", text),
        None => println!("'xyz'를 포함하지 않음"),
    }
}
