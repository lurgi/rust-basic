// 2일차 과제 (60분)
// 오늘 배운 소유권(Ownership) 개념을 활용한 과제입니다.

// ========================================
// 과제 1: 소유권 이동 이해하기 (15분)
// ========================================
// 아래 코드에는 컴파일 에러가 있습니다.
// 에러를 수정하되, 두 가지 방법으로 해결하세요:
// 1) clone()을 사용하는 방법
// 2) 소유권을 다시 돌려받는 방법

#[allow(dead_code)]
fn problem_1() {
    let message = String::from("Hello, Rust!");

    print_message(message);

    // 여기서 message를 다시 사용하고 싶습니다
    println!("메시지 길이: {}", message.len());
}

fn print_message(msg: String) {
    println!("메시지: {}", msg);
}

// TODO: 위 코드를 두 가지 방법으로 수정하세요
// 방법 1: clone()을 사용 (problem_1_clone 함수 작성)
#[allow(dead_code)]
pub fn problem_1_clone() {
    // TODO: 여기에 코드를 작성하세요
}

// 방법 2: 소유권을 돌려받기 (problem_1_return 함수 작성)
#[allow(dead_code)]
pub fn problem_1_return() {
    // TODO: 여기에 코드를 작성하세요
}

// ========================================
// 과제 2: 문자열 연결 함수 (20분)
// ========================================
// 두 개의 String을 받아서 연결한 새로운 String을 반환하는 함수를 작성하세요.
//
// 요구사항:
// - 함수는 두 개의 String을 매개변수로 받습니다
// - 두 문자열을 연결한 새로운 String을 반환합니다
// - 원본 문자열들의 소유권은 함수로 이동하며, 다시 돌려주지 않습니다
// - main에서 결과를 출력합니다
//
// 힌트:
// - format! 매크로를 사용할 수 있습니다: format!("{}{}", s1, s2)
// - 또는 String의 push_str 메서드를 사용할 수 있습니다

pub fn concatenate(s1: String, s2: String) -> String {
    // TODO: 여기에 코드를 작성하세요
    format!("{}{}", s1, s2)
}

// ========================================
// 과제 3: 벡터 필터링 (25분)
// ========================================
// 정수 벡터를 받아서 짝수만 포함하는 새로운 벡터를 반환하는 함수를 작성하세요.
//
// 요구사항:
// - 함수는 Vec<i32>를 매개변수로 받습니다
// - 짝수만 포함하는 새로운 Vec<i32>를 반환합니다
// - 원본 벡터의 소유권은 함수로 이동합니다
// - main에서 결과를 출력합니다
//
// 힌트:
// - 새로운 빈 벡터를 만들려면: let mut result = Vec::new();
// - 벡터에 요소를 추가하려면: result.push(value);
// - for 루프로 벡터를 순회할 수 있습니다

pub fn filter_even_numbers(numbers: Vec<i32>) -> Vec<i32> {
    // TODO: 여기에 코드를 작성하세요
    let mut result = Vec::new();
    result
}

pub fn run_ownership_exercises() {
    println!("=== 과제 1: 소유권 이동 이해하기 ===");
    // TODO: problem_1_clone()과 problem_1_return() 함수를 호출하세요

    println!("\n=== 과제 2: 문자열 연결 함수 ===");
    let first = String::from("Hello, ");
    let second = String::from("Rust!");
    // TODO: concatenate 함수를 호출하고 결과를 출력하세요

    println!("\n=== 과제 3: 벡터 필터링 ===");
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    // TODO: filter_even_numbers 함수를 호출하고 결과를 출력하세요
    // 힌트: 벡터 출력은 {:?} 포맷을 사용합니다
}
