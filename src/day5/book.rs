// ========================================
// 과제 1: Book 구조체 만들기 (20분)
// ========================================
// 책 정보를 저장하는 구조체를 만들고 관련 메서드를 구현하세요.
//
// 요구사항:
// - Book 구조체 정의
//   - title: String
//   - author: String
//   - pages: u32
//   - available: bool
//
// - 다음 메서드들을 impl 블록에 구현:
//   1. new (연관 함수): title, author, pages를 받아 Book 생성
//      - available은 항상 true로 초기화
//
//   2. description (&self): 책 정보를 문자열로 반환
//      - 형식: "제목 by 저자 (페이지수 pages)"
//      - 예: "Rust Programming by Steve Klabnik (500 pages)"
//
//   3. borrow (&mut self): 책을 대출
//      - available을 false로 변경
//      - 이미 대출 중이면 false 반환, 성공하면 true 반환
//
//   4. return_book (&mut self): 책을 반납
//      - available을 true로 변경

// TODO: Book 구조체를 정의하세요
#[derive(Debug)]
struct Book {
    // TODO: 필드들을 정의하세요
}

// TODO: impl 블록에 메서드들을 구현하세요
impl Book {
    // TODO: new 연관 함수

    // TODO: description 메서드

    // TODO: borrow 메서드

    // TODO: return_book 메서드
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_book_creation() {
        // TODO: Book::new로 책을 생성하고 테스트하세요
    }

    #[test]
    fn test_book_description() {
        // TODO: description 메서드를 테스트하세요
    }

    #[test]
    fn test_book_borrow() {
        // TODO: borrow 메서드를 테스트하세요
    }

    #[test]
    fn test_book_return() {
        // TODO: return_book 메서드를 테스트하세요
    }
}

pub fn run() {
    println!("=== 과제 1: Book 구조체 ===");
    // TODO: Book 테스트
    // 1. Book::new로 책 생성
    // 2. description 출력
    // 3. borrow 호출하고 결과 출력
    // 4. 다시 borrow 호출 (이미 대출 중)
    // 5. return_book 호출
    // 6. 다시 borrow 호출 (이제 가능)
}
