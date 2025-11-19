// ========================================
// 과제 2: Rc로 공유 데이터 구조
// ========================================
// Rc를 사용하여 여러 곳에서 데이터를 공유하는 구조를 만드세요.
//
// 요구사항:
// - Node 구조체
//   - value: i32
//
// - Graph 구조체
//   - nodes: Vec<Rc<Node>>
//
// - impl Graph
//   - new() -> Graph
//     * 빈 그래프 생성
//
//   - add_node(&mut self, value: i32) -> Rc<Node>
//     * 새 노드를 생성하고 그래프에 추가
//     * 생성된 노드의 Rc 반환
//     * 힌트: let node = Rc::new(Node { value });
//             self.nodes.push(Rc::clone(&node));
//             node
//
//   - node_count(&self) -> usize
//     * 노드 개수 반환
//
//   - total_references(&self) -> usize
//     * 모든 노드의 참조 카운트 합계
//     * 힌트: self.nodes.iter()
//               .map(|node| Rc::strong_count(node))
//               .sum()

use std::rc::Rc;

// TODO: Node 구조체를 정의하세요

// TODO: Graph 구조체를 정의하세요

// TODO: Graph의 모든 메서드를 구현하세요

pub fn run() {
    // println!("=== 과제 2: Rc로 공유 데이터 구조 ===");

    // let mut graph = Graph::new();

    // let node1 = graph.add_node(10);
    // let node2 = graph.add_node(20);

    // println!("노드 개수: {}", graph.node_count());
    // println!("전체 참조 카운트: {}", graph.total_references());

    // // 외부에서 추가 참조 생성
    // let external_ref1 = Rc::clone(&node1);
    // let external_ref2 = Rc::clone(&node1);

    // println!("node1의 참조 카운트: {}", Rc::strong_count(&node1));
    // println!("전체 참조 카운트: {}", graph.total_references());

    // println!("node1 값: {}", node1.value);
    // println!("external_ref1 값: {}", external_ref1.value);
}
