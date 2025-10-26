// ========================================
// 7일차 통합 과제 (90분)
// ========================================
// 1주차에 배운 모든 개념을 활용하여 Todo 관리 시스템을 만들어보세요.
//
// ========================================
// Todo 관리 시스템
// ========================================
//
// 요구사항:
//
// 1. TodoStatus 열거형
//    - Pending: 대기 중
//    - InProgress: 진행 중
//    - Completed: 완료
//
// 2. Todo 구조체
//    - id: u32
//    - title: String
//    - description: String
//    - status: TodoStatus
//
// 3. TodoList 구조체
//    - todos: Vec<Todo>
//    - next_id: u32
//
// 4. Todo 메서드들
//    - new(id: u32, title: String, description: String) -> Todo
//      * status는 Pending으로 초기화
//
//    - start(&mut self)
//      * status를 InProgress로 변경
//
//    - complete(&mut self)
//      * status를 Completed로 변경
//
//    - is_completed(&self) -> bool
//      * Completed 상태인지 확인
//
//    - status_text(&self) -> &str
//      * status를 문자열로 반환
//      * Pending -> "대기중", InProgress -> "진행중", Completed -> "완료"
//
// 5. TodoList 메서드들
//    - new() -> TodoList
//      * 빈 TodoList 생성
//
//    - add(&mut self, title: String, description: String) -> u32
//      * 새로운 Todo를 추가하고 id를 반환
//      * next_id를 1 증가
//
//    - get(&self, id: u32) -> Option<&Todo>
//      * id로 Todo를 찾아서 참조 반환
//      * 없으면 None 반환
//
//    - get_mut(&mut self, id: u32) -> Option<&mut Todo>
//      * id로 Todo를 찾아서 가변 참조 반환
//      * 없으면 None 반환
//
//    - remove(&mut self, id: u32) -> bool
//      * id로 Todo를 찾아서 삭제
//      * 성공하면 true, 실패하면 false
//
//    - list_all(&self) -> &Vec<Todo>
//      * 모든 Todo의 참조 반환
//
//    - list_by_status(&self, status: TodoStatus) -> Vec<&Todo>
//      * 특정 status의 Todo들만 필터링하여 반환
//
//    - count_by_status(&self, status: TodoStatus) -> usize
//      * 특정 status의 Todo 개수 반환

use std::future::Pending;

// TODO: TodoStatus 열거형을 정의하세요
#[derive(Debug, PartialEq, Clone)]
enum TodoStatus {
    // TODO: Pending, InProgress, Completed 정의
    //
    Pending,
    InProgress,
    Completed,
}

// TODO: Todo 구조체를 정의하세요
#[derive(Debug)]
struct Todo {
    // TODO: id, title, description, status 필드 정의
    id: u32,
    title: String,
    description: String,
    status: TodoStatus,
}

impl Todo {
    // TODO: new 메서드 구현
    fn new(id: u32, title: String, description: String) -> Todo {
        Todo {
            id,
            title,
            description,
            status: TodoStatus::Pending,
        }
    }

    // TODO: start 메서드 구현
    fn start(&mut self) {
        self.status = TodoStatus::InProgress
    }

    // TODO: complete 메서드 구현
    fn complete(&mut self) {
        self.status = TodoStatus::Completed
    }

    // TODO: is_completed 메서드 구현
    fn is_completed(&self) -> bool {
        if let TodoStatus::Completed = self.status {
            return true;
        }
        false
    }

    // TODO: status_text 메서드 구현
    fn status_text(&self) -> &str {
        match self.status {
            TodoStatus::Pending => "대기중",
            TodoStatus::InProgress => "진행중",
            TodoStatus::Completed => "완료",
        }
    }
}

// TODO: TodoList 구조체를 정의하세요
#[derive(Debug)]
struct TodoList {
    // TODO: todos, next_id 필드 정의
    todos: Vec<Todo>,
    next_id: u32,
}

// TODO: TodoList의 impl 블록을 작성하세요
impl TodoList {
    // TODO: new 메서드 구현
    fn new() -> TodoList {
        TodoList {
            todos: Vec::new(),
            next_id: 0,
        }
    }
    // TODO: add 메서드 구현
    fn add(&mut self, title: String, description: String) -> u32 {
        let current_id = self.next_id;
        self.todos.push(Todo::new(current_id, title, description));
        self.next_id += 1;
        current_id
    }

    // TODO: get 메서드 구현
    fn get(&self, id: u32) -> Option<&Todo> {
        self.todos.iter().find(|todo| todo.id == id)
    }

    // TODO: get_mut 메서드 구현
    fn get_mut(&mut self, id: u32) -> Option<&mut Todo> {
        self.todos.iter_mut().find(|todo| todo.id == id)
    }

    fn remove(&mut self, id: u32) -> bool {
        if let Some(pos) = self.todos.iter().position(|todo| todo.id == id) {
            self.todos.swap_remove(pos);
            true
        } else {
            false
        }
    }

    // TODO: list_all 메서드 구현
    fn list_all(&self) -> &Vec<Todo> {
        &self.todos
    }

    fn list_by_status(&self, status: TodoStatus) -> Vec<&Todo> {
        self.todos
            .iter()
            .filter(|todo| todo.status == status)
            .collect()
    }

    // TODO: count_by_status 메서드 구현
    fn count_by_status(&self, status: TodoStatus) -> usize {
        self.todos
            .iter()
            .filter(|todo| todo.status == status)
            .count()
    }
}

pub fn run() {
    println!("\n=== Todo 관리 시스템 ===\n");

    // TodoList 생성
    let mut todo_list = TodoList::new();

    // Todo 추가
    println!("1. Todo 추가하기");
    let id1 = todo_list.add(
        String::from("Rust 공부하기"),
        String::from("소유권과 빌림 복습"),
    );
    let id2 = todo_list.add(
        String::from("프로젝트 시작하기"),
        String::from("Todo 앱 만들기"),
    );
    let id3 = todo_list.add(String::from("운동하기"), String::from("30분 러닝"));
    println!("추가된 Todo: {}, {}, {}\n", id1, id2, id3);

    // 모든 Todo 출력
    println!("2. 모든 Todo 목록");
    for todo in todo_list.list_all() {
        println!(
            "  [{}] {} - {} ({})",
            todo.id,
            todo.title,
            todo.description,
            todo.status_text()
        );
    }
    println!();

    // Todo 상태 변경
    println!("3. Todo 상태 변경");
    if let Some(todo) = todo_list.get_mut(id1) {
        todo.start();
        println!("  Todo {} 시작: {}", id1, todo.status_text());
    }

    if let Some(todo) = todo_list.get_mut(id2) {
        todo.start();
        todo.complete();
        println!("  Todo {} 완료: {}", id2, todo.status_text());
    }
    println!();

    // 상태별 조회
    println!("4. 상태별 Todo 조회");
    let pending = todo_list.list_by_status(TodoStatus::Pending);
    println!("  대기중: {}개", pending.len());
    for todo in pending {
        println!("    - {}", todo.title);
    }

    let in_progress = todo_list.list_by_status(TodoStatus::InProgress);
    println!("  진행중: {}개", in_progress.len());
    for todo in in_progress {
        println!("    - {}", todo.title);
    }

    let completed = todo_list.list_by_status(TodoStatus::Completed);
    println!("  완료: {}개", completed.len());
    for todo in completed {
        println!("    - {}", todo.title);
    }
    println!();

    // 상태별 개수
    println!("5. 상태별 통계");
    println!(
        "  대기중: {}개",
        todo_list.count_by_status(TodoStatus::Pending)
    );
    println!(
        "  진행중: {}개",
        todo_list.count_by_status(TodoStatus::InProgress)
    );
    println!(
        "  완료: {}개",
        todo_list.count_by_status(TodoStatus::Completed)
    );
    println!();

    // Todo 삭제
    println!("6. Todo 삭제");
    let removed = todo_list.remove(id2);
    if removed {
        println!("  Todo {} 삭제 성공", id2);
    }

    // 최종 목록
    println!("\n7. 최종 Todo 목록");
    for todo in todo_list.list_all() {
        println!("  [{}] {} ({})", todo.id, todo.title, todo.status_text());
    }
}
