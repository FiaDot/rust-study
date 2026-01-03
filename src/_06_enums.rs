// ============================================================================
// 06. 열거형 (Enums)과 패턴 매칭
// ============================================================================
// C++20과의 핵심 차이점:
// 1. Rust enum은 데이터를 가질 수 있음 - C++ std::variant와 유사하지만 더 강력
// 2. 패턴 매칭(match)은 모든 케이스를 처리해야 함 (exhaustive)
// 3. Option<T>로 null 없이 값의 부재 표현
// 4. if let, while let으로 단일 패턴 간편하게 처리
// ============================================================================

pub fn run() {
    println!("\n=== 06. 열거형과 패턴 매칭 ===\n");

    basic_enum();
    enum_with_data();
    option_type();
    match_expression();
    if_let_while_let();
    pattern_matching_advanced();
}

// ----------------------------------------------------------------------------
// 기본 열거형
// ----------------------------------------------------------------------------

// C++:
// enum class Direction { North, South, East, West };

// Rust: (기본적으로 C++의 enum class처럼 스코프됨)
#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn basic_enum() {
    println!("--- 기본 열거형 ---");

    let dir = Direction::North;
    println!("방향: {:?}", dir);

    // C++ enum class처럼 타입 안전
    // let x: i32 = dir;  // 에러! 암묵적 변환 없음

    // 정수 값 할당
    #[derive(Debug)]
    #[repr(u16)]  // 기본 타입 지정 (C++의 enum class : uint16_t)
    enum HttpStatus {
        Ok = 200,
        NotFound = 404,
        InternalError = 500,
    }

    let status = HttpStatus::Ok;
    println!("상태 코드: {}", status as u16);
}

// ----------------------------------------------------------------------------
// 데이터를 가진 열거형
// ----------------------------------------------------------------------------

// C++에서 비슷한 것: std::variant + struct
// 각 variant가 서로 다른 타입과 개수의 데이터를 가질 수 있음

#[derive(Debug)]
enum Message {
    Quit,                        // 데이터 없음
    Move { x: i32, y: i32 },     // 익명 구조체
    Write(String),               // String 하나
    ChangeColor(i32, i32, i32),  // 튜플
}

// C++로 유사하게 구현:
// struct Quit {};
// struct Move { int x, y; };
// struct Write { std::string text; };
// struct ChangeColor { int r, g, b; };
// using Message = std::variant<Quit, Move, Write, ChangeColor>;

fn enum_with_data() {
    println!("\n--- 데이터를 가진 열거형 ---");

    let msg1 = Message::Quit;
    let msg2 = Message::Move { x: 10, y: 20 };
    let msg3 = Message::Write(String::from("hello"));
    let msg4 = Message::ChangeColor(255, 128, 0);

    println!("메시지들: {:?}, {:?}, {:?}, {:?}", msg1, msg2, msg3, msg4);

    // 열거형에도 메서드 구현 가능
    msg3.call();
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("종료"),
            Message::Move { x, y } => println!("이동: ({}, {})", x, y),
            Message::Write(text) => println!("작성: {}", text),
            Message::ChangeColor(r, g, b) => println!("색상: RGB({}, {}, {})", r, g, b),
        }
    }
}

// ----------------------------------------------------------------------------
// Option 타입 - null을 대체
// ----------------------------------------------------------------------------

fn option_type() {
    println!("\n--- Option 타입 ---");

    // Rust에는 null이 없음!
    // 대신 Option<T> 사용

    // 표준 라이브러리 정의:
    // enum Option<T> {
    //     None,
    //     Some(T),
    // }

    // C++: std::optional<int>
    let some_number: Option<i32> = Some(5);
    let no_number: Option<i32> = None;

    println!("some_number: {:?}", some_number);
    println!("no_number: {:?}", no_number);

    // Option<T>와 T는 다른 타입!
    // let sum = some_number + 5;  // 에러! Option<i32> + i32 불가

    // 값을 사용하려면 Option을 처리해야 함
    match some_number {
        Some(n) => println!("값: {}", n),
        None => println!("값 없음"),
    }

    // C++에서 흔한 null 버그:
    // int* ptr = nullptr;
    // *ptr = 5;  // 런타임 크래시!

    // Rust에서는 불가능:
    // 1. Option을 처리하지 않으면 컴파일 에러
    // 2. unwrap()으로 강제 추출하면 None일 때 panic

    // 유용한 Option 메서드들
    let x = Some(5);

    // unwrap: Some이면 값, None이면 panic
    println!("unwrap: {}", x.unwrap());

    // unwrap_or: None일 때 기본값
    let y: Option<i32> = None;
    println!("unwrap_or: {}", y.unwrap_or(0));

    // expect: unwrap + 커스텀 에러 메시지
    println!("expect: {}", x.expect("값이 있어야 함"));

    // is_some, is_none
    println!("is_some: {}, is_none: {}", x.is_some(), y.is_none());

    // map: Some 내부 값 변환
    let doubled = x.map(|n| n * 2);
    println!("map: {:?}", doubled);

    // and_then: flatMap (중첩 Option 방지)
    let result = x.and_then(|n| Some(n + 1));
    println!("and_then: {:?}", result);
}

// ----------------------------------------------------------------------------
// match 표현식
// ----------------------------------------------------------------------------

fn match_expression() {
    println!("\n--- match 표현식 ---");

    // match는 표현식! 값을 반환함
    let number = 13;

    let description = match number {
        1 => "one",
        2 => "two",
        3 => "three",
        13 => "thirteen",
        _ => "other",  // _ 는 catch-all (C++의 default)
    };
    println!("{} is {}", number, description);

    // 모든 케이스를 처리해야 함 (exhaustive)
    // _ 를 빼면 컴파일 에러!

    // 범위 패턴
    let score = 85;
    let grade = match score {
        90..=100 => 'A',
        80..=89 => 'B',
        70..=79 => 'C',
        60..=69 => 'D',
        _ => 'F',
    };
    println!("점수 {}: 등급 {}", score, grade);

    // 여러 패턴 (OR)
    let die = 3;
    match die {
        1 | 2 | 3 => println!("작은 수"),
        4 | 5 | 6 => println!("큰 수"),
        _ => unreachable!(),  // 도달 불가능 표시
    }

    // 가드 (조건)
    let pair = (2, -2);
    match pair {
        (x, y) if x == y => println!("같음"),
        (x, y) if x + y == 0 => println!("합이 0"),
        (x, _) if x % 2 == 0 => println!("첫 번째가 짝수"),
        _ => println!("기타"),
    }

    // 바인딩 (@)
    let msg = Message::Move { x: 10, y: 20 };
    match msg {
        Message::Move { x: 0..=10, y } => {
            println!("x가 0-10 범위, y = {}", y);
        }
        Message::Move { x, y: y_val @ 15..=25 } => {
            println!("x = {}, y가 15-25 범위 ({})", x, y_val);
        }
        _ => println!("기타"),
    }
}

// ----------------------------------------------------------------------------
// if let, while let
// ----------------------------------------------------------------------------

fn if_let_while_let() {
    println!("\n--- if let, while let ---");

    // 단일 패턴만 처리할 때 match는 장황함
    let some_value = Some(3);

    // match 사용
    match some_value {
        Some(3) => println!("match: 3이다!"),
        _ => (),
    }

    // if let 사용 - 더 간결
    if let Some(3) = some_value {
        println!("if let: 3이다!");
    }

    // if let else
    if let Some(n) = some_value {
        println!("값: {}", n);
    } else {
        println!("값 없음");
    }

    // while let - 패턴이 매치하는 동안 반복
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("pop: {}", top);
    }

    // let else (Rust 1.65+) - 매치 실패 시 early return
    fn get_count(s: &str) -> Option<usize> {
        let count_str = s.strip_prefix("count: ")?;
        count_str.parse().ok()
    }

    fn process(s: &str) {
        let Some(count) = get_count(s) else {
            println!("파싱 실패");
            return;
        };
        println!("카운트: {}", count);
    }

    process("count: 42");
    process("invalid");
}

// ----------------------------------------------------------------------------
// 고급 패턴 매칭
// ----------------------------------------------------------------------------

fn pattern_matching_advanced() {
    println!("\n--- 고급 패턴 매칭 ---");

    // 구조체 분해
    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 0, y: 7 };

    match p {
        Point { x: 0, y } => println!("x축 위, y = {}", y),
        Point { x, y: 0 } => println!("y축 위, x = {}", x),
        Point { x, y } => println!("점 ({}, {})", x, y),
    }

    // 중첩 구조 분해
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }

    enum AdvancedMessage {
        ChangeColor(Color),
    }

    let msg = AdvancedMessage::ChangeColor(Color::Rgb(255, 128, 0));

    match msg {
        AdvancedMessage::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("RGB: ({}, {}, {})", r, g, b);
        }
        AdvancedMessage::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("HSV: ({}, {}, {})", h, s, v);
        }
    }

    // 무시 패턴
    let numbers = (1, 2, 3, 4, 5);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("첫째: {}, 셋째: {}, 다섯째: {}", first, third, fifth);
        }
    }

    // .. 으로 나머지 무시
    match numbers {
        (first, .., last) => {
            println!("처음: {}, 마지막: {}", first, last);
        }
    }

    // 참조 패턴
    let robot_name = Some(String::from("Bors"));

    match &robot_name {
        Some(name) => println!("로봇 이름: {}", name),
        None => (),
    }

    // robot_name은 여전히 유효 (참조로 매치했으므로)
    println!("로봇: {:?}", robot_name);
}
