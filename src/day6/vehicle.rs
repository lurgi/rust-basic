// ========================================
// 과제 1: 교통수단 열거형 (20분)
// ========================================
// 다양한 교통수단을 표현하는 열거형을 만들고 패턴 매칭을 활용하세요.
//
// 요구사항:
// - Vehicle 열거형 정의
//   - Car(String): 차종 이름
//   - Bicycle: 데이터 없음
//   - Train { line: String, station: String }: 노선과 역
//
// - 다음 함수들을 구현:
//   1. get_travel_time (vehicle: &Vehicle) -> u32
//      - Car: 30분
//      - Bicycle: 45분
//      - Train: 20분
//      - match를 사용하여 구현
//   
//   2. describe (vehicle: &Vehicle) -> String
//      - Car: "차종으로 이동"
//      - Bicycle: "자전거로 이동"
//      - Train: "노선 line의 station역에서 기차로 이동"
//      - match를 사용하여 구현

// TODO: Vehicle 열거형을 정의하세요
#[derive(Debug)]
enum Vehicle {
    // TODO: variant들을 정의하세요
}

// TODO: get_travel_time 함수를 구현하세요
fn get_travel_time(vehicle: &Vehicle) -> u32 {
    // TODO: match를 사용하여 구현
}

// TODO: describe 함수를 구현하세요
fn describe(vehicle: &Vehicle) -> String {
    // TODO: match를 사용하여 구현
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_car_travel_time() {
        // TODO: Car의 travel_time을 테스트하세요
    }

    #[test]
    fn test_bicycle_travel_time() {
        // TODO: Bicycle의 travel_time을 테스트하세요
    }

    #[test]
    fn test_train_travel_time() {
        // TODO: Train의 travel_time을 테스트하세요
    }

    #[test]
    fn test_car_describe() {
        // TODO: Car의 describe를 테스트하세요
    }

    #[test]
    fn test_bicycle_describe() {
        // TODO: Bicycle의 describe를 테스트하세요
    }

    #[test]
    fn test_train_describe() {
        // TODO: Train의 describe를 테스트하세요
    }
}

pub fn run() {
    println!("\n=== 과제 1: 교통수단 열거형 ===");
    // TODO: Vehicle 테스트
    // 1. Car, Bicycle, Train 인스턴스 생성
    // 2. 각각의 travel_time 출력
    // 3. 각각의 describe 출력
}

