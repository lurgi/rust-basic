// ========================================
// 통합 과제 2: 제네릭 캐시 시스템 (45분)
// ========================================
// 자주 사용되는 값을 캐싱하는 시스템을 만드세요.
//
// 요구사항:
//
// 1. CacheError 열거형
//    - NotFound(String): 키를 찾을 수 없음
//    - Expired(String): 캐시 만료됨
//
// 2. Cacheable 트레잇
//    - is_valid(&self) -> bool
//      * 캐시 값이 유효한지 확인
//
// 3. CachedValue<T> 구조체
//    - value: T
//    - created_at: u64 (생성 시간, 초 단위)
//    - ttl: u64 (Time To Live, 초 단위)
//    - Cacheable 트레잇 구현
//      * 현재 시간을 0으로 가정하고, created_at + ttl > 현재시간 이면 유효
//      * 단순화를 위해 get_current_time() 함수 사용
//
// 4. Cache<T: Clone> 구조체
//    - data: HashMap<String, CachedValue<T>>
//
// 5. impl<T: Clone> Cache<T>
//    - new() -> Cache<T>
//
//    - set(&mut self, key: String, value: T, ttl: u64)
//      * 값을 캐시에 저장
//      * CachedValue를 만들어서 저장
//      * created_at는 get_current_time() 사용
//
//    - get(&self, key: &str) -> Result<T, CacheError>
//      * 캐시에서 값 가져오기
//      * 키가 없으면 NotFound 에러
//      * 값이 만료되었으면 Expired 에러
//      * 유효하면 Ok(value.clone())
//
//    - remove(&mut self, key: &str) -> Result<T, CacheError>
//      * 캐시에서 값 제거
//      * 키가 없으면 NotFound 에러
//
//    - clear(&mut self)
//      * 모든 캐시 삭제
//
//    - size(&self) -> usize
//      * 캐시된 항목 개수

use std::collections::HashMap;

// 현재 시간을 시뮬레이션하는 전역 변수
static mut CURRENT_TIME: u64 = 0;

fn get_current_time() -> u64 {
    unsafe { CURRENT_TIME }
}

fn advance_time(seconds: u64) {
    unsafe {
        CURRENT_TIME += seconds;
    }
}

// TODO: CacheError 열거형을 정의하세요
#[derive(Debug)]
enum CacheError {
    NotFound(String),
    Expired(String),
}

// TODO: Cacheable 트레잇을 정의하세요
trait Cacheable {
    fn is_valid(&self) -> bool;
}

// TODO: CachedValue 구조체를 정의하세요
struct CachedValue<T> {
    value: T,
    created_at: u64,
    ttl: u64,
}

// TODO: CachedValue에 Cacheable 트레잇을 구현하세요
impl<T> Cacheable for CachedValue<T> {
    fn is_valid(&self) -> bool {
        self.created_at + self.ttl > get_current_time()
    }
}

// TODO: Cache 구조체를 정의하세요
struct Cache<T> {
    data: HashMap<String, CachedValue<T>>,
}

// TODO: Cache의 모든 메서드를 구현하세요

impl<T: Clone> Cache<T> {
    fn new() -> Cache<T> {
        Cache {
            data: HashMap::new(),
        }
    }

    fn set(&mut self, key: String, value: T, ttl: u64) {
        self.data.insert(
            key,
            CachedValue {
                value,
                created_at: get_current_time(),
                ttl,
            },
        );
    }

    fn get(&self, key: &str) -> Result<T, CacheError> {
        match self.data.get(key) {
            Some(cached_value) => {
                if cached_value.is_valid() {
                    Ok(cached_value.value.clone())
                } else {
                    Err(CacheError::Expired(key.to_string()))
                }
            }
            None => Err(CacheError::NotFound(key.to_string())),
        }
    }

    fn remove(&mut self, key: &str) -> Result<T, CacheError> {
        match self.data.remove(key) {
            Some(cached_value) => {
                if cached_value.is_valid() {
                    Ok(cached_value.value.clone())
                } else {
                    Err(CacheError::Expired(key.to_string()))
                }
            }
            None => Err(CacheError::NotFound(String::from(key))),
        }
    }

    fn clear(&mut self) {
        self.data.clear();
    }

    fn size(&self) -> usize {
        self.data.len()
    }
}

pub fn run() {
    println!("=== 통합 과제 2: 제네릭 캐시 시스템 ===\n");

    let mut cache: Cache<String> = Cache::new();

    // 캐시에 값 저장 (TTL: 10초)
    cache.set(String::from("user_1"), String::from("Alice"), 10);

    cache.set(String::from("user_2"), String::from("Bob"), 5);

    println!("캐시 크기: {}", cache.size());

    // 값 가져오기
    match cache.get("user_1") {
        Ok(value) => println!("user_1: {}", value),
        Err(e) => println!("에러: {:?}", e),
    }

    // 5초 경과
    advance_time(5);
    println!("\n5초 경과...");

    // user_2는 만료됨 (TTL: 5초)
    match cache.get("user_2") {
        Ok(value) => println!("user_2: {}", value),
        Err(e) => println!("에러: {:?}", e),
    }

    // user_1은 여전히 유효 (TTL: 10초, 경과: 5초)
    match cache.get("user_1") {
        Ok(value) => println!("user_1: {}", value),
        Err(e) => println!("에러: {:?}", e),
    }

    // 6초 더 경과 (총 11초)
    advance_time(6);
    println!("\n6초 더 경과... (총 11초)");

    // user_1도 만료됨 (TTL: 10초, 경과: 11초)
    match cache.get("user_1") {
        Ok(value) => println!("user_1: {}", value),
        Err(e) => println!("에러: {:?}", e),
    }

    // 새 값 추가
    cache.set(String::from("user_3"), String::from("Charlie"), 20);

    println!("\n캐시 크기: {}", cache.size());

    // 캐시 비우기
    cache.clear();
    println!("캐시 비움");
    println!("캐시 크기: {}", cache.size());
}
