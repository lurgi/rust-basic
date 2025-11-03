// ========================================
// 과제 3: 제네릭 저장소 (25분)
// ========================================
// 여러 타입의 값을 저장할 수 있는 제네릭 저장소를 만드세요.
//
// 요구사항:
// - Storage<T> 구조체
//   - items: Vec<T>
//
// - impl<T> Storage<T>
//   - new() -> Storage<T>
//     * 빈 저장소 생성
//
//   - add(&mut self, item: T)
//     * 아이템 추가
//
//   - get(&self, index: usize) -> Option<&T>
//     * 인덱스로 아이템 가져오기
//
//   - remove(&mut self, index: usize) -> Option<T>
//     * 인덱스의 아이템 제거하고 반환
//     * 힌트: if index < self.items.len() { Some(self.items.remove(index)) } else { None }
//
//   - len(&self) -> usize
//     * 저장된 아이템 개수
//
//   - is_empty(&self) -> bool
//     * 비어있는지 확인
//
// - impl<T: std::fmt::Display> Storage<T>
//   - display_all(&self)
//     * 모든 아이템을 출력
//     * for item in &self.items { println!("{}", item); }

// TODO: Storage 구조체를 정의하세요
#[allow(dead_code)]
struct Storage<T> {
    items: Vec<T>,
}

// TODO: impl<T> Storage<T> 블록을 구현하세요
impl<T> Storage<T> {
    fn new() -> Storage<T> {
        unimplemented!("new 메서드를 구현하세요")
    }

    fn add(&mut self, _item: T) {
        unimplemented!("add 메서드를 구현하세요")
    }

    fn get(&self, _index: usize) -> Option<&T> {
        unimplemented!("get 메서드를 구현하세요")
    }

    fn remove(&mut self, _index: usize) -> Option<T> {
        unimplemented!("remove 메서드를 구현하세요")
    }

    fn len(&self) -> usize {
        unimplemented!("len 메서드를 구현하세요")
    }

    fn is_empty(&self) -> bool {
        unimplemented!("is_empty 메서드를 구현하세요")
    }
}

// TODO: impl<T: std::fmt::Display> Storage<T> 블록을 구현하세요
impl<T: std::fmt::Display> Storage<T> {
    fn display_all(&self) {
        unimplemented!("display_all 메서드를 구현하세요")
    }
}

pub fn run() {
    println!("=== 과제 3: 제네릭 저장소 ===");

    // 정수 저장소
    let mut num_storage = Storage::new();
    num_storage.add(10);
    num_storage.add(20);
    num_storage.add(30);

    println!("저장소 크기: {}", num_storage.len());

    if let Some(value) = num_storage.get(1) {
        println!("인덱스 1의 값: {}", value);
    }

    num_storage.display_all();

    // 문자열 저장소
    let mut str_storage = Storage::new();
    str_storage.add(String::from("Hello"));
    str_storage.add(String::from("World"));
    str_storage.add(String::from("Rust"));

    println!("\n문자열 저장소:");
    str_storage.display_all();

    // 아이템 제거
    if let Some(removed) = str_storage.remove(1) {
        println!("\n제거된 아이템: {}", removed);
    }

    println!("제거 후 크기: {}", str_storage.len());
    str_storage.display_all();
}


