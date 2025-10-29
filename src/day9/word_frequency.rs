// ========================================
// 과제 2: 단어 빈도수 계산 (20분)
// ========================================
// HashMap을 사용하여 텍스트에서 단어의 출현 빈도를 계산하세요.
//
// 요구사항:
// - count_words(text: &str) -> HashMap<String, usize>
//   * 텍스트를 공백으로 분리하여 각 단어의 출현 횟수를 카운트
//   * 대소문자 구분 없이 처리 (모두 소문자로 변환)
//   * 힌트: to_lowercase() 메서드 사용
//
// - find_most_frequent(word_counts: &HashMap<String, usize>) -> Option<(String, usize)>
//   * 가장 많이 나온 단어와 그 횟수를 튜플로 반환
//   * 빈 HashMap이면 None 반환
//   * 힌트: for 루프로 순회하면서 최댓값 추적

use std::collections::HashMap;

// TODO: count_words 함수를 구현하세요
fn count_words(_text: &str) -> HashMap<String, usize> {
    let mut word_map: HashMap<String, usize> = HashMap::new();

    for word in _text.split_whitespace() {
        let count = word_map.entry(word.to_lowercase()).or_insert(0);
        *count += 1;
    }

    word_map
}

// TODO: find_most_frequent 함수를 구현하세요
fn find_most_frequent(_word_counts: &HashMap<String, usize>) -> Option<(String, usize)> {
    if _word_counts.is_empty() {
        return None;
    }

    let mut return_tuple: (String, usize) = (String::new(), 0);
    for (word, count) in _word_counts {
        if return_tuple.1 < *count {
            return_tuple = (word.clone(), *count);
        }
    }
    Some(return_tuple)
}

pub fn run() {
    println!("=== 과제 2: 단어 빈도수 계산 ===");
    let text = "Hello world hello Rust world hello";

    // TODO: count_words 호출
    let word_counts = count_words(text);
    println!("단어 빈도수: {:?}", word_counts);

    // TODO: find_most_frequent 호출
    if let Some((word, count)) = find_most_frequent(&word_counts) {
        println!("가장 많이 나온 단어: '{}' ({}번)", word, count);
    }
}
