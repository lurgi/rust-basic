// ========================================
// 과제 1: 연관 타입으로 컨테이너 만들기 (25분)
// ========================================
// 연관 타입을 사용하여 다양한 컨테이너를 추상화하세요.
//
// 요구사항:
// - Container 트레잇
//   - type Item; (연관 타입)
//   - fn add(&mut self, item: Self::Item);
//   - fn get(&self, index: usize) -> Option<&Self::Item>;
//   - fn len(&self) -> usize;
//
// - VecContainer 구조체
//   - items: Vec<i32>
//   - Container 트레잇 구현 (Item = i32)
//
// - StringContainer 구조체
//   - items: Vec<String>
//   - Container 트레잇 구현 (Item = String)
//
// - print_container(container: &impl Container)
//   - 컨테이너의 길이를 출력하는 제네릭 함수
//   - "컨테이너 크기: {}" 형식

// TODO: Container 트레잇을 정의하세요 (연관 타입 포함)

// TODO: VecContainer 구조체를 정의하세요

// TODO: VecContainer에 Container 트레잇을 구현하세요

// TODO: StringContainer 구조체를 정의하세요

// TODO: StringContainer에 Container 트레잇을 구현하세요

// TODO: print_container 함수를 구현하세요

pub fn run() {
    println!("=== 과제 1: 연관 타입으로 컨테이너 만들기 ===");

    // let mut vec_container = VecContainer { items: Vec::new() };
    // vec_container.add(10);
    // vec_container.add(20);
    // vec_container.add(30);

    // print_container(&vec_container);

    // if let Some(item) = vec_container.get(1) {
    //     println!("인덱스 1의 값: {}", item);
    // }

    // let mut str_container = StringContainer { items: Vec::new() };
    // str_container.add(String::from("Hello"));
    // str_container.add(String::from("World"));

    // print_container(&str_container);

    // if let Some(item) = str_container.get(0) {
    //     println!("인덱스 0의 값: {}", item);
    // }
}
