// ========================================
// 과제 3: 메시지 처리 시스템 (20분)
// ========================================
// 메시지 타입을 표현하는 열거형을 만들고 처리 함수를 구현하세요.
//
// 요구사항:
// - Message 열거형 정의
//   - Text(String): 텍스트 메시지
//   - Image { url: String, width: u32, height: u32 }: 이미지
//   - Video { url: String, duration: u32 }: 비디오 (초 단위)
//
// - process_message 함수 구현
//   - Message를 받아서 처리
//   - Text: "텍스트 메시지: 내용" 출력
//   - Image: "이미지 (widthxheight): url" 출력
//   - Video: "비디오 (duration초): url" 출력
//   - match를 사용하여 구현
//
// - filter_long_videos 함수 구현
//   - Vec<Message>를 받아서 60초 이상인 비디오만 필터링
//   - 결과를 Vec<Message>로 반환
//   - for 루프와 if let을 사용하여 구현

// TODO: Message 열거형을 정의하세요
#[derive(Debug)]
enum Message {
    // TODO: variant들을 정의하세요
}

// TODO: process_message 함수를 구현하세요
fn process_message(msg: &Message) {
    // TODO: match를 사용하여 구현
}

// TODO: filter_long_videos 함수를 구현하세요
fn filter_long_videos(messages: Vec<Message>) -> Vec<Message> {
    // TODO: for 루프와 if let을 사용하여 구현
    // 힌트: 
    // - 빈 Vec를 만들고
    // - for 루프로 messages를 순회하면서
    // - if let으로 Video인지 확인하고
    // - duration이 60 이상이면 결과 Vec에 추가
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_process_text() {
        // TODO: Text 메시지 처리를 테스트하세요
    }

    #[test]
    fn test_process_image() {
        // TODO: Image 메시지 처리를 테스트하세요
    }

    #[test]
    fn test_process_video() {
        // TODO: Video 메시지 처리를 테스트하세요
    }

    #[test]
    fn test_filter_long_videos() {
        // TODO: filter_long_videos 함수를 테스트하세요
        // 짧은 비디오, 긴 비디오, 텍스트를 포함한 Vec를 만들고
        // 필터링 결과가 긴 비디오만 포함하는지 확인
    }
}

pub fn run() {
    println!("\n=== 과제 3: 메시지 처리 시스템 ===");
    // TODO: Message 테스트
    // 1. 여러 종류의 Message 생성 (Text, Image, Video)
    // 2. 각 메시지를 process_message로 처리
    // 3. Vec<Message>를 만들어서 filter_long_videos 테스트
    //    - 짧은 비디오 (30초)
    //    - 긴 비디오 (90초)
    //    - 텍스트 메시지
    //    - 긴 비디오 (120초)
    // 4. 필터링된 결과 출력 (60초 이상 비디오만 나와야 함)
}

