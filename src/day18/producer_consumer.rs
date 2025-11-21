// ========================================
// 과제 4: 생산자-소비자 패턴
// ========================================
// 여러 생산자와 하나의 소비자 패턴을 구현하세요.
//
// 요구사항:
// - Message 열거형
//   - Data(String): 데이터 메시지
//   - Shutdown: 종료 신호
//
// - producer(id: u32, tx: mpsc::Sender<Message>, count: u32)
//   * count 개의 Data 메시지 전송
//   * 메시지 내용: "Producer {id}: Message {i}"
//   * 각 메시지 사이에 짧은 sleep
//
// - consumer(rx: mpsc::Receiver<Message>) -> Vec<String>
//   * 메시지를 수신하여 Data의 내용을 수집
//   * Shutdown 메시지를 받으면 종료
//   * 수집된 메시지들을 반환
//
// - run_producer_consumer(producer_count: u32, messages_per_producer: u32) -> Vec<String>
//   * producer_count 개의 생산자 스레드 생성
//   * 하나의 소비자 스레드 생성
//   * 모든 생산자가 끝나면 Shutdown 메시지 전송
//   * 소비자의 결과 반환

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

// TODO: Message 열거형을 정의하세요

// TODO: producer 함수를 구현하세요

// TODO: consumer 함수를 구현하세요

// TODO: run_producer_consumer 함수를 구현하세요

pub fn run() {
    println!("=== 과제 4: 생산자-소비자 패턴 ===");

    // // 3개의 생산자가 각각 2개의 메시지 전송
    // let results = run_producer_consumer(3, 2);

    // println!("수신된 메시지 (총 {}):", results.len());
    // for msg in results {
    //     println!("  {}", msg);
    // }
}
