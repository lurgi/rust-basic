// 2일차 과제들
mod ownership_move;
mod string_concat;
mod vector_filter;

// 전체 실행 함수
#[allow(dead_code)]
pub fn run_day2_exercises() {
    ownership_move::run();
    println!();
    string_concat::run();
    println!();
    vector_filter::run();
}
