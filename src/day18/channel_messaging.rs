// ========================================
// 과제 2: 채널로 메시지 전달
// ========================================
// mpsc 채널을 사용하여 스레드 간 통신을 구현하세요.
//
// 요구사항:
// - Task 구조체
//   - id: u32
//   - payload: String
//
// - TaskResult 구조체
//   - task_id: u32
//   - result: String
//
// - process_tasks(tasks: Vec<Task>) -> Vec<TaskResult>
//   * 각 Task를 별도의 스레드에서 처리
//   * 스레드는 Task를 받아 처리 후 TaskResult를 채널로 전송
//   * 처리: payload를 대문자로 변환
//   * 모든 결과를 수집하여 반환
//   * 힌트:
//     - let (tx, rx) = mpsc::channel();
//     - 각 스레드에 tx.clone() 전달
//     - rx.iter().take(tasks.len()).collect()

use std::sync::mpsc;
use std::thread;

// TODO: Task 구조체를 정의하세요

// TODO: TaskResult 구조체를 정의하세요

// TODO: process_tasks 함수를 구현하세요

pub fn run() {
    println!("=== 과제 2: 채널로 메시지 전달 ===");

    // let tasks = vec![
    //     Task { id: 1, payload: String::from("hello") },
    //     Task { id: 2, payload: String::from("world") },
    //     Task { id: 3, payload: String::from("rust") },
    // ];

    // let results = process_tasks(tasks);

    // for result in results {
    //     println!("Task {}: {}", result.task_id, result.result);
    // }
}
