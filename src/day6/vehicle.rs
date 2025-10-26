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
    Car(String),
    Bicycle,
    Train { line: String, station: String },
}

// TODO: get_travel_time 함수를 구현하세요
fn get_travel_time(vehicle: &Vehicle) -> u32 {
    // TODO: match를 사용하여 구현
    match vehicle {
        Vehicle::Car(str) => 30,
        Vehicle::Bicycle => 45,
        Vehicle::Train { line, station } => 20,
    }
}

// TODO: describe 함수를 구현하세요
fn describe(vehicle: &Vehicle) -> String {
    // TODO: match를 사용하여 구현
    match vehicle {
        Vehicle::Car(str) => format!("차종으로 이동"),
        Vehicle::Bicycle => format!("자전거로 이동"),
        Vehicle::Train { line, station } => {
            format!("노선 {}의 {}역에서 기차로 이동", line, station)
        }
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_car_travel_time() {
        // TODO: Car의 travel_time을 테스트하세요
        let car = Vehicle::Car(String::from("차종"));
        assert_eq!(get_travel_time(&car), 30);
    }

    #[test]
    fn test_bicycle_travel_time() {
        // TODO: Bicycle의 travel_time을 테스트하세요
        let bicycle = Vehicle::Bicycle;
        assert_eq!(get_travel_time(&bicycle), 45);
    }

    #[test]
    fn test_train_travel_time() {
        // TODO: Train의 travel_time을 테스트하세요
        let train = Vehicle::Train {
            line: String::from("1호선"),
            station: String::from("역삼역"),
        };
        assert_eq!(get_travel_time(&train), 20);
    }

    #[test]
    fn test_car_describe() {
        // TODO: Car의 describe를 테스트하세요
        let car = Vehicle::Car(String::from("차종"));
        assert_eq!(describe(&car), "차종으로 이동");
    }

    #[test]
    fn test_bicycle_describe() {
        // TODO: Bicycle의 describe를 테스트하세요
        let bicycle = Vehicle::Bicycle;
        assert_eq!(describe(&bicycle), "자전거로 이동");
    }

    #[test]
    fn test_train_describe() {
        // TODO: Train의 describe를 테스트하세요
        let train = Vehicle::Train {
            line: String::from("1호선"),
            station: String::from("역삼역"),
        };
        assert_eq!(describe(&train), "노선 1호선의 역삼역에서 기차로 이동");
    }
}

pub fn run() {
    println!("\n=== 과제 1: 교통수단 열거형 ===");
    // TODO: Vehicle 테스트
    // 1. Car, Bicycle, Train 인스턴스 생성
    let car = Vehicle::Car("차종".to_string());
    let bicycle = Vehicle::Bicycle;
    let train = Vehicle::Train {
        line: "1호선".to_string(),
        station: "역삼역".to_string(),
    };
    // 2. 각각의 travel_time 출력
    println!("차종의 이동 시간: {}", get_travel_time(&car));
    println!("자전거의 이동 시간: {}", get_travel_time(&bicycle));
    println!("기차의 이동 시간: {}", get_travel_time(&train));
    // 3. 각각의 describe 출력
    println!("차종의 설명: {}", describe(&car));
    println!("자전거의 설명: {}", describe(&bicycle));
    println!("기차의 설명: {}", describe(&train));
}
