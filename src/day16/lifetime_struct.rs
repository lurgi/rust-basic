// ========================================
// 과제 2: 생명주기가 있는 구조체
// ========================================
// 참조를 포함하는 구조체를 만들고 메서드를 구현하세요.
//
// 요구사항:
// - TextSlice<'a> 구조체
//   - content: &'a str
//   - start: usize
//   - end: usize
//
// - impl<'a> TextSlice<'a>
//   - new(content: &'a str, start: usize, end: usize) -> TextSlice<'a>
//     * TextSlice 생성
//
//   - text(&self) -> &str
//     * content[start..end] 반환
//     * 생명주기 생략 규칙이 적용됨
//
//   - len(&self) -> usize
//     * end - start 반환
//
//   - concat(&self, othecr: &str) -> String
//     * self.text()와 other를 합친 새 String 반환
//     * 힌트: format!("{}{}", self.text(), other)

// TODO: TextSlice 구조체를 정의하세요
struct TextSlice<'a> {
    content: &'a str,
    start: usize,
    end: usize,
}

// TODO: TextSlice의 모든 메서드를 구현하세요
impl<'a> TextSlice<'a> {
    fn new(content: &'a str, start: usize, end: usize) -> TextSlice<'a> {
        TextSlice {
            content,
            start,
            end,
        }
    }

    fn text(&self) -> &str {
        &self.content[self.start..self.end]
    }

    fn len(&self) -> usize {
        self.end - self.start
    }

    fn concat(&self, other: &str) -> String {
        format!("{}{}", self.text(), other)
    }
}

pub fn run() {
    println!("=== 과제 2: 생명주기가 있는 구조체 ===");

    let content = String::from("Hello, Rust Programming!");
    let slice = TextSlice::new(&content, 7, 11);

    println!("슬라이스 텍스트: {}", slice.text());
    println!("슬라이스 길이: {}", slice.len());

    let combined = slice.concat(" Language");
    println!("결합된 텍스트: {}", combined);
}
