// ============================================================================
// 02. 소유권 (Ownership) - Rust의 핵심 개념
// ============================================================================
// C++20과의 핵심 차이점:
// 1. Rust는 컴파일 타임에 메모리 안전성 보장 - 런타임 오버헤드 없음
// 2. 모든 값은 정확히 하나의 소유자를 가짐 - C++ unique_ptr과 유사하지만 언어 레벨
// 3. 대입은 기본적으로 이동(move) - C++의 std::move가 기본 동작
// 4. 소유자가 스코프를 벗어나면 자동으로 해제 - RAII와 동일
// ============================================================================

pub fn run() {
    println!("\n=== 02. 소유권 ===\n");

    ownership_rules();
    move_semantics();
    clone_and_copy();
    ownership_functions();
}

// ----------------------------------------------------------------------------
// 소유권 규칙
// ----------------------------------------------------------------------------
fn ownership_rules() {
    println!("--- 소유권 규칙 ---");

    // Rust의 세 가지 소유권 규칙:
    // 1. 각 값은 해당 값의 소유자(owner)라고 불리는 변수를 가진다
    // 2. 한 번에 하나의 소유자만 존재할 수 있다
    // 3. 소유자가 스코프를 벗어나면, 값은 버려진다(dropped)

    {
        // s는 여기서 유효하지 않음 (아직 선언 안됨)
        let s = String::from("hello");  // s가 이 시점부터 유효
        println!("s = {}", s);
        // s를 가지고 작업 수행
    }  // 스코프 종료, s의 drop이 호출됨 (C++의 소멸자와 유사)

    // C++에서의 RAII와 동일한 개념:
    // {
    //     std::string s = "hello";
    // }  // s의 소멸자 호출
}

// ----------------------------------------------------------------------------
// 이동 시맨틱스 (Move Semantics)
// ----------------------------------------------------------------------------
fn move_semantics() {
    println!("\n--- 이동 시맨틱스 ---");

    // 스택에 저장되는 기본 타입은 복사됨
    let x = 5;
    let y = x;  // 값이 복사됨
    println!("x = {}, y = {}", x, y);  // 둘 다 사용 가능

    // 힙에 저장되는 String은 이동됨!
    let s1 = String::from("hello");
    let s2 = s1;  // s1의 소유권이 s2로 이동 (move)

    // println!("s1 = {}", s1);  // 컴파일 에러! s1은 더 이상 유효하지 않음
    // error[E0382]: borrow of moved value: `s1`

    println!("s2 = {}", s2);  // OK

    // C++과의 비교:
    // C++: std::string s1 = "hello";
    //      std::string s2 = s1;  // 복사! (깊은 복사)
    //      std::string s3 = std::move(s1);  // 이동 (명시적)
    //      // s1은 여전히 접근 가능하지만 "유효하지만 불특정" 상태

    // Rust에서는 이동이 기본이고, 이동 후 원본 사용이 컴파일 에러
    // 이것이 더 안전함 - "use after move" 버그를 원천 차단

    // 왜 이동이 기본인가?
    // String의 내부 구조:
    // ┌──────────┬─────────┬─────────┐
    // │   ptr    │   len   │   cap   │  <- 스택 (24바이트)
    // └────┬─────┴─────────┴─────────┘
    //      │
    //      v
    // ┌────┬────┬────┬────┬────┐
    // │ h  │ e  │ l  │ l  │ o  │       <- 힙
    // └────┴────┴────┴────┴────┘

    // 만약 s1과 s2가 같은 힙 데이터를 가리키면?
    // 둘 다 스코프를 벗어날 때 같은 메모리를 해제하려 함 = double free!
    // Rust는 이동으로 이 문제를 해결
}

// ----------------------------------------------------------------------------
// Clone과 Copy
// ----------------------------------------------------------------------------
fn clone_and_copy() {
    println!("\n--- Clone과 Copy ---");

    // 깊은 복사가 필요하면 clone() 명시적 호출
    let s1 = String::from("hello");
    let s2 = s1.clone();  // 힙 데이터까지 복사

    println!("s1 = {}, s2 = {}", s1, s2);  // 둘 다 유효!

    // C++: std::string s2 = s1;  // 암묵적 깊은 복사
    // Rust는 비용이 큰 작업을 명시적으로 만듦

    // Copy 트레이트 - 스택에만 있는 타입들
    // 이 타입들은 이동 대신 복사됨:
    // - 모든 정수 타입 (i32, u64 등)
    // - 불리언 (bool)
    // - 부동소수점 (f32, f64)
    // - 문자 (char)
    // - 튜플 (모든 요소가 Copy인 경우)

    let a: i32 = 5;
    let b = a;  // 복사됨
    println!("a = {}, b = {}", a, b);  // 둘 다 OK

    // Copy 타입인 튜플
    let point = (3, 4);
    let another_point = point;  // 복사
    println!("point = {:?}, another = {:?}", point, another_point);

    // Copy가 아닌 타입을 포함한 튜플은 이동됨
    let mixed = (String::from("hello"), 5);
    let _moved = mixed;
    // println!("{:?}", mixed);  // 에러! mixed는 이동됨
}

// ----------------------------------------------------------------------------
// 함수와 소유권
// ----------------------------------------------------------------------------
fn ownership_functions() {
    println!("\n--- 함수와 소유권 ---");

    // 함수에 값을 전달하면 소유권이 이동됨
    let s = String::from("hello");
    takes_ownership(s);
    // println!("{}", s);  // 에러! s의 소유권은 함수로 이동됨

    let x = 5;
    makes_copy(x);
    println!("x는 여전히 사용 가능: {}", x);  // OK, i32는 Copy

    // 함수가 값을 반환하면 소유권이 호출자에게 이동
    let s1 = gives_ownership();
    println!("받은 소유권: {}", s1);

    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
    // println!("{}", s2);  // 에러! s2는 이동됨
    println!("돌려받은 소유권: {}", s3);

    // C++에서의 유사한 패턴:
    // void takes_ownership(std::unique_ptr<std::string> s) { ... }
    // std::unique_ptr<std::string> ptr = std::make_unique<std::string>("hello");
    // takes_ownership(std::move(ptr));  // 명시적 move 필요
    // // ptr은 이제 nullptr

    println!("\n--- 소유권 주고받기 패턴 ---");

    // 매번 소유권을 주고받는 것은 번거로움
    // 해결책: 참조(borrowing) - 다음 챕터에서 다룸
    let s4 = String::from("hello");
    let (s5, len) = calculate_length_awkward(s4);
    println!("'{}'의 길이: {}", s5, len);

    // 더 좋은 방법은 참조를 사용하는 것 (03_borrowing.rs에서 다룸)
}

fn takes_ownership(some_string: String) {
    println!("소유권을 받음: {}", some_string);
}  // some_string이 스코프를 벗어나고 drop 호출

fn makes_copy(some_integer: i32) {
    println!("복사본을 받음: {}", some_integer);
}  // some_integer가 스코프를 벗어나지만, 특별한 일은 없음

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string  // 반환되면서 호출자에게 소유권 이동
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string  // 받은 것을 그대로 반환, 소유권 이동
}

fn calculate_length_awkward(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)  // 소유권을 돌려주기 위해 튜플로 반환 (번거로움!)
}
