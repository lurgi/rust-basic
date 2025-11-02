// ========================================
// 과제 3: 데이터 파이프라인 (25분)
// ========================================
// 여러 단계의 데이터 처리를 에러 전파로 연결하세요.
//
// 요구사항:
// - DataError 열거형
//   - ReadError(String): 읽기 실패
//   - ParseError(String): 파싱 실패
//   - ValidationError(String): 검증 실패
//   - ProcessError(String): 처리 실패
//
// - read_raw_data(id: u32) -> Result<String, DataError>
//   * id가 1이면 Ok("100,200,300")
//   * id가 2이면 Ok("50,abc,150") (잘못된 데이터)
//   * id가 3이면 Ok("10,20,30")
//   * 그 외는 Err(DataError::ReadError("데이터를 찾을 수 없습니다"))
//
// - parse_numbers(raw: &str) -> Result<Vec<i32>, DataError>
//   * 쉼표로 분리된 숫자들을 Vec<i32>로 변환
//   * 파싱 실패 시 Err(DataError::ParseError(...))
//   * 힌트: split(','), parse(), map, collect 사용
//
// - validate_numbers(numbers: &[i32]) -> Result<(), DataError>
//   * 모든 숫자가 0보다 크면 Ok(())
//   * 하나라도 0 이하면 Err(DataError::ValidationError("양수만 허용됩니다"))
//
// - calculate_average(numbers: &[i32]) -> Result<f64, DataError>
//   * 숫자들이 비어있으면 Err(DataError::ProcessError("빈 배열입니다"))
//   * 평균 계산 후 Ok(평균)
//
// - process_data_pipeline(id: u32) -> Result<f64, DataError>
//   * 위 함수들을 ? 연산자로 연결하여 전체 파이프라인 구성
//   * 순서: read_raw_data → parse_numbers → validate_numbers → calculate_average

// TODO: DataError 열거형을 정의하세요
#[derive(Debug)]
enum DataError {
    // TODO: variant들을 추가하세요
}

// TODO: read_raw_data 함수를 구현하세요
fn read_raw_data(_id: u32) -> Result<String, DataError> {
    unimplemented!("read_raw_data 함수를 구현하세요")
}

// TODO: parse_numbers 함수를 구현하세요
fn parse_numbers(_raw: &str) -> Result<Vec<i32>, DataError> {
    unimplemented!("parse_numbers 함수를 구현하세요")
}

// TODO: validate_numbers 함수를 구현하세요
fn validate_numbers(_numbers: &[i32]) -> Result<(), DataError> {
    unimplemented!("validate_numbers 함수를 구현하세요")
}

// TODO: calculate_average 함수를 구현하세요
fn calculate_average(_numbers: &[i32]) -> Result<f64, DataError> {
    unimplemented!("calculate_average 함수를 구현하세요")
}

// TODO: process_data_pipeline 함수를 구현하세요
// 힌트: let raw = read_raw_data(id)?;
//      let numbers = parse_numbers(&raw)?;
//      validate_numbers(&numbers)?;
//      calculate_average(&numbers)
fn process_data_pipeline(_id: u32) -> Result<f64, DataError> {
    unimplemented!("process_data_pipeline 함수를 구현하세요")
}

pub fn run() {
    println!("=== 과제 3: 데이터 파이프라인 ===");

    // 정상 케이스 (id = 1)
    match process_data_pipeline(1) {
        Ok(avg) => println!("ID 1의 평균: {}", avg),
        Err(e) => println!("처리 실패: {:?}", e),
    }

    // 파싱 에러 (id = 2)
    match process_data_pipeline(2) {
        Ok(avg) => println!("ID 2의 평균: {}", avg),
        Err(e) => println!("처리 실패: {:?}", e),
    }

    // 정상 케이스 (id = 3)
    match process_data_pipeline(3) {
        Ok(avg) => println!("ID 3의 평균: {}", avg),
        Err(e) => println!("처리 실패: {:?}", e),
    }

    // 읽기 에러 (id = 999)
    match process_data_pipeline(999) {
        Ok(avg) => println!("ID 999의 평균: {}", avg),
        Err(e) => println!("처리 실패: {:?}", e),
    }
}
