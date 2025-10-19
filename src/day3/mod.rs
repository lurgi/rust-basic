// 3일차 과제들
mod find_min_max;
mod immutable_reference;
mod mutable_reference;

// 전체 실행 함수
#[allow(dead_code)]
pub fn run_day3_exercises() {
    immutable_reference::run();
    println!();
    mutable_reference::run();
    println!();
    find_min_max::run();
}
