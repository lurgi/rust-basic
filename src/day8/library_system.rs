// ========================================
// 과제 1: 도서관 시스템 모듈 (20분)
// ========================================
// 도서관 시스템을 모듈로 구조화하세요.
//
// 요구사항:
// - library 모듈 생성
//   - books 하위 모듈
//     - pub struct Book { pub title: String, pub author: String }
//     - pub fn create_book(title: String, author: String) -> Book
//
//   - members 하위 모듈
//     - pub struct Member { pub name: String, pub id: u32 }
//     - pub fn create_member(name: String, id: u32) -> Member
//
// - main 함수에서 library::books와 library::members 사용
// - use를 사용하여 경로 단축

// TODO: library 모듈을 정의하세요
mod library {
    // TODO: books 모듈을 정의하세요
    pub mod books {
        // TODO: Book 구조체와 create_book 함수를 구현하세요
    }

    // TODO: members 모듈을 정의하세요
    pub mod members {
        // TODO: Member 구조체와 create_member 함수를 구현하세요
    }
}

pub fn run() {
    println!("=== 과제 1: 도서관 시스템 모듈 ===");
    // TODO: library::books와 library::members 사용
    // 1. use로 경로 단축
    // 2. Book과 Member 생성
    // 3. 정보 출력
}
