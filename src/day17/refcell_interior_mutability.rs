// ========================================
// 과제 3: RefCell로 내부 가변성 구현
// ========================================
// RefCell을 사용하여 불변 참조에서도 값을 변경할 수 있는 구조를 만드세요.
//
// 요구사항:
// - Counter 구조체
//   - count: RefCell<i32>
//
// - impl Counter
//   - new() -> Counter
//     * count를 0으로 초기화
//
//   - increment(&self)
//     * count를 1 증가 (&self인데도 내부 값 변경)
//     * 힌트: *self.count.borrow_mut() += 1;
//
//   - get(&self) -> i32
//     * 현재 count 값 반환
//     * 힌트: *self.count.borrow()
//
//   - reset(&self)
//     * count를 0으로 리셋

use std::cell::RefCell;

// TODO: Counter 구조체를 정의하세요

// TODO: Counter의 모든 메서드를 구현하세요

pub fn run() {
    // println!("=== 과제 3: RefCell로 내부 가변성 구현 ===");

    // let counter = Counter::new();

    // println!("초기 값: {}", counter.get());

    // // &self만 받지만 내부 값 변경 가능
    // counter.increment();
    // counter.increment();
    // counter.increment();

    // println!("증가 후: {}", counter.get());

    // counter.reset();
    // println!("리셋 후: {}", counter.get());
}
