// ========================================
// 과제 3: Point 튜플 구조체 (20분)
// ========================================
// 2D 좌표를 나타내는 튜플 구조체를 만들고 거리 계산 메서드를 구현하세요.
//
// 요구사항:
// - Point 튜플 구조체 정의
//   - 두 개의 f64 값 (x, y 좌표)
//
// - 다음 메서드들을 impl 블록에 구현:
//   1. new (연관 함수): x, y를 받아 Point 생성
//
//   2. distance_from_origin (&self) -> f64: 원점으로부터의 거리
//      - 공식: √(x² + y²)
//      - 힌트: f64의 sqrt() 메서드 사용
//
//   3. distance_to (&self, other: &Point) -> f64: 다른 점까지의 거리
//      - 공식: √((x2-x1)² + (y2-y1)²)

// TODO: Point 튜플 구조체를 정의하세요
#[derive(Debug)]
struct Point(f64, f64);

// TODO: impl 블록에 메서드들을 구현하세요
impl Point {
    // TODO: new 연관 함수

    // TODO: distance_from_origin 메서드

    // TODO: distance_to 메서드
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_point_creation() {
        // TODO: Point::new로 점을 생성하고 테스트하세요
    }

    #[test]
    fn test_distance_from_origin() {
        // TODO: distance_from_origin 메서드를 테스트하세요
        // 힌트: Point(3.0, 4.0)의 원점 거리는 5.0
    }

    #[test]
    fn test_distance_to() {
        // TODO: distance_to 메서드를 테스트하세요
    }
}

pub fn run() {
    println!("\n=== 과제 3: Point 튜플 구조체 ===");
    // TODO: Point 테스트
    // 1. Point::new(3.0, 4.0) 생성
    // 2. 원점으로부터의 거리 출력
    // 3. Point::new(0.0, 0.0) 생성
    // 4. 두 점 사이의 거리 출력
}
