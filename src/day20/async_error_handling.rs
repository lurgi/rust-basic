// ========================================
// 과제 2: 비동기 에러 처리
// ========================================
// Result를 사용하여 비동기 코드에서 에러를 처리하세요.
//
// 요구사항:
// - ProcessError 열거형
//   - InvalidInput(String)
//   - Timeout
//   - Failed(String)
//
// - async fn process_with_validation(value: i32) -> Result<i32, ProcessError>
//   * value가 0이면 InvalidInput 에러
//   * value가 음수면 Failed 에러
//   * 100ms 대기 후 value * 2 반환
//
// - async fn process_with_timeout(value: i32, timeout_ms: u64) -> Result<i32, ProcessError>
//   * process_with_validation을 timeout_ms 내에 실행
//   * 타임아웃 시 ProcessError::Timeout 반환
//   * 힌트: tokio::time::timeout 사용
//
// - async fn process_batch(values: Vec<i32>) -> Vec<Result<i32, ProcessError>>
//   * 각 값을 tokio::spawn으로 처리
//   * 모든 결과 (성공/실패 모두) 수집

use tokio::time::{Duration, sleep, timeout};

// TODO: ProcessError 열거형을 정의하세요
#[derive(Debug)]
enum ProcessError {
    InvalidInput(String),
    Timeout,
    Failed(String),
}

// TODO: process_with_validation 함수를 구현하세요
async fn process_with_validation(value: i32) -> Result<i32, ProcessError> {
    sleep(Duration::from_millis(100)).await;

    match value {
        0 => Err(ProcessError::InvalidInput(
            "0은 유효하지 않은 입력입니다".to_string(),
        )),
        value if value < 0 => Err(ProcessError::Failed(
            "음수는 유효하지 않은 입력입니다".to_string(),
        )),
        _ => Ok(value * 2),
    }
}

// TODO: process_with_timeout 함수를 구현하세요
async fn process_with_timeout(value: i32, timeout_ms: u64) -> Result<i32, ProcessError> {
    let result = timeout(
        Duration::from_millis(timeout_ms),
        process_with_validation(value),
    )
    .await;

    match result {
        Ok(val) => val,
        Err(_) => Err(ProcessError::Timeout),
    }
}

// TODO: process_batch 함수를 구현하세요
async fn process_batch(values: Vec<i32>) -> Vec<Result<i32, ProcessError>> {
    let mut handles = Vec::new();

    for value in values {
        let handle = tokio::spawn(async move { process_with_timeout(value, 1000).await });
        handles.push(handle);
    }

    let mut results = Vec::new();

    for handle in handles {
        results.push(handle.await.unwrap());
    }

    results
}

pub async fn run() {
    println!("=== 과제 2: 비동기 에러 처리 ===");

    // 개별 테스트
    match process_with_validation(10).await {
        Ok(result) => println!("성공: {}", result),
        Err(e) => println!("에러: {:?}", e),
    }

    match process_with_validation(0).await {
        Ok(result) => println!("성공: {}", result),
        Err(e) => println!("에러: {:?}", e),
    }

    // 타임아웃 테스트
    match process_with_timeout(10, 200).await {
        Ok(result) => println!("타임아웃 테스트 성공: {}", result),
        Err(e) => println!("타임아웃 테스트 에러: {:?}", e),
    }

    match process_with_timeout(10, 50).await {
        Ok(result) => println!("타임아웃 테스트 성공: {}", result),
        Err(e) => println!("타임아웃 테스트 에러: {:?}", e),
    }

    // 배치 처리
    let values = vec![5, 0, 10, -1, 15];
    let results = process_batch(values).await;

    println!("\n배치 처리 결과:");
    for (i, result) in results.iter().enumerate() {
        match result {
            Ok(value) => println!("  [{}] 성공: {}", i, value),
            Err(e) => println!("  [{}] 실패: {:?}", i, e),
        }
    }
}
