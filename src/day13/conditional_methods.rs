// ========================================
// 과제 4: 조건부 메서드 (20분)
// ========================================
// 특정 트레잇 바운드가 있을 때만 메서드를 제공하세요.
//
// 요구사항:
// - Container<T> 구조체
//   - value: T
//
// - impl<T> Container<T>
//   - new(value: T) -> Container<T>
//
// - impl<T: Clone> Container<T>
//   - duplicate(&self) -> (T, T)
//     * value를 복제하여 두 개로 반환
//     * (self.value.clone(), self.value.clone())
//
// - impl<T: PartialOrd> Container<T>
//   - is_greater_than(&self, other: &T) -> bool
//     * self.value > *other
//
// - impl<T: std::fmt::Display> Container<T>
//   - display(&self)
//     * println!("Container: {}", self.value)

// TODO: Container 구조체를 정의하세요

// TODO: 모든 타입에 대한 impl 블록

// TODO: Clone 트레잇이 있는 타입에 대한 impl 블록

// TODO: PartialOrd 트레잇이 있는 타입에 대한 impl 블록

// TODO: Display 트레잇이 있는 타입에 대한 impl 블록

pub fn run() {
    println!("=== 과제 4: 조건부 메서드 ===");

    // // i32는 Clone, PartialOrd, Display 모두 구현
    // let num_container = Container::new(42);
    // let (a, b) = num_container.duplicate();
    // println!("복제된 값: {}, {}", a, b);

    // println!("10보다 큰가? {}", num_container.is_greater_than(&10));
    // num_container.display();

    // // String은 Clone과 Display 구현
    // let str_container = Container::new(String::from("Hello"));
    // let (s1, s2) = str_container.duplicate();
    // println!("복제된 문자열: {}, {}", s1, s2);
    // str_container.display();

    // // str_container.is_greater_than(&String::from("Hi")); // OK (String은 PartialOrd 구현)
}

