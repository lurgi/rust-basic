mod day1;
mod day2;
mod day3;

// 각 날짜별 전체 실행 함수 불러오기
#[allow(unused_imports)]
use day1::run_day1_exercises;
#[allow(unused_imports)]
use day2::run_day2_exercises;
#[allow(unused_imports)]
use day3::run_day3_exercises;

fn main() {
    // 학습할 과제를 선택해서 실행하세요
    // run_day1_exercises();
    // run_day2_exercises();
    run_day3_exercises();
}
