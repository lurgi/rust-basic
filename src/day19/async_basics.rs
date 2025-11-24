// ========================================
// 과제 1: 기본 async/await
// ========================================
// async/await를 사용하여 비동기 함수를 작성하세요.
//
// 요구사항:
// - async fn fetch_number(n: u32) -> u32
//   * n 밀리초 대기 후 n을 반환
//   * 힌트: tokio::time::sleep(Duration::from_millis(n as u64)).await;
//
// - async fn sum_async(a: u32, b: u32) -> u32
//   * fetch_number(a)와 fetch_number(b)를 await
//   * 두 결과를 더해서 반환
//
// - async fn sum_parallel(a: u32, b: u32) -> u32
//   * fetch_number(a)와 fetch_number(b)를 병렬 실행
//   * 힌트: tokio::join! 사용

use tokio::time::{sleep, Duration};

// TODO: fetch_number 함수를 구현하세요

// TODO: sum_async 함수를 구현하세요

// TODO: sum_parallel 함수를 구현하세요

pub async fn run() {
    println!("=== 과제 1: 기본 async/await ===");

    // let result = fetch_number(100).await;
    // println!("fetch_number(100): {}", result);

    // println!("\n순차 실행:");
    // let start = std::time::Instant::now();
    // let result = sum_async(100, 200).await;
    // println!("결과: {}, 소요 시간: {:?}", result, start.elapsed());

    // println!("\n병렬 실행:");
    // let start = std::time::Instant::now();
    // let result = sum_parallel(100, 200).await;
    // println!("결과: {}, 소요 시간: {:?}", result, start.elapsed());
}

