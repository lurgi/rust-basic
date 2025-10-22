// ========================================
// 과제 2: 계산기 연산 (20분)
// ========================================
// 사칙연산을 표현하는 열거형을 만들고 계산 함수를 구현하세요.
//
// 요구사항:
// - Operation 열거형 정의
//   - Add(f64, f64): 덧셈
//   - Subtract(f64, f64): 뺄셈
//   - Multiply(f64, f64): 곱셈
//   - Divide(f64, f64): 나눗셈
//
// - calculate 함수 구현
//   - Operation을 받아서 계산 결과를 Option<f64>로 반환
//   - 나눗셈에서 0으로 나누면 None 반환
//   - 그 외에는 Some(결과) 반환
//   - match를 사용하여 구현

// TODO: Operation 열거형을 정의하세요
#[derive(Debug)]
enum Operation {
    // TODO: variant들을 정의하세요
}

// TODO: calculate 함수를 구현하세요
fn calculate(op: Operation) -> Option<f64> {
    // TODO: match를 사용하여 구현
    // 힌트: 나눗셈에서 divisor가 0.0인지 확인하세요
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_add() {
        // TODO: Add 연산을 테스트하세요
    }

    #[test]
    fn test_subtract() {
        // TODO: Subtract 연산을 테스트하세요
    }

    #[test]
    fn test_multiply() {
        // TODO: Multiply 연산을 테스트하세요
    }

    #[test]
    fn test_divide() {
        // TODO: Divide 연산을 테스트하세요
    }

    #[test]
    fn test_divide_by_zero() {
        // TODO: 0으로 나누기를 테스트하세요 (None이 반환되어야 함)
    }
}

pub fn run() {
    println!("\n=== 과제 2: 계산기 연산 ===");
    // TODO: Operation 테스트
    // 1. Add(10.0, 5.0) 계산 및 출력
    // 2. Subtract(10.0, 5.0) 계산 및 출력
    // 3. Multiply(10.0, 5.0) 계산 및 출력
    // 4. Divide(10.0, 5.0) 계산 및 출력
    // 5. Divide(10.0, 0.0) 계산 및 출력 (None이어야 함)
}

