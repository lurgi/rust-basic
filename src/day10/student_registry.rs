// ========================================
// 과제 2: 학생 정보 조회 시스템 (20분)
// ========================================
// Option을 사용하여 학생 정보를 조회하는 시스템을 만드세요.
//
// 요구사항:
// - Student 구조체
//   - name: String
//   - age: u8
//   - grade: Option<char>  // 학점이 없을 수 있음
//
// - StudentRegistry 구조체
//   - students: Vec<Student>
//
// - impl StudentRegistry
//   - new() -> StudentRegistry
//     * 빈 레지스트리 생성
//
//   - add_student(&mut self, student: Student)
//     * 학생 추가
//
//   - find_by_name(&self, name: &str) -> Option<&Student>
//     * 이름으로 학생 찾기
//     * 없으면 None
//
//   - get_grade(&self, name: &str) -> Option<char>
//     * 학생을 찾고, 학점이 있으면 반환
//     * 학생이 없거나 학점이 None이면 None

// TODO: Student 구조체를 정의하세요
#[derive(Debug)]
struct Student {
    // TODO
}

// TODO: StudentRegistry 구조체를 정의하세요
struct StudentRegistry {
    // TODO
}

// TODO: StudentRegistry의 impl 블록을 작성하세요
impl StudentRegistry {
    // TODO: new 메서드를 구현하세요
    fn new() -> StudentRegistry {
        unimplemented!("new 메서드를 구현하세요")
    }

    // TODO: add_student 메서드를 구현하세요
    fn add_student(&mut self, _student: Student) {
        unimplemented!("add_student 메서드를 구현하세요")
    }

    // TODO: find_by_name 메서드를 구현하세요
    fn find_by_name(&self, _name: &str) -> Option<&Student> {
        unimplemented!("find_by_name 메서드를 구현하세요")
    }

    // TODO: get_grade 메서드를 구현하세요
    fn get_grade(&self, _name: &str) -> Option<char> {
        unimplemented!("get_grade 메서드를 구현하세요")
    }
}

pub fn run() {
    println!("=== 과제 2: 학생 정보 조회 시스템 ===");

    let mut registry = StudentRegistry::new();

    // TODO: 학생 추가
    registry.add_student(Student {
        name: String::from("Alice"),
        age: 20,
        grade: Some('A'),
    });

    registry.add_student(Student {
        name: String::from("Bob"),
        age: 21,
        grade: Some('B'),
    });

    registry.add_student(Student {
        name: String::from("Charlie"),
        age: 19,
        grade: None, // 학점 없음
    });

    // TODO: 학생 조회
    if let Some(student) = registry.find_by_name("Alice") {
        println!("찾은 학생: {:?}", student);
    }

    // TODO: 학점 조회
    match registry.get_grade("Alice") {
        Some(grade) => println!("Alice의 학점: {}", grade),
        None => println!("Alice의 학점 정보 없음"),
    }

    match registry.get_grade("Charlie") {
        Some(grade) => println!("Charlie의 학점: {}", grade),
        None => println!("Charlie의 학점 정보 없음"),
    }
}
