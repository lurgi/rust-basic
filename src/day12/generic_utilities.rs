// ========================================
// 과제 1: 제네릭 유틸리티 함수들 (20분)
// ========================================
// 다양한 타입에서 동작하는 유틸리티 함수를 만드세요.
//
// 요구사항:
// - print_type_and_value<T: std::fmt::Debug>(value: T)
//   * 값을 출력하는 제네릭 함수
//   * 힌트: println!("{:?}", value) 사용
//
// - get_last<T>(list: &[T]) -> Option<&T>
//   * 슬라이스의 마지막 요소를 반환
//   * 비어있으면 None
//   * 힌트: list.last() 메서드 사용
//
// - swap_pair<T>(pair: (T, T)) -> (T, T)
//   * 튜플의 순서를 바꾸기
//   * (a, b) -> (b, a)
//
// - find_index<T: PartialEq>(list: &[T], target: &T) -> Option<usize>
//   * 슬라이스에서 target의 인덱스 찾기
//   * 없으면 None
//   * 힌트: iter().position() 사용

// TODO: print_type_and_value 함수를 구현하세요
fn print_type_and_value<T: std::fmt::Debug>(_value: T) {
    unimplemented!("print_type_and_value 함수를 구현하세요")
}

// TODO: get_last 함수를 구현하세요
fn get_last<T>(_list: &[T]) -> Option<&T> {
    unimplemented!("get_last 함수를 구현하세요")
}

// TODO: swap_pair 함수를 구현하세요
fn swap_pair<T>(_pair: (T, T)) -> (T, T) {
    unimplemented!("swap_pair 함수를 구현하세요")
}

// TODO: find_index 함수를 구현하세요
fn find_index<T: PartialEq>(_list: &[T], _target: &T) -> Option<usize> {
    unimplemented!("find_index 함수를 구현하세요")
}

pub fn run() {
    println!("=== 과제 1: 제네릭 유틸리티 함수들 ===");

    // print_type_and_value 테스트
    print_type_and_value(42);
    print_type_and_value("hello");
    print_type_and_value(vec![1, 2, 3]);

    // get_last 테스트
    let numbers = vec![1, 2, 3, 4, 5];
    if let Some(last) = get_last(&numbers) {
        println!("마지막 숫자: {}", last);
    }

    let empty: Vec<i32> = vec![];
    match get_last(&empty) {
        Some(last) => println!("마지막: {}", last),
        None => println!("빈 배열"),
    }

    // swap_pair 테스트
    let pair = (1, 2);
    let swapped = swap_pair(pair);
    println!("원본: {:?}, 교환: {:?}", pair, swapped);

    // find_index 테스트
    let fruits = vec!["apple", "banana", "cherry"];
    match find_index(&fruits, &"banana") {
        Some(idx) => println!("banana의 인덱스: {}", idx),
        None => println!("찾을 수 없음"),
    }
}


