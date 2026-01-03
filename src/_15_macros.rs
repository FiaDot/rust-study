// ============================================================================
// 15. 매크로 (Macros)
// ============================================================================
// C++20과의 핵심 차이점:
// 1. Rust 매크로는 AST 레벨에서 동작 - C++ 전처리기는 텍스트 치환
// 2. 위생적(hygienic) 매크로 - 이름 충돌 방지
// 3. 패턴 매칭 기반 - C++ 템플릿 메타프로그래밍과 유사한 표현력
// 4. 컴파일 타임에 타입 검사 - C++ 매크로는 타입 안전하지 않음
// 5. 절차적 매크로로 derive, attribute 등 구현 가능
// ============================================================================

pub fn run() {
    println!("\n=== 15. 매크로 ===\n");

    declarative_macros();
    macro_patterns();
    repetition();
    hygiene();
    useful_macros();
    procedural_macros_intro();
}

// ----------------------------------------------------------------------------
// 선언적 매크로 기초 (macro_rules!)
// ----------------------------------------------------------------------------

// 가장 간단한 매크로
// C++: #define SAY_HELLO() std::cout << "Hello!" << std::endl
macro_rules! say_hello {
    () => {
        println!("안녕하세요!");
    };
}

// 인자를 받는 매크로
// C++: #define PRINT_VAR(x) std::cout << #x << " = " << x << std::endl
macro_rules! print_var {
    ($var:expr) => {
        println!("{} = {:?}", stringify!($var), $var);
    };
}

fn declarative_macros() {
    println!("--- 선언적 매크로 기초 ---");

    // 매크로 호출 - ! 가 매크로임을 표시
    say_hello!();

    let x = 42;
    let name = "Rust";
    print_var!(x);
    print_var!(name);
    print_var!(x + 10);

    // C++ 매크로와의 차이:
    // 1. 매크로 이름 뒤에 ! 필수 - 함수와 구분
    // 2. 인자가 표현식으로 파싱됨 - 텍스트 치환 아님
    // 3. stringify!로 코드를 문자열로 변환 가능
}

// ----------------------------------------------------------------------------
// 매크로 패턴
// ----------------------------------------------------------------------------

// 다양한 지정자(designator) 사용
// $name:지정자 형태로 인자 캡처

// 주요 지정자:
// expr  - 표현식
// ident - 식별자 (변수명, 함수명 등)
// ty    - 타입
// pat   - 패턴
// stmt  - 문장
// block - 블록
// item  - 아이템 (함수, 구조체 등)
// path  - 경로 (std::vec::Vec)
// tt    - 토큰 트리 (모든 것)
// literal - 리터럴 값

macro_rules! create_function {
    ($func_name:ident) => {
        fn $func_name() {
            println!("함수 {}가 호출됨", stringify!($func_name));
        }
    };
}

macro_rules! print_type {
    ($val:expr, $t:ty) => {
        let _: $t = $val;
        println!("{}: {}", stringify!($val), std::any::type_name::<$t>());
    };
}

// 여러 패턴 매칭 (오버로딩과 유사)
macro_rules! calculate {
    // 패턴 1: 두 값 더하기
    (add $a:expr, $b:expr) => {
        $a + $b
    };
    // 패턴 2: 두 값 곱하기
    (mul $a:expr, $b:expr) => {
        $a * $b
    };
    // 패턴 3: 단일 값 제곱
    (square $a:expr) => {
        $a * $a
    };
}

fn macro_patterns() {
    println!("\n--- 매크로 패턴 ---");

    // ident로 함수 생성
    create_function!(foo);
    create_function!(bar);
    foo();
    bar();

    // ty로 타입 지정
    print_type!(42, i32);
    print_type!(3.14, f64);

    // 패턴 매칭
    println!("add: {}", calculate!(add 2, 3));
    println!("mul: {}", calculate!(mul 4, 5));
    println!("square: {}", calculate!(square 6));
}

// ----------------------------------------------------------------------------
// 반복 (Repetition)
// ----------------------------------------------------------------------------

// $(...),* 형태로 반복
// * : 0회 이상
// + : 1회 이상
// ? : 0회 또는 1회

// vec! 매크로와 유사한 구현
macro_rules! my_vec {
    // 빈 벡터
    () => {
        Vec::new()
    };
    // 요소가 있는 벡터
    ($($element:expr),+ $(,)?) => {
        {
            let mut v = Vec::new();
            $(
                v.push($element);
            )+
            v
        }
    };
}

// 가변 인자 함수처럼 동작하는 매크로
macro_rules! sum {
    ($($x:expr),*) => {
        {
            let mut total = 0;
            $(
                total += $x;
            )*
            total
        }
    };
}

// 구조체 필드 생성
macro_rules! make_struct {
    ($name:ident { $($field:ident : $t:ty),* $(,)? }) => {
        #[derive(Debug)]
        struct $name {
            $($field: $t),*
        }
    };
}

fn repetition() {
    println!("\n--- 반복 ---");

    // my_vec! 사용
    let v1: Vec<i32> = my_vec!();
    let v2 = my_vec![1, 2, 3];
    let v3 = my_vec![10, 20, 30, 40,];  // 후행 쉼표 OK
    println!("v1: {:?}", v1);
    println!("v2: {:?}", v2);
    println!("v3: {:?}", v3);

    // sum! 사용
    println!("sum: {}", sum!(1, 2, 3, 4, 5));
    println!("sum empty: {}", sum!());

    // 구조체 생성
    make_struct!(Point { x: i32, y: i32 });
    let p = Point { x: 10, y: 20 };
    println!("Point: {:?}", p);

    // C++ 가변 인자 템플릿과 비교:
    // template<typename... Args>
    // auto sum(Args... args) { return (args + ...); }
    //
    // Rust 매크로가 더 유연하고 복잡한 패턴 가능
}

// ----------------------------------------------------------------------------
// 위생성 (Hygiene)
// ----------------------------------------------------------------------------

macro_rules! five_times {
    ($x:expr) => {
        5 * $x
    };
}

macro_rules! create_var {
    ($name:ident, $value:expr) => {
        let $name = $value;
    };
}

// 매크로 내부 변수는 외부와 충돌하지 않음
macro_rules! using_temp {
    ($e:expr) => {
        {
            let temp = $e;  // 이 temp는 외부 temp와 다름
            temp * temp
        }
    };
}

fn hygiene() {
    println!("\n--- 위생성 (Hygiene) ---");

    // 기본적인 매크로 확장
    let result = five_times!(2 + 3);  // 5 * (2 + 3) = 25
    println!("five_times!(2 + 3) = {}", result);

    // C++ 매크로의 문제:
    // #define FIVE_TIMES(x) 5 * x
    // FIVE_TIMES(2 + 3) = 5 * 2 + 3 = 13  // 의도와 다름!

    // 변수 생성
    create_var!(answer, 42);
    println!("answer = {}", answer);

    // 위생적 매크로 - 이름 충돌 방지
    let temp = 10;
    let squared = using_temp!(temp + 5);
    println!("temp = {}, squared = {}", temp, squared);
    // 매크로 내부의 temp와 외부의 temp는 별개

    // C++ 매크로에서는 이름 충돌 위험:
    // #define SQUARE(x) ({ int temp = (x); temp * temp; })
    // int temp = 10;
    // SQUARE(temp + 5);  // 이름 충돌 가능성!
}

// ----------------------------------------------------------------------------
// 유용한 매크로 패턴
// ----------------------------------------------------------------------------

// 에러와 함께 조기 반환
macro_rules! try_or_return {
    ($expr:expr) => {
        match $expr {
            Ok(val) => val,
            Err(e) => {
                println!("에러 발생: {:?}", e);
                return;
            }
        }
    };
}

// 해시맵 생성
macro_rules! hashmap {
    ($($key:expr => $value:expr),* $(,)?) => {
        {
            let mut map = std::collections::HashMap::new();
            $(
                map.insert($key, $value);
            )*
            map
        }
    };
}

// 조건부 컴파일과 함께 사용
macro_rules! debug_print {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        println!("[DEBUG] {}", format!($($arg)*));
    };
}

// 메서드 체이닝 빌더
macro_rules! builder_field {
    ($name:ident, $t:ty) => {
        pub fn $name(mut self, value: $t) -> Self {
            self.$name = value;
            self
        }
    };
}

struct RequestBuilder {
    url: String,
    method: String,
    timeout: u32,
}

impl RequestBuilder {
    fn new() -> Self {
        RequestBuilder {
            url: String::new(),
            method: String::from("GET"),
            timeout: 30,
        }
    }

    builder_field!(url, String);
    builder_field!(method, String);
    builder_field!(timeout, u32);

    fn build(self) -> String {
        format!("{} {} (timeout: {}s)", self.method, self.url, self.timeout)
    }
}

fn useful_macros() {
    println!("\n--- 유용한 매크로 패턴 ---");

    // hashmap! 매크로
    let scores = hashmap! {
        "Alice" => 100,
        "Bob" => 85,
        "Carol" => 92,
    };
    println!("점수: {:?}", scores);

    // debug_print! - 디버그 빌드에서만 출력
    debug_print!("이것은 디버그 메시지입니다: {}", 42);

    // 빌더 패턴
    let request = RequestBuilder::new()
        .url(String::from("https://api.example.com"))
        .method(String::from("POST"))
        .timeout(60)
        .build();
    println!("요청: {}", request);

    // 표준 라이브러리의 유용한 매크로들
    // println!, format!, vec!, panic!, assert!, cfg!, include_str! 등

    // concat! - 컴파일 타임 문자열 연결
    let s = concat!("Hello", ", ", "World", "!");
    println!("concat!: {}", s);

    // include_str! - 파일 내용을 문자열로 포함
    // let content = include_str!("data.txt");

    // env! - 컴파일 타임 환경 변수
    let version = env!("CARGO_PKG_VERSION");
    println!("패키지 버전: {}", version);
}

// ----------------------------------------------------------------------------
// 절차적 매크로 소개
// ----------------------------------------------------------------------------

fn procedural_macros_intro() {
    println!("\n--- 절차적 매크로 소개 ---");

    // 절차적 매크로는 별도 크레이트에서 정의해야 함
    // 여기서는 개념만 설명

    println!("절차적 매크로의 세 가지 종류:");
    println!("1. derive 매크로 - #[derive(MyTrait)]");
    println!("2. attribute 매크로 - #[my_attribute]");
    println!("3. function-like 매크로 - my_macro!(...)");

    // derive 매크로 예시 (serde)
    // #[derive(Serialize, Deserialize)]
    // struct User { name: String, age: u32 }

    // attribute 매크로 예시 (tokio)
    // #[tokio::main]
    // async fn main() { ... }

    // 절차적 매크로 작성 (별도 크레이트 필요):
    //
    // // my_macro/src/lib.rs
    // use proc_macro::TokenStream;
    //
    // #[proc_macro_derive(MyTrait)]
    // pub fn my_trait_derive(input: TokenStream) -> TokenStream {
    //     // TokenStream 파싱 및 코드 생성
    // }

    println!("\n실제 사용 중인 derive 매크로들:");
    println!("- Debug, Clone, Copy, PartialEq, Eq, Hash, Default");
    println!("- serde: Serialize, Deserialize");
    println!("- thiserror: Error");

    // C++ 템플릿 메타프로그래밍과 비교:
    // - Rust 매크로는 더 명시적이고 읽기 쉬움
    // - 에러 메시지가 더 명확함
    // - 절차적 매크로는 임의의 Rust 코드 실행 가능

    // derive 예시 - 이미 표준 라이브러리에서 사용 중
    #[derive(Debug, Clone, PartialEq)]
    struct DemoPoint {
        x: i32,
        y: i32,
    }

    let p1 = DemoPoint { x: 1, y: 2 };
    let p2 = p1.clone();
    println!("Debug: {:?}", p1);
    println!("PartialEq: {}", p1 == p2);
}
