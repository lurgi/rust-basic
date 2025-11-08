// ========================================
// 과제 1: 도형 트레잇 (20분)
// ========================================
// 도형들이 공통으로 가져야 할 동작을 트레잇으로 정의하세요.
//
// 요구사항:
// - Shape 트레잇
//   - area(&self) -> f64: 넓이 계산
//   - perimeter(&self) -> f64: 둘레 계산
//
// - Rectangle 구조체
//   - width: f64
//   - height: f64
//   - Shape 트레잇 구현
//
// - Circle 구조체
//   - radius: f64
//   - Shape 트레잇 구현
//   - 원주율 사용: std::f64::consts::PI
//
// - print_shape_info<T: Shape>(shape: &T)
//   - 도형의 넓이와 둘레를 출력하는 제네릭 함수

// TODO: Shape 트레잇을 정의하세요

use std::f64::consts::PI;

trait Shape {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
}

// TODO: Rectangle 구조체를 정의하세요
struct Rectangle {
    width: f64,
    height: f64,
}

// TODO: Rectangle에 Shape 트레잇을 구현하세요
impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.height * self.width
    }
    fn perimeter(&self) -> f64 {
        2.0 * (self.height + self.width)
    }
}

// TODO: Circle 구조체를 정의하세요
struct Circle {
    radius: f64,
}

// TODO: Circle에 Shape 트레잇을 구현하세요
impl Shape for Circle {
    fn area(&self) -> f64 {
        self.radius.powi(2) * PI
    }
    fn perimeter(&self) -> f64 {
        2.0 * self.radius * PI
    }
}

// TODO: print_shape_info 함수를 구현하세요
fn print_shape_info<T: Shape>(shape: &T) {
    println!("도형의 넓이: {:?}", shape.area());
    println!("도형의 둘레: {:?}", shape.perimeter());
}

pub fn run() {
    println!("=== 과제 1: 도형 트레잇 ===");

    let rect = Rectangle {
        width: 10.0,
        height: 5.0,
    };

    let circle = Circle { radius: 7.0 };

    print_shape_info(&rect);
    print_shape_info(&circle);
}
