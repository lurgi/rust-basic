// ========================================
// 과제 1: Box로 연결 리스트 만들기
// ========================================
// Box를 사용하여 간단한 연결 리스트를 구현하세요.
//
// 요구사항:
// - List 열거형
//   - Cons(i32, Box<List>): 값과 다음 노드
//   - Nil: 리스트의 끝
//
// - impl List
//   - new() -> List
//     * 빈 리스트 생성 (Nil)
//
//   - prepend(self, value: i32) -> List
//     * 리스트 앞에 값 추가
//     * Cons(value, Box::new(self)) 반환
//
//   - len(&self) -> usize
//     * 리스트의 길이 계산 (재귀적으로)
//     * 힌트: match self {
//               List::Cons(_, tail) => 1 + tail.len(),
//               List::Nil => 0,
//             }
//
//   - stringify(&self) -> String
//     * "1 -> 2 -> 3 -> Nil" 형식으로 문자열 생성

// TODO: List 열거형을 정의하세요

// TODO: List의 모든 메서드를 구현하세요

pub fn run() {
    // println!("=== 과제 1: Box로 연결 리스트 만들기 ===");

    // let list = List::new().prepend(3).prepend(2).prepend(1);

    // println!("리스트: {}", list.stringify());
    // println!("길이: {}", list.len());
}
