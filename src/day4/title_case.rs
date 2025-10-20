// ========================================
// 과제 2: 문자열 변환 (20분)
// ========================================
// 문자열의 각 단어의 첫 글자를 대문자로 바꾸는 함수를 작성하세요.
// (Title Case 변환)
//
// 요구사항:
// - 함수는 &str을 매개변수로 받습니다
// - 각 단어의 첫 글자를 대문자로, 나머지는 소문자로 변환
// - 새로운 String을 반환합니다
// - 단어는 공백으로 구분됩니다
//
// 힌트:
// - split_whitespace()로 단어를 분리할 수 있습니다
// - chars()로 문자를 순회할 수 있습니다
// - enumerate()로 인덱스를 얻을 수 있습니다
// - 첫 번째 문자는 to_uppercase(), 나머지는 to_lowercase()

#[allow(dead_code)]
fn to_title_case(_s: &str) -> String {
    // TODO: 여기에 코드를 작성하세요
    let mut result = Vec::<String>::new();
    let words = _s.split_whitespace();

    for word in words {
        let mut new_word = String::new();
        for (i, ch) in word.chars().enumerate() {
            if i == 0 {
                for c in ch.to_uppercase() {
                    new_word.push(c);
                }
            } else {
                for c in ch.to_lowercase() {
                    new_word.push(c);
                }
            }
        }
        result.push(new_word);
    }

    result.join(" ")
}

pub fn run() {
    println!("=== 과제 2: 문자열 변환 ===");
    let _text = "hello rust programming";
    // TODO: to_title_case 함수를 호출하고 결과를 출력하세요
    println!("{}", to_title_case(_text));
    let _mixed = "HELLO world RuSt";
    // TODO: 대소문자가 섞인 경우도 테스트하세요
    println!("{}", to_title_case(_mixed));
}
