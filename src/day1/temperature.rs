// 과제 1: 온도 변환기
// 섭씨를 화씨로 변환하는 함수
// 공식: F = C × 1.8 + 32
pub fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 1.8 + 32.0
}

// 화씨를 섭씨로 변환하는 함수
// 공식: C = (F - 32) / 1.8
pub fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) / 1.8
}
