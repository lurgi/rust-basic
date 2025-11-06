// ========================================
// 과제 2: 비교 가능한 제품 (25분)
// ========================================
// 제품을 비교할 수 있도록 표준 트레잇들을 구현하세요.
//
// 요구사항:
// - Product 구조체
//   - name: String
//   - price: u32
//   - #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
//
// - impl Display for Product
//   - "{name}: {price}원" 형식으로 출력
//
// - find_cheapest<T: Ord>(items: &[T]) -> &T
//   - 슬라이스에서 가장 작은 값 찾기
//   * 힌트: items.iter().min().unwrap()
//
// - find_most_expensive<T: Ord>(items: &[T]) -> &T
//   - 슬라이스에서 가장 큰 값 찾기
//   * 힌트: items.iter().max().unwrap()

use std::fmt;

// TODO: Product 구조체를 정의하세요 (derive 사용)

// TODO: Display 트레잇을 구현하세요

// TODO: find_cheapest 함수를 구현하세요

// TODO: find_most_expensive 함수를 구현하세요

pub fn run() {
    println!("=== 과제 2: 비교 가능한 제품 ===");

    // let products = vec![
    //     Product {
    //         name: String::from("키보드"),
    //         price: 50000,
    //     },
    //     Product {
    //         name: String::from("마우스"),
    //         price: 30000,
    //     },
    //     Product {
    //         name: String::from("모니터"),
    //         price: 300000,
    //     },
    // ];

    // let cheapest = find_cheapest(&products);
    // let most_expensive = find_most_expensive(&products);

    // println!("가장 저렴한 제품: {}", cheapest);
    // println!("가장 비싼 제품: {}", most_expensive);

    // // Clone 테스트
    // let cloned = cheapest.clone();
    // println!("복제된 제품: {}", cloned);

    // // PartialEq 테스트
    // if cheapest == &cloned {
    //     println!("원본과 복제본이 같습니다");
    // }
}

