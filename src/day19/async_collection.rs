// ========================================
// 과제 4: 비동기 데이터 수집
// ========================================
// 비동기로 데이터를 수집하는 시스템을 만드세요.
//
// 요구사항:
// - Data 구조체
//   - id: u32
//   - value: String
//
// - async fn fetch_data(id: u32) -> Data
//   * id * 50 밀리초 대기
//   * Data 생성하여 반환 (value는 "Data {id}")
//
// - async fn fetch_all(ids: Vec<u32>) -> Vec<Data>
//   * 모든 id에 대해 fetch_data를 병렬로 실행
//   * 모든 결과를 수집하여 반환
//   * 힌트: let futures: Vec<_> = ids.into_iter()
//               .map(|id| fetch_data(id))
//               .collect();
//           futures::future::join_all(futures).await
//   * 또는 간단히 반복문으로 futures를 모아서 await

use tokio::time::{Duration, sleep};

// TODO: Data 구조체를 정의하세요
struct Data {
    id: u32,
    value: String,
}

// TODO: fetch_data 함수를 구현하세요
async fn fetch_data(id: u32) -> Data {
    sleep(Duration::from_millis(id as u64 * 50)).await;
    Data {
        id,
        value: format!("Data {}", id),
    }
}

// TODO: fetch_all 함수를 구현하세요
async fn fetch_all(ids: Vec<u32>) -> Vec<Data> {
    let futures: Vec<_> = ids.into_iter().map(|id| fetch_data(id)).collect();

    futures::future::join_all(futures).await
}

pub async fn run() {
    println!("=== 과제 4: 비동기 데이터 수집 ===");

    let ids = vec![1, 2, 3, 4, 5];

    let start = std::time::Instant::now();
    let data = fetch_all(ids).await;
    println!("수집 완료 (소요 시간: {:?})", start.elapsed());

    for item in data {
        println!("  ID {}: {}", item.id, item.value);
    }
}
