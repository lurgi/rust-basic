// ========================================
// 과제 2: Future 조합
// ========================================
// 여러 비동기 작업을 조합하세요.
//
// 요구사항:
// - async fn process_step(step: u32, delay: u64) -> String
//   * delay 밀리초 대기
//   * "Step {step} 완료" 반환
//
// - async fn process_pipeline(steps: Vec<u32>) -> Vec<String>
//   * 각 step을 순차적으로 처리 (각각 100ms 대기)
//   * 모든 결과를 벡터로 반환
//   * 힌트: let mut results = Vec::new();
//           for step in steps {
//               results.push(process_step(step, 100).await);
//           }
//
// - async fn process_all_parallel(steps: Vec<u32>) -> Vec<String>
//   * 모든 step을 병렬로 처리
//   * 힌트: futures::future::join_all 사용
//           또는 tokio::spawn으로 각각 실행 후 결과 수집

use tokio::time::{sleep, Duration};

// TODO: process_step 함수를 구현하세요

// TODO: process_pipeline 함수를 구현하세요

// TODO: process_all_parallel 함수를 구현하세요
// 힌트: 간단한 방법은 futures를 벡터에 모아서 반복문으로 await
// 또는 tokio::spawn을 사용

pub async fn run() {
    println!("=== 과제 2: Future 조합 ===");

    // let steps = vec![1, 2, 3, 4, 5];

    // println!("순차 처리:");
    // let start = std::time::Instant::now();
    // let results = process_pipeline(steps.clone()).await;
    // println!("결과: {:?}", results);
    // println!("소요 시간: {:?}", start.elapsed());

    // println!("\n병렬 처리:");
    // let start = std::time::Instant::now();
    // let results = process_all_parallel(steps).await;
    // println!("결과: {:?}", results);
    // println!("소요 시간: {:?}", start.elapsed());
}
