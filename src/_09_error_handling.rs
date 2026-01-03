// ============================================================================
// 09. 에러 처리 (Error Handling)
// ============================================================================
// C++20과의 핵심 차이점:
// 1. 예외 없음 - Result<T, E>로 에러 반환
// 2. ? 연산자로 에러 전파 간소화
// 3. panic!은 복구 불가능한 에러용 (C++ abort와 유사)
// 4. Option<T>으로 null 대체
// 5. std::expected (C++23)과 유사하지만 더 통합됨
// ============================================================================

use std::fs::File;
use std::io::{self, Read};

pub fn run() {
    println!("\n=== 09. 에러 처리 ===\n");

    panic_demo();
    result_basics();
    result_methods();
    question_mark_operator();
    custom_errors();
    option_result_conversion();
}

// ----------------------------------------------------------------------------
// panic! - 복구 불가능한 에러
// ----------------------------------------------------------------------------

fn panic_demo() {
    println!("--- panic! ---");

    // panic!은 프로그램을 즉시 종료
    // C++의 abort() 또는 throw 후 catch 없음과 유사

    // panic!("크래시!");  // 프로그램 종료

    // 배열 범위 초과도 panic
    let v = vec![1, 2, 3];
    // let x = v[99];  // panic: index out of bounds

    // panic vs Result:
    // - panic: 프로그래밍 에러 (버그), 복구 불가능
    // - Result: 예상 가능한 에러, 복구 가능

    // RUST_BACKTRACE=1로 실행하면 스택 트레이스 확인 가능

    println!("panic 없이 계속 실행");
}

// ----------------------------------------------------------------------------
// Result 기초
// ----------------------------------------------------------------------------

fn result_basics() {
    println!("\n--- Result 기초 ---");

    // Result 정의:
    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }

    // C++: std::expected<T, E> (C++23) 또는 반환값 + 에러 코드

    // 파일 열기 예제
    let result = File::open("hello.txt");

    // match로 처리
    match result {
        Ok(file) => println!("파일 열기 성공: {:?}", file),
        Err(error) => println!("파일 열기 실패: {}", error),
    }

    // C++ 대비 장점:
    // 1. 에러를 무시할 수 없음 (Result를 사용하지 않으면 경고)
    // 2. 어떤 에러가 발생할 수 있는지 타입에서 명확
    // 3. 컴파일 타임에 에러 처리 강제

    // 에러 종류에 따른 처리
    let result = File::open("hello.txt");

    match result {
        Ok(file) => println!("파일: {:?}", file),
        Err(ref error) if error.kind() == io::ErrorKind::NotFound => {
            println!("파일을 찾을 수 없음, 생성 시도...");
            // File::create("hello.txt") 등
        }
        Err(error) => println!("기타 에러: {}", error),
    }
}

// ----------------------------------------------------------------------------
// Result 메서드
// ----------------------------------------------------------------------------

fn result_methods() {
    println!("\n--- Result 메서드 ---");

    // unwrap: Ok면 값, Err면 panic
    // 프로토타입이나 확실히 성공하는 경우에만 사용
    let ok_result: Result<i32, &str> = Ok(42);
    println!("unwrap: {}", ok_result.unwrap());

    // expect: unwrap + 커스텀 에러 메시지
    let ok_result: Result<i32, &str> = Ok(42);
    println!("expect: {}", ok_result.expect("값이 있어야 함"));

    // unwrap_or: Err일 때 기본값
    let err_result: Result<i32, &str> = Err("에러");
    println!("unwrap_or: {}", err_result.unwrap_or(0));

    // unwrap_or_else: Err일 때 클로저 실행
    let err_result: Result<i32, &str> = Err("에러");
    let value = err_result.unwrap_or_else(|e| {
        println!("에러 발생: {}", e);
        -1
    });
    println!("unwrap_or_else: {}", value);

    // map: Ok 내부 값 변환
    let ok_result: Result<i32, &str> = Ok(5);
    let doubled = ok_result.map(|n| n * 2);
    println!("map: {:?}", doubled);

    // map_err: Err 변환
    let err_result: Result<i32, &str> = Err("문자열 에러");
    let mapped: Result<i32, String> = err_result.map_err(|e| format!("변환됨: {}", e));
    println!("map_err: {:?}", mapped);

    // and_then: 체이닝 (flatMap)
    fn square(x: i32) -> Result<i32, &'static str> {
        Ok(x * x)
    }

    let result: Result<i32, &str> = Ok(2);
    let chained = result.and_then(square).and_then(square);
    println!("and_then: {:?}", chained); // Ok(16)

    // or_else: Err일 때 다른 Result 시도
    fn fallback() -> Result<i32, &'static str> {
        Ok(0)
    }

    let err_result: Result<i32, &str> = Err("에러");
    let recovered = err_result.or_else(|_| fallback());
    println!("or_else: {:?}", recovered);

    // ok: Result -> Option (에러 무시)
    let result: Result<i32, &str> = Ok(42);
    let option = result.ok();
    println!("ok: {:?}", option);

    // is_ok, is_err
    let result: Result<i32, &str> = Ok(42);
    println!("is_ok: {}, is_err: {}", result.is_ok(), result.is_err());
}

// ----------------------------------------------------------------------------
// ? 연산자
// ----------------------------------------------------------------------------

fn question_mark_operator() {
    println!("\n--- ? 연산자 ---");

    // ? 연산자: Ok면 값 추출, Err면 조기 반환
    // C++에는 직접적인 대응이 없음 (매크로나 예외로 구현)

    // ? 없이 작성한 코드
    fn read_username_long() -> Result<String, io::Error> {
        let file_result = File::open("hello.txt");

        let mut file = match file_result {
            Ok(f) => f,
            Err(e) => return Err(e),
        };

        let mut username = String::new();

        match file.read_to_string(&mut username) {
            Ok(_) => Ok(username),
            Err(e) => Err(e),
        }
    }

    // ? 사용 - 훨씬 간결
    fn read_username_short() -> Result<String, io::Error> {
        let mut file = File::open("hello.txt")?;
        let mut username = String::new();
        file.read_to_string(&mut username)?;
        Ok(username)
    }

    // 더 짧게 - 체이닝
    fn read_username_chained() -> Result<String, io::Error> {
        let mut username = String::new();
        File::open("hello.txt")?.read_to_string(&mut username)?;
        Ok(username)
    }

    // 가장 짧게 - 표준 라이브러리 함수
    fn read_username_fs() -> Result<String, io::Error> {
        std::fs::read_to_string("hello.txt")
    }

    // 결과 확인
    match read_username_short() {
        Ok(name) => println!("사용자명: {}", name),
        Err(e) => println!("읽기 실패: {}", e),
    }

    // ?는 From 트레이트로 에러 변환도 수행
    // io::Error -> CustomError 자동 변환 가능
}

// ----------------------------------------------------------------------------
// 커스텀 에러
// ----------------------------------------------------------------------------

fn custom_errors() {
    println!("\n--- 커스텀 에러 ---");

    // 간단한 에러 열거형
    #[derive(Debug)]
    enum ParseError {
        Empty,
        InvalidFormat,
        OutOfRange(i32),
    }

    // std::error::Error 트레이트 구현
    impl std::fmt::Display for ParseError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                ParseError::Empty => write!(f, "입력이 비어있음"),
                ParseError::InvalidFormat => write!(f, "잘못된 형식"),
                ParseError::OutOfRange(n) => write!(f, "범위 초과: {}", n),
            }
        }
    }

    impl std::error::Error for ParseError {}

    fn parse_positive(s: &str) -> Result<i32, ParseError> {
        if s.is_empty() {
            return Err(ParseError::Empty);
        }

        let n: i32 = s.parse().map_err(|_| ParseError::InvalidFormat)?;

        if n <= 0 {
            return Err(ParseError::OutOfRange(n));
        }

        Ok(n)
    }

    // 테스트
    for input in &["42", "", "abc", "-5"] {
        match parse_positive(input) {
            Ok(n) => println!("'{}' -> {}", input, n),
            Err(e) => println!("'{}' -> 에러: {}", input, e),
        }
    }

    // 에러 래핑 - 원인 에러 보존
    #[derive(Debug)]
    struct ReadConfigError {
        filename: String,
        source: io::Error,
    }

    impl std::fmt::Display for ReadConfigError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "설정 파일 '{}' 읽기 실패", self.filename)
        }
    }

    impl std::error::Error for ReadConfigError {
        fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
            Some(&self.source)
        }
    }

    // 실무에서는 thiserror, anyhow 크레이트 사용 권장
    // #[derive(thiserror::Error, Debug)]
    // enum Error {
    //     #[error("IO 에러: {0}")]
    //     Io(#[from] io::Error),
    //     #[error("파싱 에러")]
    //     Parse,
    // }
}

// ----------------------------------------------------------------------------
// Option과 Result 변환
// ----------------------------------------------------------------------------

fn option_result_conversion() {
    println!("\n--- Option과 Result 변환 ---");

    // Option -> Result
    let opt: Option<i32> = Some(42);
    let result: Result<i32, &str> = opt.ok_or("값 없음");
    println!("ok_or: {:?}", result);

    let none: Option<i32> = None;
    let result: Result<i32, &str> = none.ok_or("값 없음");
    println!("ok_or (None): {:?}", result);

    // Result -> Option
    let result: Result<i32, &str> = Ok(42);
    let opt: Option<i32> = result.ok();
    println!("ok: {:?}", opt);

    let err: Result<i32, &str> = Err("에러");
    let opt: Option<i32> = err.ok();
    println!("ok (Err): {:?}", opt);

    // transpose: Option<Result<T, E>> <-> Result<Option<T>, E>
    let opt_result: Option<Result<i32, &str>> = Some(Ok(42));
    let result_opt: Result<Option<i32>, &str> = opt_result.transpose();
    println!("transpose: {:?}", result_opt);

    // collect로 Result<Vec<T>, E> 만들기
    let strings = vec!["1", "2", "3"];
    let numbers: Result<Vec<i32>, _> = strings.iter().map(|s| s.parse()).collect();
    println!("collect Ok: {:?}", numbers);

    let mixed = vec!["1", "two", "3"];
    let numbers: Result<Vec<i32>, _> = mixed.iter().map(|s| s.parse::<i32>()).collect();
    println!("collect Err: {:?}", numbers);
}
