// ========================================
// 과제 1: 기본 스레드 생성
// ========================================
// 여러 스레드를 생성하고 관리하세요.
//
// 요구사항:
// - create_threads(count: u32) -> Vec<thread::JoinHandle<u32>>
//   * count 개의 스레드 생성
//   * 각 스레드는 자신의 번호(0부터 시작)를 반환
//   * 힌트: thread::spawn(move || i) 사용
//
// - wait_all(handles: Vec<thread::JoinHandle<u32>>) -> Vec<u32>
//   * 모든 스레드의 완료를 대기하고 결과 수집
//   * 힌트: handles.into_iter().map(|h| h.join().unwrap()).collect()

use std::thread;
use std::time::Duration;

// TODO: create_threads 함수를 구현하세요
fn create_threads(count: u32) -> Vec<thread::JoinHandle<u32>> {
    let mut threads = Vec::new();

    for i in 0..count {
        threads.push(thread::spawn(move || i));
    }

    threads
}

// TODO: wait_all 함수를 구현하세요
fn wait_all(handles: Vec<thread::JoinHandle<u32>>) -> Vec<u32> {
    handles.into_iter().map(|h| h.join().unwrap()).collect()
}

pub fn run() {
    println!("=== 과제 1: 기본 스레드 생성 ===");

    let handles = create_threads(5);
    println!("5개의 스레드 생성됨");

    let results = wait_all(handles);
    println!("스레드 결과: {:?}", results);
}
