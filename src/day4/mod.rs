// 4일차 과제들
mod extract_username;
mod last_word;
mod title_case;

// 전체 실행 함수
#[allow(dead_code)]
pub fn run_day4_exercises() {
    last_word::run();
    println!();
    title_case::run();
    println!();
    extract_username::run();
}
