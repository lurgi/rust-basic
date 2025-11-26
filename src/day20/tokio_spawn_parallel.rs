// ========================================
// 과제 1: tokio::spawn으로 병렬 처리
// ========================================
// tokio::spawn을 사용하여 여러 작업을 병렬로 처리하세요.
//
// 요구사항:
// - async fn compute(id: u32, duration_ms: u64) -> u32
//   * duration_ms 밀리초 대기
//   * id * 2를 반환
//
// - async fn parallel_compute(tasks: Vec<(u32, u64)>) -> Vec<u32>
//   * tasks는 (id, duration_ms) 튜플의 벡터
//   * 각 작업을 tokio::spawn으로 병렬 실행
//   * 모든 결과를 순서대로 수집하여 반환
//   * 힌트: handles.push(tokio::spawn(async move { compute(id, duration).await }));

use tokio::time::{Duration, sleep};

// TODO: compute 함수를 구현하세요

// TODO: parallel_compute 함수를 구현하세요

pub async fn run() {
    println!("=== 과제 1: tokio::spawn으로 병렬 처리 ===");

    // let tasks = vec![
    //     (1, 100),
    //     (2, 200),
    //     (3, 150),
    //     (4, 50),
    //     (5, 100),
    // ];

    // let start = std::time::Instant::now();
    // let results = parallel_compute(tasks).await;
    // println!("결과: {:?}", results);
    // println!("소요 시간: {:?}", start.elapsed());
    // 병렬 실행이므로 약 200ms (가장 긴 작업) 소요
}
