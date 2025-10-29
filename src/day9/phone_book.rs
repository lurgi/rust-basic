// ========================================
// 과제 3: 전화번호부 (20분)
// ========================================
// HashMap을 사용하여 전화번호부를 구현하세요.
//
// 요구사항:
// - PhoneBook 구조체
//   - contacts: HashMap<String, String>
//
// - impl PhoneBook
//   - new() -> PhoneBook
//     * 빈 전화번호부 생성
//
//   - add(&mut self, name: String, phone: String)
//     * 연락처 추가
//
//   - get(&self, name: &str) -> Option<&String>
//     * 이름으로 전화번호 찾기
//
//   - remove(&mut self, name: &str) -> bool
//     * 연락처 삭제
//     * 성공하면 true, 없으면 false
//
//   - list_all(&self) -> Vec<(&String, &String)>
//     * 모든 연락처를 벡터로 반환
//     * (이름, 전화번호) 튜플의 벡터

use std::collections::HashMap;

// TODO: PhoneBook 구조체를 정의하세요
struct PhoneBook {
    // TODO: contacts 필드를 추가하세요
    #[allow(dead_code)]
    contacts: HashMap<String, String>,
}

// TODO: PhoneBook의 impl 블록을 작성하세요
impl PhoneBook {
    // TODO: new 메서드를 구현하세요
    fn new() -> PhoneBook {
        PhoneBook {
            contacts: HashMap::new(),
        }
    }

    // TODO: add 메서드를 구현하세요
    fn add(&mut self, _name: String, _phone: String) {
        self.contacts.insert(_name, _phone);
    }

    // TODO: get 메서드를 구현하세요
    fn get(&self, _name: &str) -> Option<&String> {
        self.contacts.get(_name)
    }

    // TODO: remove 메서드를 구현하세요
    fn remove(&mut self, _name: &str) -> bool {
        self.contacts.remove(_name).is_some()
    }

    // TODO: list_all 메서드를 구현하세요
    fn list_all(&self) -> Vec<(&String, &String)> {
        let mut contacts_vec = Vec::new();

        for (name, phone) in &self.contacts {
            contacts_vec.push((name, phone));
        }

        contacts_vec

        // or self.contacts.iter().collect()
    }
}

pub fn run() {
    println!("=== 과제 3: 전화번호부 ===");
    let mut phone_book = PhoneBook::new();

    // TODO: 연락처 추가
    phone_book.add(String::from("Alice"), String::from("010-1234-5678"));
    phone_book.add(String::from("Bob"), String::from("010-2345-6789"));
    phone_book.add(String::from("Charlie"), String::from("010-3456-7890"));

    // TODO: 연락처 조회
    if let Some(phone) = phone_book.get("Alice") {
        println!("Alice의 전화번호: {}", phone);
    }

    // TODO: 모든 연락처 출력
    println!("\n모든 연락처:");
    for (name, phone) in phone_book.list_all() {
        println!("  {} : {}", name, phone);
    }

    // TODO: 연락처 삭제
    if phone_book.remove("Bob") {
        println!("\nBob의 연락처가 삭제되었습니다.");
    }

    // TODO: 삭제 후 연락처 출력
    println!("\n삭제 후 연락처:");
    for (name, phone) in phone_book.list_all() {
        println!("  {} : {}", name, phone);
    }
}
