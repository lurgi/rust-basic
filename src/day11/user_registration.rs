// ========================================
// 과제 2: 사용자 등록 시스템 (25분)
// ========================================
// 커스텀 에러 타입을 사용하여 사용자 등록 시스템을 만드세요.
//
// 요구사항:
// - RegistrationError 열거형
//   - InvalidEmail(String): 유효하지 않은 이메일
//   - PasswordTooShort(usize): 비밀번호가 너무 짧음 (현재 길이 포함)
//   - UsernameTaken(String): 이미 사용 중인 사용자명
//   - AgeRestriction(u8): 나이 제한 (현재 나이 포함)
//
// - User 구조체
//   - username: String
//   - email: String
//   - password: String
//   - age: u8
//
// - validate_email(email: &str) -> Result<(), RegistrationError>
//   * '@'가 포함되어 있으면 Ok(())
//   * 없으면 Err(RegistrationError::InvalidEmail(...))
//
// - validate_password(password: &str) -> Result<(), RegistrationError>
//   * 8자 이상이면 Ok(())
//   * 짧으면 Err(RegistrationError::PasswordTooShort(현재_길이))
//
// - validate_age(age: u8) -> Result<(), RegistrationError>
//   * 14세 이상이면 Ok(())
//   * 미만이면 Err(RegistrationError::AgeRestriction(age))
//
// - register_user(username: &str, email: &str, password: &str, age: u8)
//   -> Result<User, RegistrationError>
//   * 모든 검증을 통과하면 User 생성
//   * ? 연산자를 사용하여 각 검증을 순차적으로 수행
//   * 힌트: validate_email(email)?;
//           validate_password(password)?;
//           validate_age(age)?;

// TODO: RegistrationError 열거형을 정의하세요
#[derive(Debug)]
enum RegistrationError {
    // TODO: variant들을 추가하세요
}

// TODO: User 구조체를 정의하세요
#[allow(dead_code)]
struct User {
    // TODO: 필드들을 추가하세요
}

// TODO: validate_email 함수를 구현하세요
fn validate_email(_email: &str) -> Result<(), RegistrationError> {
    unimplemented!("validate_email 함수를 구현하세요")
}

// TODO: validate_password 함수를 구현하세요
fn validate_password(_password: &str) -> Result<(), RegistrationError> {
    unimplemented!("validate_password 함수를 구현하세요")
}

// TODO: validate_age 함수를 구현하세요
fn validate_age(_age: u8) -> Result<(), RegistrationError> {
    unimplemented!("validate_age 함수를 구현하세요")
}

// TODO: register_user 함수를 구현하세요
fn register_user(
    _username: &str,
    _email: &str,
    _password: &str,
    _age: u8,
) -> Result<User, RegistrationError> {
    unimplemented!("register_user 함수를 구현하세요")
}

pub fn run() {
    println!("=== 과제 2: 사용자 등록 시스템 ===");

    // 정상 케이스
    match register_user("alice", "alice@example.com", "password123", 25) {
        Ok(user) => println!("사용자 등록 성공: {}", user.username),
        Err(e) => println!("등록 실패: {:?}", e),
    }

    // 이메일 검증 실패
    match register_user("bob", "bob-invalid", "password123", 25) {
        Ok(user) => println!("사용자 등록 성공: {}", user.username),
        Err(e) => println!("등록 실패: {:?}", e),
    }

    // 비밀번호 검증 실패
    match register_user("charlie", "charlie@example.com", "short", 25) {
        Ok(user) => println!("사용자 등록 성공: {}", user.username),
        Err(e) => println!("등록 실패: {:?}", e),
    }

    // 나이 검증 실패
    match register_user("dave", "dave@example.com", "password123", 12) {
        Ok(user) => println!("사용자 등록 성공: {}", user.username),
        Err(e) => println!("등록 실패: {:?}", e),
    }
}
