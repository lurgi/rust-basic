// ========================================
// 과제 4: 재시도 로직 구현
// ========================================
// 실패한 작업을 자동으로 재시도하는 시스템을 만드세요.
//
// 요구사항:
// - async fn unreliable_operation(id: u32, fail_count: &mut u32) -> Result<String, String>
//   * fail_count가 2 미만이면 실패 (Err 반환)
//   * fail_count를 1 증가
//   * fail_count가 2 이상이면 성공 (Ok 반환)
//   * 50ms 대기
//
// - async fn retry_operation<F, Fut, T, E>(
//       mut operation: F,
//       max_retries: u32,
//       delay_ms: u64,
//   ) -> Result<T, E>
//   where
//       F: FnMut() -> Fut,
//       Fut: std::future::Future<Output = Result<T, E>>,
//   * operation을 최대 max_retries번 시도
//   * 실패 시 delay_ms 대기 후 재시도
//   * 모두 실패하면 마지막 에러 반환
//
// - async fn process_with_retry(id: u32) -> Result<String, String>
//   * unreliable_operation을 최대 3번 재시도
//   * 재시도 간격 100ms

use tokio::time::{Duration, sleep};

// TODO: unreliable_operation 함수를 구현하세요

// TODO: retry_operation 함수를 구현하세요
// 힌트: loop { match operation().await { ... } }

// TODO: process_with_retry 함수를 구현하세요

pub async fn run() {
    println!("=== 과제 4: 재시도 로직 구현 ===");

    // match process_with_retry(1).await {
    //     Ok(result) => println!("최종 성공: {}", result),
    //     Err(e) => println!("최종 실패: {}", e),
    // }

    // // 여러 작업을 병렬로 재시도
    // let mut handles = vec![];
    // for i in 1..=3 {
    //     handles.push(tokio::spawn(async move {
    //         process_with_retry(i).await
    //     }));
    // }

    // for (i, handle) in handles.into_iter().enumerate() {
    //     match handle.await.unwrap() {
    //         Ok(result) => println!("작업 {} 성공: {}", i + 1, result),
    //         Err(e) => println!("작업 {} 실패: {}", i + 1, e),
    //     }
    // }
}
