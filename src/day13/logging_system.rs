// ========================================
// 과제 3: 로깅 시스템 (25분)
// ========================================
// 로그를 기록하는 트레잇을 만들고 여러 방식으로 구현하세요.
//
// 요구사항:
// - Logger 트레잇
//   - log(&self, message: &str): 로그 기록 (기본 구현 없음)
//   - log_error(&self, message: &str): 에러 로그
//     * 기본 구현: self.log(&format!("[ERROR] {}", message))
//   - log_info(&self, message: &str): 정보 로그
//     * 기본 구현: self.log(&format!("[INFO] {}", message))
//
// - ConsoleLogger 구조체
//   - Logger 트레잇 구현
//   - log 메서드: println!으로 콘솔에 출력
//
// - FileLogger 구조체
//   - filename: String
//   - Logger 트레잇 구현
//   - log 메서드: "파일 {filename}에 기록: {message}" 형식으로 출력
//     (실제 파일 쓰기는 하지 않고 시뮬레이션만)
//
// - log_multiple<T: Logger>(logger: &T, messages: &[&str])
//   - 여러 메시지를 한 번에 로깅
//   - 각 메시지를 logger.log()로 기록

// TODO: Logger 트레잇을 정의하세요

// TODO: ConsoleLogger 구조체를 정의하세요

// TODO: ConsoleLogger에 Logger 트레잇을 구현하세요

// TODO: FileLogger 구조체를 정의하세요

// TODO: FileLogger에 Logger 트레잇을 구현하세요

// TODO: log_multiple 함수를 구현하세요

pub fn run() {
    println!("=== 과제 3: 로깅 시스템 ===");

    // let console_logger = ConsoleLogger;
    // let file_logger = FileLogger {
    //     filename: String::from("app.log"),
    // };

    // // ConsoleLogger 테스트
    // println!("\n[ConsoleLogger 테스트]");
    // console_logger.log("일반 메시지");
    // console_logger.log_info("정보 메시지");
    // console_logger.log_error("에러 발생!");

    // // FileLogger 테스트
    // println!("\n[FileLogger 테스트]");
    // file_logger.log("일반 메시지");
    // file_logger.log_info("정보 메시지");
    // file_logger.log_error("에러 발생!");

    // // log_multiple 테스트
    // println!("\n[Multiple 로깅 테스트]");
    // let messages = vec!["첫 번째 메시지", "두 번째 메시지", "세 번째 메시지"];
    // log_multiple(&console_logger, &messages);
}
