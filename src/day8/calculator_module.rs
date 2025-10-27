// ========================================
// 과제 2: 계산기 모듈 (20분)
// ========================================
// 계산기 기능을 모듈로 분리하세요.
//
// 요구사항:
// - calculator 모듈 생성
//   - basic 하위 모듈
//     - pub fn add(a: f64, b: f64) -> f64
//     - pub fn subtract(a: f64, b: f64) -> f64
//     - pub fn multiply(a: f64, b: f64) -> f64
//     - pub fn divide(a: f64, b: f64) -> Option<f64>
//
//   - advanced 하위 모듈
//     - pub fn power(base: f64, exponent: f64) -> f64
//     - pub fn square_root(x: f64) -> Option<f64>
//
// - main 함수에서 use를 사용하여 각 함수를 호출
// - 결과를 출력

// TODO: calculator 모듈을 정의하세요
mod calculator {
    // TODO: basic 모듈을 정의하세요
    pub mod basic {
        // TODO: 사칙연산 함수들을 구현하세요
        pub fn add(a: f64, b: f64) -> f64 {
            a + b
        }
        pub fn subtract(a: f64, b: f64) -> f64 {
            a - b
        }
        pub fn multiply(a: f64, b: f64) -> f64 {
            a * b
        }
        pub fn divide(a: f64, b: f64) -> Option<f64> {
            if b == 0.0 { None } else { Some(a / b) }
        }
    }

    // TODO: advanced 모듈을 정의하세요
    pub mod advanced {
        // TODO: power와 square_root 함수를 구현하세요
        // 힌트: f64의 powf()와 sqrt() 메서드 사용
        pub fn power(base: f64, exponent: f64) -> f64 {
            base.powf(exponent)
        }

        pub fn square_root(x: f64) -> Option<f64> {
            if x < 0.0 { None } else { Some(x.sqrt()) }
        }
    }
}

pub fn run() {
    println!("=== 과제 2: 계산기 모듈 ===");
    // TODO: calculator::basic과 calculator::advanced 사용
    // 1. use로 경로 단축
    use calculator::{advanced, basic};
    // 2. 각 함수 호출 및 결과 출력
    // 기본 연산
    println!("10 + 5 = {}", basic::add(10.0, 5.0));
    println!("10 - 5 = {}", basic::subtract(10.0, 5.0));
    println!("10 * 5 = {}", basic::multiply(10.0, 5.0));

    match basic::divide(10.0, 5.0) {
        Some(result) => println!("10 / 5 = {}", result),
        None => println!("0으로 나눌 수 없습니다"),
    }

    match basic::divide(10.0, 0.0) {
        Some(result) => println!("10 / 0 = {}", result),
        None => println!("10 / 0 = 오류: 0으로 나눌 수 없습니다"),
    }

    // 고급 연산
    println!("2의 3승 = {}", advanced::power(2.0, 3.0));

    match advanced::square_root(16.0) {
        Some(result) => println!("16의 제곱근 = {}", result),
        None => println!("음수의 제곱근은 계산할 수 없습니다"),
    }
}
