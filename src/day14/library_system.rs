// ========================================
// 통합 과제 1: 도서 관리 시스템 (45분)
// ========================================
// 2주차에 배운 모든 개념을 활용하여 도서 관리 시스템을 만드세요.
//
// 요구사항:
//
// 1. LibraryError 열거형 (11일차: 커스텀 에러)
//    - BookNotFound(u32): 책을 찾을 수 없음 (책 ID 포함)
//    - BookAlreadyExists(u32): 이미 존재하는 책
//    - InvalidOperation(String): 잘못된 작업
//
// 2. Displayable 트레잇 (13일차: 트레잇)
//    - display_info(&self) -> String
//
// 3. Book 구조체
//    - id: u32
//    - title: String
//    - author: String
//    - available: bool (대출 가능 여부)
//    - #[derive(Debug, Clone)]
//    - Displayable 트레잇 구현
//      * "{title} by {author} (대출가능/대출중)" 형식
//
// 4. Library<T: Displayable + Clone> 구조체 (12일차: 제네릭)
//    - books: HashMap<u32, T> (9일차: HashMap)
//
// 5. impl<T: Displayable + Clone> Library<T>
//    - new() -> Library<T>
//      * 빈 도서관 생성
//
//    - add_book(&mut self, id: u32, book: T) -> Result<(), LibraryError>
//      * 책 추가, 이미 존재하면 에러
//      * 힌트: if self.books.contains_key(&id) { return Err(...) }
//
//    - get_book(&self, id: u32) -> Result<&T, LibraryError>
//      * ID로 책 찾기, 없으면 에러
//      * 힌트: self.books.get(&id).ok_or(LibraryError::BookNotFound(id))
//
//    - remove_book(&mut self, id: u32) -> Result<T, LibraryError>
//      * 책 제거, 없으면 에러
//      * 힌트: self.books.remove(&id).ok_or(...)
//
//    - list_all_books(&self) -> Vec<String>
//      * 모든 책의 정보를 문자열 벡터로 반환
//      * 힌트: self.books.values().map(|book| book.display_info()).collect()
//
//    - count(&self) -> usize
//      * 전체 책 개수

use std::collections::HashMap;

#[derive(Debug)]
enum LibraryError {
    BookNotFound(u32),
    BookAlreadyExists(u32),
    InvalidOperation(String),
}

// TODO: Displayable 트레잇을 정의하세요
trait Displayable {
    fn display_info(&self) -> String;
}

// TODO: Book 구조체를 정의하세요
#[derive(Debug, Clone)]
struct Book {
    id: u32,
    title: String,
    author: String,
    available: bool,
}

// TODO: Book에 Displayable 트레잇을 구현하세요
impl Displayable for Book {
    fn display_info(&self) -> String {
        let status_kr = if self.available {
            "대출가능"
        } else {
            "대출중"
        };

        format!("{} by {} ({})", self.title, self.author, status_kr)
    }
}

// TODO: Library 구조체를 정의하세요
struct Library<T: Displayable + Clone> {
    books: HashMap<u32, T>,
}

// TODO: Library의 모든 메서드를 구현하세요
impl<T: Displayable + Clone> Library<T> {
    fn new() -> Library<T> {
        Library {
            books: HashMap::new(),
        }
    }

    fn add_book(&mut self, id: u32, book: T) -> Result<(), LibraryError> {
        if self.books.insert(id, book).is_some() {
            Err(LibraryError::BookAlreadyExists(id))
        } else {
            Ok(())
        }
    }

    fn get_book(&self, id: u32) -> Result<&T, LibraryError> {
        self.books.get(&id).ok_or(LibraryError::BookNotFound(id))
    }

    fn remove_book(&mut self, id: u32) -> Result<T, LibraryError> {
        self.books.remove(&id).ok_or(LibraryError::BookNotFound(id))
    }

    fn list_all_books(&self) -> Vec<String> {
        self.books
            .values()
            .map(|book| book.display_info())
            .collect()
    }

    fn count(&self) -> usize {
        self.books.len()
    }
}

pub fn run() {
    println!("=== 통합 과제 1: 도서 관리 시스템 ===\n");

    let mut library = Library::new();

    // 책 추가
    let book1 = Book {
        id: 1,
        title: String::from("Rust 프로그래밍"),
        author: String::from("Steve Klabnik"),
        available: true,
    };

    let book2 = Book {
        id: 2,
        title: String::from("The Rust Programming Language"),
        author: String::from("Carol Nichols"),
        available: false,
    };

    match library.add_book(1, book1.clone()) {
        Ok(_) => println!("책 추가 성공: {}", book1.display_info()),
        Err(e) => println!("에러: {:?}", e),
    }

    match library.add_book(2, book2.clone()) {
        Ok(_) => println!("책 추가 성공: {}", book2.display_info()),
        Err(e) => println!("에러: {:?}", e),
    }

    // 중복 추가 시도
    match library.add_book(1, book1.clone()) {
        Ok(_) => println!("책 추가 성공"),
        Err(e) => println!("에러: {:?}", e),
    }

    println!("\n전체 도서 목록:");
    for info in library.list_all_books() {
        println!("- {}", info);
    }

    println!("\n전체 책 개수: {}", library.count());

    // 책 찾기
    println!("\n책 검색:");
    match library.get_book(1) {
        Ok(book) => println!("찾은 책: {}", book.display_info()),
        Err(e) => println!("에러: {:?}", e),
    }

    match library.get_book(999) {
        Ok(book) => println!("찾은 책: {}", book.display_info()),
        Err(e) => println!("에러: {:?}", e),
    }

    // 책 제거
    println!("\n책 제거:");
    match library.remove_book(1) {
        Ok(book) => println!("제거된 책: {}", book.display_info()),
        Err(e) => println!("에러: {:?}", e),
    }

    println!("\n제거 후 전체 책 개수: {}", library.count());
}
