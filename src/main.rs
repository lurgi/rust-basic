mod fibonacci;
mod fizzbuzz;
mod temperature;

// 모듈에서 함수들을 직접 불러오기
use fibonacci::fibonacci;
use fizzbuzz::fizzbuzz;
use temperature::{celsius_to_fahrenheit, fahrenheit_to_celsius};

// 과제 1: 온도 변환기 실행
fn run_temperature_converter() {
    println!("=== 과제 1: 온도 변환기 ===");
    let celsius = 25.0;
    let fahrenheit = 77.0;

    // TODO: celsius_to_fahrenheit와 fahrenheit_to_celsius 함수를 호출하고 결과를 출력하세요
}

// 과제 2: 피보나치 수열 실행
fn run_fibonacci() {
    println!("=== 과제 2: 피보나치 수열 ===");
    // TODO: 10번째 피보나치 수를 출력하세요
}

// 과제 3: FizzBuzz 실행
fn run_fizzbuzz() {
    println!("=== 과제 3: FizzBuzz ===");
    // TODO: fizzbuzz 함수를 호출하세요
}

fn main() {
    run_temperature_converter();
    run_fibonacci();
    run_fizzbuzz();
}
