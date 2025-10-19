// ========================================
// 과제 1: 불변 참조 사용하기 (15분)
// ========================================
// 문자열의 첫 번째 단어를 찾아 그 길이를 반환하는 함수를 작성하세요.
//
// 요구사항:
// - 함수는 String의 참조(&String)를 받습니다
// - 공백(' ')을 기준으로 첫 번째 단어를 찾습니다
// - 첫 번째 단어의 길이를 반환합니다
// - 공백이 없으면 전체 문자열의 길이를 반환합니다
// - 원본 문자열은 수정하지 않습니다
//
// 힌트:
// - s.chars()로 문자를 순회할 수 있습니다
// - enumerate()를 사용하면 인덱스를 얻을 수 있습니다
// - for (i, ch) in s.chars().enumerate() { ... }

pub fn first_word_length(s: &String) -> usize {
    // TODO: 여기에 코드를 작성하세요

    for (i, ch) in s.chars().enumerate() {
        if ch == ' ' {
            return i;
        }
    }

    s.len()
}

pub fn run() {
    println!("=== 과제 1: 불변 참조 사용하기 ===");
    let sentence = String::from("Hello Rust Programming");
    // TODO: first_word_length 함수를 호출하고 결과를 출력하세요
    // 함수 호출 후 sentence를 다시 출력해서 원본이 유지되는지 확인하세요
    let length = first_word_length(&sentence);
    println!("첫 번째 단어의 길이: {}", length);
    println!("원본 문자열: {}", sentence);
}
