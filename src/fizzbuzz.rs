// 과제 3: FizzBuzz 변형
pub fn fizzbuzz(n: i32) {
    // TODO: 1부터 100까지 반복하면서
    // 3의 배수이면 "Fizz"
    // 5의 배수이면 "Buzz"
    // 3과 5의 공배수이면 "FizzBuzz"
    // 그 외에는 숫자를 출력하세요
    // 힌트: 나머지 연산자 % 를 사용하세요
    for i in 1..=n {
        if i % 3 == 0 && i % 5 == 0 {
            println!("FizzBuzz");
        } else if i % 5 == 0 {
            println!("Buzz");
        } else if i % 3 == 0 {
            println!("Fizz");
        } else {
            println!("{}", i);
        }
    }
}
