// ========================================
// 과제 3: 타임아웃과 select
// ========================================
// 타임아웃과 select를 사용하여 비동기 작업을 제어하세요.
//
// 요구사항:
// - async fn slow_operation(duration: u64) -> String
//   * duration 밀리초 대기 후 "완료" 반환
//
// - async fn with_timeout(duration: u64, timeout_ms: u64) -> Result<String, String>
//   * slow_operation(duration)을 timeout_ms 밀리초 내에 완료
//   * 성공하면 Ok(결과), 타임아웃이면 Err("타임아웃")
//   * 힌트: tokio::time::timeout 사용
//
// - async fn race_tasks(task1_delay: u64, task2_delay: u64) -> String
//   * 두 작업 중 먼저 완료되는 것의 결과 반환
//   * 힌트: tokio::select! 사용
//           tokio::select! {
//               result = slow_operation(task1_delay) => format!("Task 1: {}", result),
//               result = slow_operation(task2_delay) => format!("Task 2: {}", result),
//           }

use tokio::time::{sleep, Duration, timeout};

// TODO: slow_operation 함수를 구현하세요

// TODO: with_timeout 함수를 구현하세요

// TODO: race_tasks 함수를 구현하세요

pub async fn run() {
    println!("=== 과제 3: 타임아웃과 select ===");

    // // 성공 케이스
    // match with_timeout(500, 1000).await {
    //     Ok(result) => println!("성공: {}", result),
    //     Err(e) => println!("에러: {}", e),
    // }

    // // 타임아웃 케이스
    // match with_timeout(1500, 1000).await {
    //     Ok(result) => println!("성공: {}", result),
    //     Err(e) => println!("에러: {}", e),
    // }

    // // 레이스
    // let winner = race_tasks(500, 1000).await;
    // println!("승자: {}", winner);

    // let winner = race_tasks(1000, 500).await;
    // println!("승자: {}", winner);
}
