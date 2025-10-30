// ========================================
// 과제 3: 파일 읽기 시뮬레이터 (25분)
// ========================================
// Result를 사용하여 파일 읽기를 시뮬레이션하세요.
// (실제 파일을 읽지 않고 시뮬레이션만 합니다)
//
// 요구사항:
// - FileError 열거형
//   - NotFound(String): 파일을 찾을 수 없음
//   - PermissionDenied(String): 권한 없음
//   - InvalidFormat(String): 잘못된 형식
//
// - read_file(filename: &str) -> Result<String, FileError>
//   * "test.txt" → Ok("파일 내용입니다")
//   * "secret.txt" → Err(FileError::PermissionDenied(...))
//   * "data.csv" → Ok("1,2,3")
//   * 그 외 → Err(FileError::NotFound(...))
//
// - parse_csv(content: &str) -> Result<Vec<i32>, FileError>
//   * 쉼표로 분리된 숫자를 파싱
//   * 파싱 실패 시 Err(FileError::InvalidFormat(...))
//   * 힌트: split(',')와 parse() 사용
//
// - read_and_parse(filename: &str) -> Result<Vec<i32>, FileError>
//   * read_file을 호출하고 결과를 parse_csv로 파싱
//   * 두 함수 모두 Result를 반환하므로 match 또는 ?로 처리

// TODO: FileError 열거형을 정의하세요
#[derive(Debug)]
enum FileError {
    NotFound(String),
    PermissionDenied(String),
    InvalidFormat(String),
}

// TODO: read_file 함수를 구현하세요
fn read_file(_filename: &str) -> Result<String, FileError> {
    match _filename {
        "test.txt" => Ok("파일 내용입니다".to_string()),
        "secret.txt" => Err(FileError::PermissionDenied("권한 없음".to_string())),
        "data.csv" => Ok("1,2,3".to_string()),
        _ => Err(FileError::NotFound("파일을 찾을 수 없음".to_string())),
    }
}

// TODO: parse_csv 함수를 구현하세요
fn parse_csv(_content: &str) -> Result<Vec<i32>, FileError> {
    _content
        .split(",")
        .map(|str| {
            str.parse()
                .map_err(|_| FileError::InvalidFormat("잘못된 형식".to_string()))
        })
        .collect()
}

// TODO: read_and_parse 함수를 구현하세요
fn read_and_parse(_filename: &str) -> Result<Vec<i32>, FileError> {
    // read_file(_filename).and_then(|content| parse_csv(&content))
    let content = read_file(_filename)?;
    parse_csv(&content)
}

pub fn run() {
    println!("=== 과제 3: 파일 읽기 시뮬레이터 ===");

    // TODO: read_file 테스트
    match read_file("test.txt") {
        Ok(content) => println!("파일 내용: {}", content),
        Err(e) => println!("에러: {:?}", e),
    }

    match read_file("secret.txt") {
        Ok(content) => println!("파일 내용: {}", content),
        Err(e) => println!("에러: {:?}", e),
    }

    // TODO: read_and_parse 테스트
    match read_and_parse("data.csv") {
        Ok(numbers) => println!("파싱된 숫자: {:?}", numbers),
        Err(e) => println!("에러: {:?}", e),
    }

    match read_and_parse("nonexistent.csv") {
        Ok(numbers) => println!("파싱된 숫자: {:?}", numbers),
        Err(e) => println!("에러: {:?}", e),
    }
}
