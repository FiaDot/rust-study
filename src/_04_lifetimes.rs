// ============================================================================
// 04. 수명 (Lifetimes)
// ============================================================================
// C++20과의 핵심 차이점:
// 1. C++에는 수명 개념이 없음 - 댕글링 참조 검증을 프로그래머에게 맡김
// 2. Rust는 컴파일러가 모든 참조의 수명을 추적
// 3. 대부분의 경우 수명은 추론됨 (수명 생략 규칙)
// 4. 명시적 수명 어노테이션은 컴파일러에게 힌트를 주는 것
// ============================================================================

pub fn run() {
    println!("\n=== 04. 수명 ===\n");

    lifetime_basics();
    lifetime_annotations();
    lifetime_in_structs();
    static_lifetime();
}

// ----------------------------------------------------------------------------
// 수명 기초
// ----------------------------------------------------------------------------
fn lifetime_basics() {
    println!("--- 수명 기초 ---");

    // 모든 참조는 수명을 가짐 - 참조가 유효한 범위
    // 대부분의 경우 수명은 암묵적이고 추론됨

    let r;                      // 참조 선언
    {
        let x = 5;
        r = &x;                 // x의 참조를 r에 저장
        println!("r: {}", r);   // 여기서는 OK
    }  // x가 스코프를 벗어남
    // println!("r: {}", r);    // 에러! r은 댕글링 참조

    // 위 코드가 컴파일되지 않는 이유:
    // - r의 수명: 외부 스코프 전체
    // - x의 수명: 내부 블록만
    // - r이 x보다 오래 살아남으려 함 = 컴파일 에러

    // C++에서는 이런 코드가 컴파일됨 (정의되지 않은 동작):
    // int* r;
    // {
    //     int x = 5;
    //     r = &x;
    // }
    // std::cout << *r;  // 댕글링 포인터!
}

// ----------------------------------------------------------------------------
// 수명 어노테이션
// ----------------------------------------------------------------------------
fn lifetime_annotations() {
    println!("\n--- 수명 어노테이션 ---");

    // 두 문자열 중 긴 것을 반환하는 함수를 생각해보자
    // 반환되는 참조는 어떤 수명을 가져야 할까?

    let string1 = String::from("long string is long");
    let result;

    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
        println!("긴 문자열: {}", result);
    }
    // result를 여기서 사용하면? string2가 이미 drop됨
    // 컴파일러는 result가 string2를 참조할 수 있음을 알고 있음

    // 수명 어노테이션을 사용하지 않으면:
    // fn longest(x: &str, y: &str) -> &str {  // 컴파일 에러!
    //     if x.len() > y.len() { x } else { y }
    // }
    // error[E0106]: missing lifetime specifier

    // 컴파일러가 물어보는 것:
    // "반환값의 수명이 x와 같아? y와 같아? 둘 다?"

    // 다른 예제: 항상 첫 번째 매개변수 반환
    let s1 = String::from("hello");
    let s2 = String::from("world");
    let result = first(&s1, &s2);
    println!("첫 번째: {}", result);
}

// 수명 어노테이션 문법: 'a (작은따옴표 + 소문자)
// 'a는 "x와 y 중 더 짧은 수명"을 의미
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    // x와 y는 최소한 'a 만큼 살아있음
    // 반환값도 최소한 'a 만큼 유효함
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 항상 첫 번째 매개변수만 반환하면 두 번째는 수명 불필요
fn first<'a>(x: &'a str, _y: &str) -> &'a str {
    x
}

// 수명은 함수 시그니처의 계약:
// "이 참조들이 유효한 동안 반환값도 유효하다"

// 수명 생략 규칙 (Lifetime Elision Rules)
// 컴파일러가 자동으로 수명을 추론하는 규칙:
// 1. 각 참조 매개변수는 자신만의 수명을 가짐
// 2. 입력 수명이 하나면 출력 수명도 그것과 같음
// 3. &self나 &mut self가 있으면 self의 수명이 출력 수명

// 따라서 이것은:
fn first_word(s: &str) -> &str {
    // 수명 생략 규칙 적용됨
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

// 이것과 동일:
fn _first_word_explicit<'a>(s: &'a str) -> &'a str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

// ----------------------------------------------------------------------------
// 구조체에서의 수명
// ----------------------------------------------------------------------------
fn lifetime_in_structs() {
    println!("\n--- 구조체에서의 수명 ---");

    // 구조체가 참조를 포함하면 수명 어노테이션 필요
    // 구조체는 그 참조보다 오래 살 수 없음

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();

    let excerpt = ImportantExcerpt {
        part: first_sentence,
    };

    println!("발췌: {}", excerpt.part);

    // C++에서 비슷한 패턴 (위험할 수 있음):
    // struct ImportantExcerpt {
    //     std::string_view part;  // 댕글링 가능성 있음
    // };

    // Rust에서는 수명 어노테이션으로 안전성 보장
    // excerpt는 novel보다 오래 살 수 없음 (컴파일러가 보장)
}

// 참조를 포함하는 구조체는 수명 어노테이션 필요
struct ImportantExcerpt<'a> {
    part: &'a str,  // 'a 수명 동안 유효한 문자열 슬라이스
}

// 구조체 메서드에서의 수명
impl<'a> ImportantExcerpt<'a> {
    // self 참조의 수명이 반환값에 자동 적용 (규칙 3)
    fn level(&self) -> i32 {
        3
    }

    // 여러 참조가 있어도 &self가 있으면 규칙 3 적용
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("주목하세요: {}", announcement);
        self.part
    }
}

// ----------------------------------------------------------------------------
// 정적 수명
// ----------------------------------------------------------------------------
fn static_lifetime() {
    println!("\n--- 정적 수명 ---");

    // 'static 수명 = 프로그램 전체 기간 동안 유효
    // 문자열 리터럴은 'static 수명을 가짐 (바이너리에 저장)

    let s: &'static str = "프로그램 전체 동안 유효";
    println!("{}", s);

    // C++에서 유사한 개념:
    // const char* s = "literal";  // 정적 저장 기간

    // 'static을 남용하지 말 것!
    // 대부분의 경우 댕글링 참조 문제는 수명 어노테이션으로 해결
    // 'static은 정말 프로그램 전체 기간이 필요할 때만 사용

    // 제네릭 + 트레이트 바운드 + 수명을 모두 함께 사용
    use std::fmt::Display;

    fn longest_with_announcement<'a, T>(
        x: &'a str,
        y: &'a str,
        ann: T,
    ) -> &'a str
    where
        T: Display,
    {
        println!("알림: {}", ann);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    let result = longest_with_announcement(
        "hello",
        "world!",
        "수명과 제네릭 함께 사용",
    );
    println!("결과: {}", result);
}
