// ========================================
// 과제 2: BankAccount 구조체 (20분)
// ========================================
// 은행 계좌를 나타내는 구조체를 만들고 입출금 기능을 구현하세요.
//
// 요구사항:
// - BankAccount 구조체 정의
//   - owner: String (계좌 소유자)
//   - balance: f64 (잔액)
//
// - 다음 메서드들을 impl 블록에 구현:
//   1. new (연관 함수): owner를 받아 BankAccount 생성
//      - balance는 0.0으로 초기화
//
//   2. deposit (&mut self, amount: f64): 입금
//      - balance에 amount를 더함
//      - amount가 0보다 크지 않으면 아무것도 하지 않음
//
//   3. withdraw (&mut self, amount: f64) -> bool: 출금
//      - balance가 amount보다 크거나 같으면 출금하고 true 반환
//      - 잔액이 부족하면 false 반환
//      - amount가 0보다 크지 않으면 false 반환
//
//   4. get_balance (&self) -> f64: 현재 잔액 반환

// TODO: BankAccount 구조체를 정의하세요
#[derive(Debug)]
struct BankAccount {
    // TODO: 필드들을 정의하세요
    owner: String,
    balance: f64,
}

// TODO: impl 블록에 메서드들을 구현하세요
impl BankAccount {
    // TODO: new 연관 함수
    fn new(owner: String) -> BankAccount {
        BankAccount {
            owner,
            balance: 0.0,
        }
    }
    // TODO: deposit 메서드
    fn deposit(&mut self, amount: f64) {
        if amount > 0.0 {
            self.balance += amount;
        }
    }

    // TODO: withdraw 메서드
    fn withdraw(&mut self, amount: f64) -> bool {
        if amount <= 0.0 {
            return false;
        }
        if self.balance >= amount {
            self.balance -= amount;
            return true;
        }
        false
    }

    // TODO: get_balance 메서드
    fn get_balance(&self) -> f64 {
        self.balance
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_account_creation() {
        // TODO: BankAccount::new로 계좌를 생성하고 테스트하세요
        let account = BankAccount::new(String::from("John Doe"));
        assert_eq!(account.owner, "John Doe");
        assert_eq!(account.balance, 0.0);
    }

    #[test]
    fn test_deposit() {
        // TODO: deposit 메서드를 테스트하세요
        let mut account = BankAccount::new(String::from("John Doe"));
        account.deposit(100.0);
        assert_eq!(account.balance, 100.0);
    }

    #[test]
    fn test_withdraw() {
        // TODO: withdraw 메서드를 테스트하세요
        let mut account = BankAccount::new(String::from("John Doe"));
        account.deposit(100.0);
        assert_eq!(account.withdraw(50.0), true);
        assert_eq!(account.balance, 50.0);
    }

    #[test]
    fn test_insufficient_balance() {
        // TODO: 잔액 부족 시 출금 실패를 테스트하세요
        let mut account = BankAccount::new(String::from("John Doe"));
        account.deposit(100.0);
        assert_eq!(account.withdraw(150.0), false);
        assert_eq!(account.balance, 100.0);
    }
}

pub fn run() {
    println!("\n=== 과제 2: BankAccount 구조체 ===");
    // TODO: BankAccount 테스트
    // 1. BankAccount::new로 계좌 생성
    let mut account = BankAccount::new(String::from("John Doe"));
    // 2. 초기 잔액 출력
    println!("초기 잔액: {}", account.balance);
    // 3. 1000.0 입금
    account.deposit(1000.0);
    println!("잔액: {}", account.balance);
    // 4. 잔액 출력
    println!("잔액: {}", account.balance);
    // 5. 500.0 출금 시도
    account.withdraw(500.0);
    println!("잔액: {}", account.balance);
    // 6. 잔액 출력
    println!("잔액: {}", account.balance);
    // 7. 1000.0 출금 시도 (잔액 부족)
    account.withdraw(1000.0);
    // 8. 최종 잔액 출력
    println!("최종 잔액: {}", account.balance);
}
