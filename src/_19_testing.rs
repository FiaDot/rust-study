// ============================================================================
// 19. 테스트 (Testing)
// ============================================================================
// C++20과의 핵심 차이점:
// 1. 테스트가 언어/빌드 시스템에 내장 - 별도 프레임워크 불필요
// 2. #[test] 어트리뷰트로 테스트 함수 표시
// 3. cargo test로 모든 테스트 실행
// 4. 단위 테스트는 같은 파일에, 통합 테스트는 tests/ 디렉터리에
// 5. 문서 테스트 (doc tests) 지원
// ============================================================================

pub fn run() {
    println!("\n=== 19. 테스트 ===\n");

    test_basics_explanation();
    assertion_macros_explanation();
    test_organization_explanation();
    test_attributes_explanation();
    test_commands_explanation();
}

// ============================================================================
// 테스트 기본 구조
// ============================================================================

fn test_basics_explanation() {
    println!("--- 테스트 기본 구조 ---");

    println!(r#"
// 테스트 함수 정의
#[test]
fn it_works() {{
    assert_eq!(2 + 2, 4);
}}

// 테스트 모듈 (관례적 구조)
#[cfg(test)]
mod tests {{
    use super::*;  // 부모 모듈의 항목 가져오기

    #[test]
    fn test_addition() {{
        assert_eq!(add(2, 3), 5);
    }}

    #[test]
    fn test_subtraction() {{
        assert_eq!(subtract(5, 3), 2);
    }}
}}
"#);

    println!("실행 방법:");
    println!("  cargo test              # 모든 테스트 실행");
    println!("  cargo test test_name    # 특정 테스트만 실행");
    println!("  cargo test --lib        # 라이브러리 테스트만");
    println!("  cargo test --doc        # 문서 테스트만");
}

// ============================================================================
// 단언 매크로 (Assertion Macros)
// ============================================================================

fn assertion_macros_explanation() {
    println!("\n--- 단언 매크로 ---");

    println!(r#"
// 기본 단언
assert!(condition);              // condition이 true인지 확인
assert!(value > 0, "값이 양수여야 함: {{}}", value);  // 커스텀 메시지

// 동등성 비교
assert_eq!(left, right);         // left == right
assert_ne!(left, right);         // left != right

// 실패 예상 테스트
#[test]
#[should_panic]
fn test_panic() {{
    panic!("의도된 패닉");
}}

// 특정 메시지로 패닉 확인
#[test]
#[should_panic(expected = "out of bounds")]
fn test_specific_panic() {{
    let v = vec![1, 2, 3];
    v[99];  // 패닉 발생
}}

// Result를 반환하는 테스트
#[test]
fn test_with_result() -> Result<(), String> {{
    if 2 + 2 == 4 {{
        Ok(())
    }} else {{
        Err(String::from("수학이 이상해요"))
    }}
}}
"#);

    // 실제 동작 예시
    println!("실제 단언 동작:");

    // assert!
    let value = 10;
    assert!(value > 0);
    println!("  assert!(10 > 0) - 통과");

    // assert_eq!
    assert_eq!(2 + 2, 4);
    println!("  assert_eq!(2 + 2, 4) - 통과");

    // assert_ne!
    assert_ne!("hello", "world");
    println!("  assert_ne!(\"hello\", \"world\") - 통과");
}

// ============================================================================
// 테스트 구성
// ============================================================================

fn test_organization_explanation() {
    println!("\n--- 테스트 구성 ---");

    println!(r#"
프로젝트 구조:
my_project/
├── Cargo.toml
├── src/
│   ├── lib.rs          # 라이브러리 루트
│   ├── main.rs         # 바이너리 (있는 경우)
│   └── utils.rs        # 모듈
└── tests/              # 통합 테스트
    ├── integration_test.rs
    └── common/
        └── mod.rs      # 테스트 헬퍼

=== 1. 단위 테스트 (Unit Tests) ===
- 같은 파일 내 #[cfg(test)] 모듈에 작성
- private 함수도 테스트 가능
- cargo test --lib로 실행

// src/lib.rs
pub fn public_fn() -> i32 {{ 42 }}
fn private_fn() -> i32 {{ 100 }}

#[cfg(test)]
mod tests {{
    use super::*;

    #[test]
    fn test_public() {{
        assert_eq!(public_fn(), 42);
    }}

    #[test]
    fn test_private() {{
        // private 함수도 테스트 가능!
        assert_eq!(private_fn(), 100);
    }}
}}

=== 2. 통합 테스트 (Integration Tests) ===
- tests/ 디렉터리에 별도 파일로 작성
- public API만 테스트 가능
- 각 파일이 별도 크레이트로 컴파일

// tests/integration_test.rs
use my_project::public_fn;

#[test]
fn test_from_outside() {{
    assert_eq!(public_fn(), 42);
}}

=== 3. 문서 테스트 (Doc Tests) ===
- 문서 주석 내 코드 블록 자동 테스트
- 문서와 코드 동기화 보장

/// 두 수를 더합니다.
///
/// # Examples
///
/// ```
/// let result = my_crate::add(2, 3);
/// assert_eq!(result, 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {{
    a + b
}}
"#);
}

// ============================================================================
// 테스트 어트리뷰트
// ============================================================================

fn test_attributes_explanation() {
    println!("\n--- 테스트 어트리뷰트 ---");

    println!(r#"
// 기본 테스트
#[test]
fn basic_test() {{ }}

// 무시할 테스트
#[test]
#[ignore]
fn slow_test() {{
    // 시간이 오래 걸리는 테스트
}}

// 무시된 테스트 실행: cargo test -- --ignored
// 모든 테스트 실행: cargo test -- --include-ignored

// 패닉 예상
#[test]
#[should_panic]
fn panics() {{ panic!(); }}

// 특정 패닉 메시지 확인
#[test]
#[should_panic(expected = "divide by zero")]
fn panics_with_message() {{
    divide(1, 0);
}}

// 조건부 컴파일
#[test]
#[cfg(target_os = "linux")]
fn linux_only_test() {{ }}

// 타임아웃 (nightly 기능)
// #[test]
// #[timeout(1000)]  // 1초
// fn must_finish_quickly() {{ }}
"#);
}

// ============================================================================
// cargo test 명령어
// ============================================================================

fn test_commands_explanation() {
    println!("\n--- cargo test 명령어 ---");

    println!(r#"
=== 기본 명령어 ===
cargo test                    # 모든 테스트 실행
cargo test --release          # 릴리즈 모드로 테스트
cargo test --no-fail-fast     # 실패해도 계속 실행

=== 필터링 ===
cargo test test_name          # 이름에 'test_name' 포함된 테스트
cargo test tests::            # 'tests::' 모듈의 테스트
cargo test --test integration # tests/integration.rs만 실행

=== 테스트 종류 선택 ===
cargo test --lib              # 단위 테스트만
cargo test --doc              # 문서 테스트만
cargo test --bins             # 바이너리 테스트만
cargo test --examples         # 예제 테스트만

=== 출력 제어 ===
cargo test -- --nocapture     # println! 출력 보기
cargo test -- --show-output   # 성공한 테스트 출력도 보기
cargo test -- --test-threads=1  # 단일 스레드로 실행

=== 무시된 테스트 ===
cargo test -- --ignored       # #[ignore] 테스트만 실행
cargo test -- --include-ignored  # 모든 테스트 (무시된 것 포함)

=== 테스트 목록 ===
cargo test -- --list          # 테스트 목록만 출력
cargo test -- --list --ignored  # 무시된 테스트 목록
"#);

    println!("=== 예시 출력 ===");
    println!(r#"
$ cargo test

running 3 tests
test tests::test_addition ... ok
test tests::test_subtraction ... ok
test tests::test_multiplication ... ok

test result: ok. 3 passed; 0 failed; 0 ignored

   Doc-tests my_project

running 2 tests
test src/lib.rs - add (line 5) ... ok
test src/lib.rs - subtract (line 15) ... ok

test result: ok. 2 passed; 0 failed; 0 ignored
"#);
}

// ============================================================================
// 실제 테스트 예제 (이 파일 내에서)
// ============================================================================

// 테스트할 함수들
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

pub fn divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("divide by zero");
    }
    a / b
}

pub fn is_even(n: i32) -> bool {
    n % 2 == 0
}

// 테스트 모듈
#[cfg(test)]
mod tests {
    use super::*;

    // 기본 테스트
    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
        assert_eq!(add(-1, 1), 0);
        assert_eq!(add(0, 0), 0);
    }

    #[test]
    fn test_subtract() {
        assert_eq!(subtract(5, 3), 2);
        assert_eq!(subtract(3, 5), -2);
    }

    #[test]
    fn test_divide() {
        assert_eq!(divide(10, 2), 5);
        assert_eq!(divide(7, 2), 3);  // 정수 나눗셈
    }

    // 패닉 테스트
    #[test]
    #[should_panic(expected = "divide by zero")]
    fn test_divide_by_zero() {
        divide(1, 0);
    }

    // 여러 케이스 테스트
    #[test]
    fn test_is_even() {
        let test_cases = [
            (0, true),
            (1, false),
            (2, true),
            (-2, true),
            (-3, false),
        ];

        for (input, expected) in test_cases {
            assert_eq!(
                is_even(input),
                expected,
                "is_even({}) should be {}",
                input,
                expected
            );
        }
    }

    // Result 반환 테스트
    #[test]
    fn test_with_result() -> Result<(), String> {
        let result = add(2, 2);
        if result == 4 {
            Ok(())
        } else {
            Err(format!("Expected 4, got {}", result))
        }
    }

    // 무시되는 테스트 (cargo test -- --ignored로 실행)
    #[test]
    #[ignore]
    fn slow_test() {
        std::thread::sleep(std::time::Duration::from_secs(1));
        assert!(true);
    }

    // 셋업이 필요한 테스트
    #[test]
    fn test_with_setup() {
        // Arrange (준비)
        let data = vec![1, 2, 3, 4, 5];
        let expected_sum = 15;

        // Act (실행)
        let actual_sum: i32 = data.iter().sum();

        // Assert (검증)
        assert_eq!(actual_sum, expected_sum);
    }
}

// ============================================================================
// 테스트 헬퍼 및 픽스처
// ============================================================================

#[cfg(test)]
mod test_helpers {
    // 테스트용 구조체
    #[derive(Debug, PartialEq)]
    pub struct TestUser {
        pub name: String,
        pub age: u32,
    }

    impl TestUser {
        // 테스트 픽스처 생성
        pub fn sample() -> Self {
            TestUser {
                name: String::from("Test User"),
                age: 25,
            }
        }

        pub fn with_name(name: &str) -> Self {
            TestUser {
                name: String::from(name),
                age: 25,
            }
        }
    }

    // 테스트 헬퍼 함수
    pub fn assert_in_range<T: PartialOrd + std::fmt::Debug>(
        value: T,
        min: T,
        max: T,
    ) {
        assert!(
            value >= min && value <= max,
            "{:?} is not in range [{:?}, {:?}]",
            value,
            min,
            max
        );
    }
}

#[cfg(test)]
mod advanced_tests {
    use super::test_helpers::*;

    #[test]
    fn test_with_fixture() {
        let user = TestUser::sample();
        assert_eq!(user.name, "Test User");
        assert_eq!(user.age, 25);
    }

    #[test]
    fn test_custom_assertion() {
        let value = 50;
        assert_in_range(value, 0, 100);
    }
}
