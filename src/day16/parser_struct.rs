// ========================================
// 과제 4: 파서 구조체
// ========================================
// 문자열을 파싱하는 구조체를 만드세요.
//
// 요구사항:
// - Parser<'a> 구조체
//   - input: &'a str
//   - position: usize
//
// - impl<'a> Parser<'a>
//   - new(input: &'a str) -> Parser<'a>
//     * position은 0으로 초기화
//
//   - current_char(&self) -> Option<char>
//     * 현재 위치의 문자 반환
//     * 힌트: self.input.chars().nth(self.position)
//
//   - advance(&mut self)
//     * position을 1 증가
//
//   - remaining(&self) -> &str
//     * 현재 위치부터 끝까지의 문자열 반환
//     * 힌트: &self.input[self.position..]
//
//   - is_finished(&self) -> bool
//     * position >= input.len()

use std::thread::sleep;

// TODO: Parser 구조체를 정의하세요
struct Parser<'a> {
    input: &'a str,
    position: usize,
}

// TODO: Parser의 모든 메서드를 구현하세요
impl<'a> Parser<'a> {
    fn new(input: &'a str) -> Parser<'a> {
        Parser { input, position: 0 }
    }

    fn current_char(&self) -> Option<char> {
        self.input.chars().nth(self.position)
    }

    fn advance(&mut self) {
        self.position = self.position + 1
    }

    fn remaining(&self) -> &str {
        &self.input[self.position..]
    }

    fn is_finished(&self) -> bool {
        self.position >= self.input.len()
    }
}

pub fn run() {
    println!("=== 과제 4: 파서 구조체 ===");

    let input = String::from("Hello");
    let mut parser = Parser::new(&input);

    while !parser.is_finished() {
        if let Some(ch) = parser.current_char() {
            println!("현재 문자: {}, 남은 문자열: {}", ch, parser.remaining());
        }
        parser.advance();
    }

    println!("파싱 완료!");
}
