// 과제 2: 피보나치 수열
// n번째 피보나치 수를 계산하는 함수
// 피보나치 수열: 0, 1, 1, 2, 3, 5, 8, 13...
pub fn fibonacci(n: u32) -> u32 {
    // TODO: 반복문을 사용해서 구현하세요
    // 힌트: 이전 두 수를 저장할 변수가 필요합니다
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }

    let mut a = 0;
    let mut b = 1;
    for _ in 1..n {
        let c = a + b;
        a = b;
        b = c;
    }

    b
}
