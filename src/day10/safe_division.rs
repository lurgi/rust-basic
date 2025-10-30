// ========================================
// 과제 1: 안전한 나눗셈 계산기 (15분)
// ========================================
// Result와 Option을 사용하여 안전한 나눗셈 함수를 구현하세요.
//
// 요구사항:
// - safe_divide_result(a: i32, b: i32) -> Result<i32, String>
//   * b가 0이면 Err("0으로 나눌 수 없습니다")
//   * 그 외에는 Ok(a / b)
//
// - safe_divide_option(a: i32, b: i32) -> Option<i32>
//   * b가 0이면 None
//   * 그 외에는 Some(a / b)

// TODO: safe_divide_result 함수를 구현하세요
fn safe_divide_result(_a: i32, _b: i32) -> Result<i32, String> {
    if _b == 0 {
        return Err("0으로 나눌 수 없습니다".to_string());
    }
    Ok(_a / _b)
}

// TODO: safe_divide_option 함수를 구현하세요
fn safe_divide_option(_a: i32, _b: i32) -> Option<i32> {
    if _b == 0 {
        return None;
    }
    Some(_a / _b)
}

pub fn run() {
    println!("=== 과제 1: 안전한 나눗셈 계산기 ===");

    // TODO: safe_divide_result 테스트
    match safe_divide_result(10, 2) {
        Ok(result) => println!("10 / 2 = {}", result),
        Err(e) => println!("에러: {}", e),
    }

    match safe_divide_result(10, 0) {
        Ok(result) => println!("10 / 0 = {}", result),
        Err(e) => println!("에러: {}", e),
    }

    // TODO: safe_divide_option 테스트
    if let Some(result) = safe_divide_option(20, 4) {
        println!("20 / 4 = {}", result);
    } else {
        println!("나눗셈 실패");
    }

    if let Some(result) = safe_divide_option(20, 0) {
        println!("20 / 0 = {}", result);
    } else {
        println!("나눗셈 실패");
    }
}
