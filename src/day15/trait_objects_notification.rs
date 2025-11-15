// ========================================
// 과제 3: 트레잇 객체로 알림 시스템 (30분)
// ========================================
// 트레잇 객체를 사용하여 다양한 알림 방식을 처리하세요.
//
// 요구사항:
// - Notification 트레잇
//   - fn send(&self, message: &str);
//
// - EmailNotification 구조체
//   - email: String
//   - Notification 트레잇 구현
//     * "이메일 {}로 전송: {}" 형식
//
// - SmsNotification 구조체
//   - phone: String
//   - Notification 트레잇 구현
//     * "SMS {}로 전송: {}" 형식
//
// - PushNotification 구조체
//   - device_id: String
//   - Notification 트레잇 구현
//     * "푸시 알림 (기기 {})로 전송: {}" 형식
//
// - NotificationService 구조체
//   - channels: Vec<Box<dyn Notification>>
//
// - impl NotificationService
//   - new() -> NotificationService
//
//   - add_channel(&mut self, channel: Box<dyn Notification>)
//     * 알림 채널 추가
//
//   - broadcast(&self, message: &str)
//     * 모든 채널로 메시지 전송

// TODO: Notification 트레잇을 정의하세요

// TODO: EmailNotification 구조체를 정의하세요

// TODO: EmailNotification에 Notification 트레잇을 구현하세요

// TODO: SmsNotification 구조체를 정의하세요

// TODO: SmsNotification에 Notification 트레잇을 구현하세요

// TODO: PushNotification 구조체를 정의하세요

// TODO: PushNotification에 Notification 트레잇을 구현하세요

// TODO: NotificationService 구조체를 정의하세요

// TODO: NotificationService의 모든 메서드를 구현하세요

pub fn run() {
    println!("=== 과제 3: 트레잇 객체로 알림 시스템 ===");

    // let mut service = NotificationService::new();

    // // 다양한 알림 채널 추가
    // service.add_channel(Box::new(EmailNotification {
    //     email: String::from("user@example.com"),
    // }));

    // service.add_channel(Box::new(SmsNotification {
    //     phone: String::from("010-1234-5678"),
    // }));

    // service.add_channel(Box::new(PushNotification {
    //     device_id: String::from("device_12345"),
    // }));

    // // 모든 채널로 메시지 전송
    // service.broadcast("시스템 점검 안내");
    // println!();
    // service.broadcast("새로운 메시지가 도착했습니다");
}
