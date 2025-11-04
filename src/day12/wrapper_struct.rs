// ========================================
// 과제 2: 제네릭 래퍼 구조체 (25분)
// ========================================
// 값을 감싸는 제네릭 래퍼 구조체를 만드세요.
//
// 요구사항:
// - Wrapper<T> 구조체
//   - value: T
//
// - impl<T> Wrapper<T>
//   - new(value: T) -> Wrapper<T>
//     * 새 래퍼 생성
//
//   - get(&self) -> &T
//     * 값의 참조 반환
//
//   - unwrap(self) -> T
//     * 래퍼를 소비하고 내부 값 반환
//
//   - map<U, F>(self, f: F) -> Wrapper<U>
//     where F: FnOnce(T) -> U
//     * 내부 값을 변환하여 새 Wrapper 생성
//     * 힌트: Wrapper::new(f(self.value))
//
// - impl<T: std::fmt::Display> Wrapper<T>
//   - display(&self)
//     * 내부 값을 출력
//     * Display 트레잇이 있는 타입에만 구현

// TODO: Wrapper 구조체를 정의하세요
#[allow(dead_code)]
struct Wrapper<T> {
    value: T,
}

// TODO: impl<T> Wrapper<T> 블록을 구현하세요
impl<T> Wrapper<T> {
    fn new(_value: T) -> Wrapper<T> {
        Wrapper { value: _value }
    }

    fn get(&self) -> &T {
        &self.value
    }

    fn unwrap(self) -> T {
        self.value
    }

    fn map<U, F>(self, _f: F) -> Wrapper<U>
    where
        F: FnOnce(T) -> U,
    {
        Wrapper {
            value: _f(self.value),
        }
    }
}

// TODO: impl<T: std::fmt::Display> Wrapper<T> 블록을 구현하세요
impl<T: std::fmt::Display> Wrapper<T> {
    fn display(&self) {
        println!("{}", self.value)
    }
}

pub fn run() {
    println!("=== 과제 2: 제네릭 래퍼 구조체 ===");

    // 정수 래퍼
    let num_wrapper = Wrapper::new(42);
    println!("값: {}", num_wrapper.get());
    num_wrapper.display();

    // map으로 변환
    let str_wrapper = Wrapper::new(5).map(|x| format!("숫자: {}", x));
    println!("변환된 값: {}", str_wrapper.get());

    // unwrap으로 값 추출
    let text = Wrapper::new(String::from("Hello"));
    let extracted = text.unwrap();
    println!("추출된 값: {}", extracted);
}
