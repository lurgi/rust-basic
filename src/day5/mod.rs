// 5일차: 구조체(Structs)와 메서드
mod bank_account;
mod book;
mod point;

// 전체 실행 함수
#[allow(dead_code)]
pub fn run_day5_exercises() {
    book::run();
    bank_account::run();
    point::run();
}
