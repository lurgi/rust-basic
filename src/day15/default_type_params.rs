// ========================================
// 과제 2: 기본 타입 매개변수로 포인트 연산 (25분)
// ========================================
// 기본 타입 매개변수를 사용하여 포인트 덧셈을 구현하세요.
//
// 요구사항:
// - Point<T = i32> 구조체
//   - x: T
//   - y: T
//
// - impl<T> Point<T>
//   - new(x: T, y: T) -> Point<T>
//
// - impl Add for Point<i32>
//   - 기본 타입(i32)에 대한 덧셈 구현
//   - Point + Point = Point
//
// - impl<T: std::fmt::Display> Point<T>
//   - display(&self)
//     * "({}, {})" 형식으로 출력

use std::ops::Add;

// TODO: Point 구조체를 정의하세요 (기본 타입 매개변수 사용)

// TODO: 모든 타입에 대한 impl 블록 (new 메서드)

// TODO: Point<i32>에 대한 Add 트레잇 구현

// TODO: Display 트레잇이 있는 타입에 대한 impl 블록 (display 메서드)

pub fn run() {
    println!("=== 과제 2: 기본 타입 매개변수로 포인트 연산 ===");

    // // 기본 타입(i32) 사용
    // let p1 = Point::new(1, 2);
    // let p2 = Point::new(3, 4);
    // let p3 = p1 + p2;
    // p3.display();

    // // 다른 타입(f64) 사용
    // let p4 = Point::new(1.5, 2.5);
    // let p5 = Point::new(3.5, 4.5);
    // p4.display();
    // p5.display();
}
