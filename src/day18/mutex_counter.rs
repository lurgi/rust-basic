// ========================================
// 과제 3: Mutex로 공유 카운터
// ========================================
// Arc와 Mutex를 사용하여 스레드 안전한 카운터를 구현하세요.
//
// 요구사항:
// - Counter 구조체
//   - count: Mutex<i32>
//
// - impl Counter
//   - new() -> Arc<Counter>
//     * count를 0으로 초기화한 Counter를 Arc로 감싸서 반환
//
//   - increment(&self)
//     * count를 1 증가
//
//   - decrement(&self)
//     * count를 1 감소
//
//   - get(&self) -> i32
//     * 현재 count 값 반환
//
// - parallel_increments(counter: Arc<Counter>, threads: u32, increments_per_thread: u32)
//   * 지정된 수의 스레드 생성
//   * 각 스레드에서 increments_per_thread 번 increment 호출
//   * 모든 스레드 완료 대기

use std::sync::{Arc, Mutex};
use std::thread;

// TODO: Counter 구조체를 정의하세요

// TODO: Counter의 모든 메서드를 구현하세요

// TODO: parallel_increments 함수를 구현하세요

pub fn run() {
    println!("=== 과제 3: Mutex로 공유 카운터 ===");

    // let counter = Counter::new();

    // // 10개 스레드가 각각 100번씩 증가
    // parallel_increments(Arc::clone(&counter), 10, 100);

    // println!("최종 카운트: {} (예상: 1000)", counter.get());
}
