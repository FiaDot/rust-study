// ============================================================================
// 03. 빌림 (Borrowing)과 참조 (References)
// ============================================================================
// C++20과의 핵심 차이점:
// 1. Rust 참조는 항상 유효함 - 댕글링 참조 불가능 (컴파일 에러)
// 2. 가변 참조는 한 번에 하나만 - 데이터 레이스 방지
// 3. 불변 참조 여러 개 OR 가변 참조 하나 - 동시 불가
// 4. 참조의 수명은 컴파일러가 추적 (다음 챕터에서 자세히)
// ============================================================================

pub fn run() {
    println!("\n=== 03. 빌림과 참조 ===\n");

    references_intro();
    mutable_references();
    reference_rules();
    dangling_references();
    slices();
}

// ----------------------------------------------------------------------------
// 참조 기초
// ----------------------------------------------------------------------------
fn references_intro() {
    println!("--- 참조 기초 ---");

    let s1 = String::from("hello");

    // & 연산자로 참조 생성 - 소유권을 넘기지 않고 빌려줌
    // C++: const std::string& ref = s1;
    let len = calculate_length(&s1);

    // s1은 여전히 유효! 소유권이 이동하지 않았음
    println!("'{}'의 길이: {}", s1, len);

    // 참조는 소유하지 않으므로 drop되지 않음
    // 참조가 가리키는 값은 참조가 사라져도 유지됨

    // 참조 역참조 (dereference)
    let x = 5;
    let r = &x;

    println!("x = {}", x);
    println!("r = {}", r);      // 자동 역참조
    println!("*r = {}", *r);    // 명시적 역참조

    // C++ 참조 vs Rust 참조:
    // C++: int& r = x;      // 참조, 재할당 불가
    // C++: int* p = &x;     // 포인터, null 가능
    // Rust: let r = &x;     // 참조, null 불가능, 재할당 가능
    // Rust: let r: &i32;    // 초기화 없이 선언 불가 (C++과 달리)
}

fn calculate_length(s: &String) -> usize {
    // s는 String에 대한 참조
    // s를 통해 값을 읽을 수 있지만 수정할 수 없음
    s.len()
}  // s가 스코프를 벗어나지만, 소유권이 없으므로 drop 안 됨

// ----------------------------------------------------------------------------
// 가변 참조
// ----------------------------------------------------------------------------
fn mutable_references() {
    println!("\n--- 가변 참조 ---");

    let mut s = String::from("hello");

    // 가변 참조로 값을 수정할 수 있음
    // C++: std::string& ref = s; (비const 참조)
    change(&mut s);

    println!("변경 후: {}", s);

    // 가변 참조의 핵심 규칙:
    // 특정 스코프에서 특정 데이터에 대한 가변 참조는 하나만 가능!

    let mut data = String::from("hello");

    let r1 = &mut data;
    // let r2 = &mut data;  // 컴파일 에러!
    // error[E0499]: cannot borrow `data` as mutable more than once

    println!("r1: {}", r1);
    // r1의 사용이 끝난 후에는 새로운 가변 참조 가능
    let r2 = &mut data;
    println!("r2: {}", r2);

    // 이 규칙이 데이터 레이스를 방지:
    // - 두 개 이상의 포인터가 동시에 같은 데이터에 접근
    // - 적어도 하나가 쓰기 작업
    // - 동기화 없음
    // Rust는 컴파일 타임에 이를 방지!
}

fn change(s: &mut String) {
    s.push_str(", world");
}

// ----------------------------------------------------------------------------
// 참조 규칙 상세
// ----------------------------------------------------------------------------
fn reference_rules() {
    println!("\n--- 참조 규칙 ---");

    let mut s = String::from("hello");

    // 규칙: 불변 참조 여러 개 OR 가변 참조 하나
    // 불변 참조 여러 개는 OK (모두 읽기만 하니까)
    let r1 = &s;
    let r2 = &s;
    println!("r1: {}, r2: {}", r1, r2);
    // r1, r2의 마지막 사용 지점 이후...

    // 이제 가변 참조 가능 (NLL - Non-Lexical Lifetimes)
    let r3 = &mut s;
    println!("r3: {}", r3);

    // 불변 참조와 가변 참조 동시 사용 불가
    let mut data = String::from("hello");
    let r_immut = &data;
    // let r_mut = &mut data;  // 에러! 불변 참조가 아직 사용 중
    println!("불변 참조: {}", r_immut);
    // r_immut 사용 끝
    let r_mut = &mut data;  // 이제 OK
    println!("가변 참조: {}", r_mut);

    // C++에서는 이런 버그가 런타임에 발생할 수 있음:
    // std::vector<int> v = {1, 2, 3};
    // int& ref = v[0];
    // v.push_back(4);  // 재할당 가능성
    // ref = 10;        // 댕글링 참조! 정의되지 않은 동작

    // Rust에서는 컴파일 에러로 방지됨
}

// ----------------------------------------------------------------------------
// 댕글링 참조 방지
// ----------------------------------------------------------------------------
fn dangling_references() {
    println!("\n--- 댕글링 참조 방지 ---");

    // Rust는 댕글링 참조를 컴파일 타임에 방지

    // 이 함수는 컴파일되지 않음:
    // fn dangle() -> &String {
    //     let s = String::from("hello");
    //     &s  // s는 함수 끝에서 drop됨
    //        // 반환되는 참조는 해제된 메모리를 가리킴!
    // }
    // error[E0106]: missing lifetime specifier

    // 해결책: 소유권을 반환
    let s = no_dangle();
    println!("안전하게 반환: {}", s);

    // C++에서 흔한 버그:
    // const std::string& dangle() {
    //     std::string s = "hello";
    //     return s;  // 경고는 나오지만 컴파일됨!
    // }
    // // 호출 시 정의되지 않은 동작
}

fn no_dangle() -> String {
    let s = String::from("hello");
    s  // 소유권 이동, 안전함!
}

// ----------------------------------------------------------------------------
// 슬라이스 (Slice)
// ----------------------------------------------------------------------------
fn slices() {
    println!("\n--- 슬라이스 ---");

    // 슬라이스는 컬렉션의 일부를 참조
    // C++20: std::span과 유사

    let s = String::from("hello world");

    // 문자열 슬라이스 &str
    let hello: &str = &s[0..5];   // "hello"
    let world: &str = &s[6..11];  // "world"
    println!("{} {}", hello, world);

    // 범위 문법
    let s = String::from("hello");
    let slice1 = &s[0..2];    // "he"
    let slice2 = &s[..2];     // "he" (0 생략)
    let slice3 = &s[3..];     // "lo" (끝까지)
    let slice4 = &s[..];      // "hello" (전체)
    println!("{}, {}, {}, {}", slice1, slice2, slice3, slice4);

    // 문자열 리터럴은 슬라이스!
    let s: &str = "Hello, world!";  // 바이너리에 저장된 문자열을 가리킴
    println!("리터럴: {}", s);

    // 슬라이스의 장점 - 원본과 동기화
    let mut s = String::from("hello world");

    let word = first_word(&s);
    println!("첫 단어: {}", word);

    // s.clear();  // 에러! 불변 참조(word)가 있는 동안 가변 작업 불가
    // error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable

    println!("word 사용 후: {}", word);

    // 배열 슬라이스
    let a = [1, 2, 3, 4, 5];
    let slice: &[i32] = &a[1..3];  // [2, 3]
    println!("배열 슬라이스: {:?}", slice);
}

fn first_word(s: &str) -> &str {
    // &str을 받으면 String과 &str 모두 처리 가능
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
