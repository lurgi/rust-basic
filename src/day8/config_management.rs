// ========================================
// 과제 3: 설정 관리 모듈 (20분)
// ========================================
// 애플리케이션 설정을 관리하는 모듈을 만드세요.
//
// 요구사항:
// - config 모듈 생성
//   - pub struct AppConfig
//     - app_name: String (비공개)
//     - pub version: String (공개)
//     - pub debug_mode: bool (공개)
//
//   - impl AppConfig
//     - pub fn new(app_name: String, version: String) -> AppConfig
//       * debug_mode는 false로 초기화
//
//     - pub fn get_app_name(&self) -> &str
//       * app_name을 반환 (비공개 필드 접근)
//
//     - pub fn enable_debug(&mut self)
//       * debug_mode를 true로 변경
//
// - main 함수에서 AppConfig 생성 및 사용

// TODO: config 모듈을 정의하세요
mod config {
    // TODO: AppConfig 구조체를 정의하세요
    // 힌트: app_name은 pub 없이, version과 debug_mode는 pub로
    pub struct AppConfig {
        app_name: String,
        pub version: String,
        pub debug_mode: bool,
    }

    // TODO: AppConfig의 impl 블록을 작성하세요
    impl AppConfig {
        pub fn new(app_name: String, version: String) -> AppConfig {
            AppConfig {
                app_name,
                version,
                debug_mode: false,
            }
        }

        pub fn get_app_name(&self) -> &str {
            &self.app_name
        }

        pub fn enable_debug(&mut self) {
            self.debug_mode = true;
        }
    }
}

pub fn run() {
    println!("=== 과제 3: 설정 관리 모듈 ===");
    // TODO: config::AppConfig 사용
    // 1. AppConfig 생성
    let mut app = config::AppConfig::new("새로운 앱".to_string(), "0.0.1".to_string());
    // 2. app_name, version 출력
    println!(
        "앱 이름: {} \n 앱 버전: {}",
        app.get_app_name(),
        app.version
    );
    // 3. debug_mode 확인
    println!("enable_debug 호출 전 {}", app.debug_mode);
    // 4. enable_debug 호출
    app.enable_debug();
    // 5. debug_mode 다시 확인
    println!("enable_debug 호출 후 {}", app.debug_mode);
}
