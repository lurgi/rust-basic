// ========================================
// 과제 1: 소유권 이동 이해하기 (15분)
// ========================================
// 아래 코드에는 컴파일 에러가 있습니다.
// 에러를 수정하되, 두 가지 방법으로 해결하세요:
// 1) clone()을 사용하는 방법
// 2) 소유권을 다시 돌려받는 방법

// 의도적인 컴파일 에러 예제 (주석 처리됨)
// #[allow(dead_code)]
// fn problem_1() {
//     let message = String::from("Hello, Rust!");
//
//     print_message(message);
//
//     // 여기서 message를 다시 사용하고 싶습니다
//     println!("메시지 길이: {}", message.len());
// }
//
// fn print_message(msg: String) {
//     println!("메시지: {}", msg);
// }

// TODO: 위 코드를 두 가지 방법으로 수정하세요
// 방법 1: clone()을 사용 (problem_1_clone 함수 작성)
pub fn problem_1_clone() {
    // TODO: 여기에 코드를 작성하세요
}

// 방법 2: 소유권을 돌려받기 (problem_1_return 함수 작성)
pub fn problem_1_return() {
    // TODO: 여기에 코드를 작성하세요
}

pub fn run() {
    println!("=== 과제 1: 소유권 이동 이해하기 ===");
    // TODO: problem_1_clone()과 problem_1_return() 함수를 호출하세요
}
