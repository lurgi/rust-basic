// ========================================
// 과제 4: 연관 타입과 트레잇 객체 결합 (30분)
// ========================================
// 연관 타입과 트레잇 객체를 함께 사용하여 데이터 처리 파이프라인을 만드세요.
//
// 요구사항:
// - Processor 트레잇
//   - type Input;
//   - type Output;
//   - fn process(&self, input: Self::Input) -> Self::Output;
//
// - UppercaseProcessor 구조체
//   - Processor 트레잇 구현
//     * Input = String
//     * Output = String
//     * 문자열을 대문자로 변환
//
// - LengthProcessor 구조체
//   - Processor 트레잇 구현
//     * Input = String
//     * Output = usize
//     * 문자열의 길이 반환
//
// - DoubleProcessor 구조체
//   - Processor 트레잇 구현
//     * Input = i32
//     * Output = i32
//     * 숫자를 2배로 만듦
//
// 주의: 연관 타입을 사용하는 트레잇은 트레잇 객체로 만들기 어렵습니다.
// 대신 각 프로세서를 개별적으로 사용하는 방식으로 구현하세요.

// TODO: Processor 트레잇을 정의하세요 (연관 타입 포함)
trait Processor {
    type Input;
    type Output;

    fn process(&self, input: Self::Input) -> Self::Output;
}

// TODO: UppercaseProcessor 구조체를 정의하세요
struct UppercaseProcessor;

// TODO: UppercaseProcessor에 Processor 트레잇을 구현하세요
impl Processor for UppercaseProcessor {
    type Input = String;
    type Output = String;

    fn process(&self, input: Self::Input) -> Self::Output {
        input.to_uppercase()
    }
}

// TODO: LengthProcessor 구조체를 정의하세요
struct LengthProcessor;

// TODO: LengthProcessor에 Processor 트레잇을 구현하세요
impl Processor for LengthProcessor {
    type Input = String;
    type Output = usize;

    fn process(&self, input: Self::Input) -> Self::Output {
        input.len()
    }
}

// TODO: DoubleProcessor 구조체를 정의하세요
struct DoubleProcessor;

// TODO: DoubleProcessor에 Processor 트레잇을 구현하세요
impl Processor for DoubleProcessor {
    type Input = i32;
    type Output = i32;

    fn process(&self, input: Self::Input) -> Self::Output {
        input * 2
    }
}

pub fn run() {
    println!("=== 과제 4: 연관 타입과 트레잇 객체 결합 ===");

    let uppercase = UppercaseProcessor;
    let length = LengthProcessor;
    let double = DoubleProcessor;

    // UppercaseProcessor 테스트
    let result = uppercase.process(String::from("hello"));
    println!("대문자 변환: {}", result);

    // LengthProcessor 테스트
    let result = length.process(String::from("hello world"));
    println!("문자열 길이: {}", result);

    // DoubleProcessor 테스트
    let result = double.process(21);
    println!("2배 만들기: {}", result);
}
