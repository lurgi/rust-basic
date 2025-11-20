// ========================================
// 과제 4: Rc<RefCell<T>>로 공유 가변 데이터
// ========================================
// Rc와 RefCell을 조합하여 여러 곳에서 공유하면서 변경도 가능한 구조를 만드세요.
//
// 요구사항:
// - Task 구조체
//   - id: u32
//   - description: String
//   - completed: bool
//
// - TaskManager 구조체
//   - tasks: Vec<Rc<RefCell<Task>>>
//
// - impl TaskManager
//   - new() -> TaskManager
//
//   - add_task(&mut self, id: u32, description: String) -> Rc<RefCell<Task>>
//     * 새 Task를 생성하고 추가
//     * completed는 false로 초기화
//     * 생성된 Task의 Rc<RefCell<Task>> 반환
//
//   - complete_task(&self, task: &Rc<RefCell<Task>>)
//     * 특정 Task의 completed를 true로 변경
//     * 힌트: task.borrow_mut().completed = true;
//
//   - list_tasks(&self)
//     * 모든 Task를 출력
//     * "[완료] Task 1: 설거지" 또는 "[미완료] Task 2: 청소" 형식
//
//   - completed_count(&self) -> usize
//     * 완료된 Task 개수
//     * 힌트: self.tasks.iter()
//               .filter(|task| task.borrow().completed)
//               .count()

use std::cell::RefCell;
use std::rc::Rc;

// TODO: Task 구조체를 정의하세요
struct Task {
    id: u32,
    description: String,
    completed: bool,
}

// TODO: TaskManager 구조체를 정의하세요
struct TaskManager {
    tasks: Vec<Rc<RefCell<Task>>>,
}

// TODO: TaskManager의 모든 메서드를 구현하세요
impl TaskManager {
    fn new() -> TaskManager {
        TaskManager { tasks: Vec::new() }
    }

    fn add_task(&mut self, id: u32, description: String) -> Rc<RefCell<Task>> {
        let new_task = Rc::new(RefCell::new(Task {
            id,
            description,
            completed: false,
        }));
        self.tasks.push(Rc::clone(&new_task));
        new_task
    }

    fn complete_task(&self, task: &Rc<RefCell<Task>>) {
        task.borrow_mut().completed = true;
    }

    fn list_tasks(&self) {
        for task in self.tasks.iter() {
            let t = task.borrow();
            let status = if t.completed { "완료" } else { "미완료" };
            println!("[{}] Task {}: {}", status, t.id, t.description)
        }
    }

    fn completed_count(&self) -> usize {
        self.tasks
            .iter()
            .filter(|task| task.borrow().completed)
            .count()
    }
}

pub fn run() {
    println!("=== 과제 4: Rc<RefCell<T>>로 공유 가변 데이터 ===");

    let mut manager = TaskManager::new();

    let task1 = manager.add_task(1, String::from("설거지"));
    let _task2 = manager.add_task(2, String::from("청소"));
    let task3 = manager.add_task(3, String::from("빨래"));

    println!("초기 상태:");
    manager.list_tasks();

    // task1 완료 처리
    manager.complete_task(&task1);

    // 외부 참조를 통해서도 완료 처리 가능
    task3.borrow_mut().completed = true;

    println!("\n작업 완료 후:");
    manager.list_tasks();

    println!("\n완료된 작업 수: {}", manager.completed_count());
}
