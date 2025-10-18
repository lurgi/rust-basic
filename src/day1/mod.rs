// 1일차 과제들
mod fibonacci;
mod fizzbuzz;
mod temperature;

// 모듈에서 함수들을 다시 내보내기
use fibonacci::fibonacci;
use fizzbuzz::fizzbuzz;
use temperature::{celsius_to_fahrenheit, fahrenheit_to_celsius};

// 과제 1: 온도 변환기 실행
fn run_temperature_converter() {
    println!("=== 과제 1: 온도 변환기 ===");
    let celsius = 25.0;
    let fahrenheit = 77.0;

    // TODO: celsius_to_fahrenheit와 fahrenheit_to_celsius 함수를 호출하고 결과를 출력하세요
    println!(
        "{}°C = {}°F\n{}°F = {}°C",
        celsius,
        celsius_to_fahrenheit(celsius),
        fahrenheit,
        fahrenheit_to_celsius(fahrenheit)
    );
}

// 과제 2: 피보나치 수열 실행
fn run_fibonacci() {
    println!("=== 과제 2: 피보나치 수열 ===");
    // TODO: 10번째 피보나치 수를 출력하세요
    println!("10번째 피보나치 수 = {}", fibonacci(10));
}

// 과제 3: FizzBuzz 실행
fn run_fizzbuzz() {
    println!("=== 과제 3: FizzBuzz ===");
    // TODO: fizzbuzz 함수를 호출하세요
    fizzbuzz(100);
}

#[allow(dead_code)]
pub fn run_day1_exercises() {
    run_temperature_converter();
    println!();
    run_fibonacci();
    println!();
    run_fizzbuzz();
}
