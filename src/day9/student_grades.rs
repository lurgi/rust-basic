// ========================================
// 과제 1: 학생 성적 관리 (20분)
// ========================================
// Vec를 사용하여 학생들의 점수를 관리하고 통계를 계산하세요.
//
// 요구사항:
// - calculate_average(scores: &Vec<i32>) -> f64
//   * 평균 점수 계산
//
// - find_highest(scores: &Vec<i32>) -> Option<i32>
//   * 최고 점수 찾기
//   * 빈 벡터면 None 반환
//
// - find_lowest(scores: &Vec<i32>) -> Option<i32>
//   * 최저 점수 찾기
//   * 빈 벡터면 None 반환
//
// - filter_passing(scores: &Vec<i32>, passing_score: i32) -> Vec<i32>
//   * 합격 점수 이상인 점수들만 필터링
//
// 힌트:
// - 평균은 합계 / 개수
// - 최고/최저는 첫 번째 값으로 초기화하고 비교

// TODO: calculate_average 함수를 구현하세요
fn calculate_average(_scores: &Vec<i32>) -> f64 {
    if _scores.len() == 0 {
        return 0.0;
    }

    let sum: i32 = _scores.iter().sum();
    sum as f64 / _scores.len() as f64
}

// TODO: find_highest 함수를 구현하세요
fn find_highest(_scores: &Vec<i32>) -> Option<i32> {
    if _scores.is_empty() {
        return None;
    }

    let mut highest = _scores[0];
    for &score in _scores {
        if highest < score {
            highest = score;
        }
    }
    Some(highest)
}

// TODO: find_lowest 함수를 구현하세요
fn find_lowest(_scores: &Vec<i32>) -> Option<i32> {
    if _scores.is_empty() {
        return None;
    }

    let mut lowest = _scores[0];
    for &score in _scores {
        if lowest >= score {
            lowest = score
        }
    }
    Some(lowest)
}

// TODO: filter_passing 함수를 구현하세요
fn filter_passing(_scores: &Vec<i32>, _passing_score: i32) -> Vec<i32> {
    let mut new_vec = Vec::new();

    for &score in _scores {
        if _passing_score < score {
            new_vec.push(score)
        }
    }
    new_vec
}

pub fn run() {
    println!("=== 과제 1: 학생 성적 관리 ===");
    let scores = vec![85, 92, 78, 95, 88, 76, 90, 82];

    // TODO: calculate_average 호출
    println!("평균 점수: {:.2}", calculate_average(&scores));

    // TODO: find_highest 호출
    if let Some(highest) = find_highest(&scores) {
        println!("최고 점수: {}", highest);
    }

    // TODO: find_lowest 호출
    if let Some(lowest) = find_lowest(&scores) {
        println!("최저 점수: {}", lowest);
    }

    // TODO: filter_passing 호출 (80점 이상)
    let passing = filter_passing(&scores, 80);
    println!("합격 점수(80 이상): {:?}", passing);
}
