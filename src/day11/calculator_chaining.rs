// ========================================
// 과제 1: 계산기 체이닝 (20분)
// ========================================
// ? 연산자를 사용하여 여러 단계의 계산을 체이닝하세요.
//
// 요구사항:
// - safe_add(a: i32, b: i32) -> Result<i32, String>
//   * i32 오버플로우 체크
//   * 오버플로우 발생 시 Err("덧셈 오버플로우")
//   * 힌트: checked_add() 메서드 사용
//
// - safe_multiply(a: i32, b: i32) -> Result<i32, String>
//   * i32 오버플로우 체크
//   * 오버플로우 발생 시 Err("곱셈 오버플로우")
//   * 힌트: checked_mul() 메서드 사용
//
// - safe_divide(a: i32, b: i32) -> Result<i32, String>
//   * 0으로 나누기 체크
//   * 0으로 나누면 Err("0으로 나눌 수 없습니다")
//
// - calculate_formula(a: i32, b: i32, c: i32) -> Result<i32, String>
//   * 다음 공식을 계산: ((a + b) * c) / 2
//   * ? 연산자를 사용하여 각 단계를 체이닝
//   * 각 단계에서 에러가 발생하면 즉시 반환

// TODO: safe_add 함수를 구현하세요
fn safe_add(_a: i32, _b: i32) -> Result<i32, String> {
    unimplemented!("safe_add 함수를 구현하세요")
}

// TODO: safe_multiply 함수를 구현하세요
fn safe_multiply(_a: i32, _b: i32) -> Result<i32, String> {
    unimplemented!("safe_multiply 함수를 구현하세요")
}

// TODO: safe_divide 함수를 구현하세요
fn safe_divide(_a: i32, _b: i32) -> Result<i32, String> {
    unimplemented!("safe_divide 함수를 구현하세요")
}

// TODO: calculate_formula 함수를 구현하세요
// 힌트: let step1 = safe_add(a, b)?;
//      let step2 = safe_multiply(step1, c)?;
//      ...
fn calculate_formula(_a: i32, _b: i32, _c: i32) -> Result<i32, String> {
    unimplemented!("calculate_formula 함수를 구현하세요")
}

pub fn run() {
    println!("=== 과제 1: 계산기 체이닝 ===");

    // 정상 케이스
    match calculate_formula(10, 20, 3) {
        Ok(result) => println!("((10 + 20) * 3) / 2 = {}", result),
        Err(e) => println!("에러: {}", e),
    }

    // 0으로 나누기
    match calculate_formula(10, 20, 0) {
        Ok(result) => println!("결과: {}", result),
        Err(e) => println!("에러: {}", e),
    }

    // 오버플로우 (큰 숫자)
    match calculate_formula(i32::MAX, 1, 2) {
        Ok(result) => println!("결과: {}", result),
        Err(e) => println!("에러: {}", e),
    }
}
