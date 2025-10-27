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
    }

    // TODO: advanced 모듈을 정의하세요
    pub mod advanced {
        // TODO: power와 square_root 함수를 구현하세요
        // 힌트: f64의 powf()와 sqrt() 메서드 사용
    }
}

pub fn run() {
    println!("=== 과제 2: 계산기 모듈 ===");
    // TODO: calculator::basic과 calculator::advanced 사용
    // 1. use로 경로 단축
    // 2. 각 함수 호출 및 결과 출력
}
