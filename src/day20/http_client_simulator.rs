// ========================================
// 과제 3: HTTP 클라이언트 시뮬레이터
// ========================================
// 실제 HTTP 요청 대신 시뮬레이션으로 HTTP 클라이언트를 만드세요.
// (실제 reqwest를 사용하지 않고 비동기 동작만 시뮬레이션)
//
// 요구사항:
// - ApiResponse 구조체
//   - status: u16
//   - body: String
//
// - HttpClient 구조체
//   - 빈 구조체
//
// - impl HttpClient
//   - new() -> HttpClient
//
//   - async fn get(&self, url: &str) -> Result<ApiResponse, String>
//     * url이 "error"를 포함하면 Err 반환
//     * 100ms 대기 (네트워크 시뮬레이션)
//     * Ok(ApiResponse { status: 200, body: format!("Response from {}", url) })
//
//   - async fn post(&self, url: &str, body: &str) -> Result<ApiResponse, String>
//     * url이 "error"를 포함하면 Err 반환
//     * body가 비어있으면 Err("빈 body") 반환
//     * 150ms 대기
//     * Ok(ApiResponse { status: 201, body: format!("Created: {}", body) })
//
// - async fn fetch_multiple(urls: Vec<String>) -> Vec<Result<ApiResponse, String>>
//   * 모든 URL을 병렬로 요청 (tokio::spawn 사용)
//   * 모든 결과 수집

use tokio::time::{Duration, sleep};

// TODO: ApiResponse 구조체를 정의하세요

// TODO: HttpClient 구조체를 정의하세요

// TODO: HttpClient의 모든 메서드를 구현하세요

// TODO: fetch_multiple 함수를 구현하세요

pub async fn run() {
    println!("=== 과제 3: HTTP 클라이언트 시뮬레이터 ===");

    // let client = HttpClient::new();

    // // GET 요청
    // match client.get("https://api.example.com/users").await {
    //     Ok(response) => println!("GET 성공 ({}): {}", response.status, response.body),
    //     Err(e) => println!("GET 실패: {}", e),
    // }

    // // GET 에러
    // match client.get("https://api.example.com/error").await {
    //     Ok(response) => println!("GET 성공 ({}): {}", response.status, response.body),
    //     Err(e) => println!("GET 실패: {}", e),
    // }

    // // POST 요청
    // match client.post("https://api.example.com/users", r#"{"name": "Alice"}"#).await {
    //     Ok(response) => println!("POST 성공 ({}): {}", response.status, response.body),
    //     Err(e) => println!("POST 실패: {}", e),
    // }

    // // 병렬 요청
    // let urls = vec![
    //     String::from("https://api.example.com/users/1"),
    //     String::from("https://api.example.com/users/2"),
    //     String::from("https://api.example.com/error"),
    //     String::from("https://api.example.com/users/3"),
    // ];

    // println!("\n병렬 요청 시작:");
    // let start = std::time::Instant::now();
    // let results = fetch_multiple(urls).await;
    // println!("소요 시간: {:?}", start.elapsed());

    // for (i, result) in results.iter().enumerate() {
    //     match result {
    //         Ok(response) => println!("  [{}] 성공 ({}): {}", i, response.status, response.body),
    //         Err(e) => println!("  [{}] 실패: {}", i, e),
    //     }
    // }
}
