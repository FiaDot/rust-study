// ============================================================================
// 05. 구조체 (Structs)
// ============================================================================
// C++20과의 핵심 차이점:
// 1. struct와 class 구분 없음 - 모두 struct (기본 private 없음)
// 2. 메서드는 impl 블록에 별도로 정의
// 3. 상속 없음 - 대신 컴포지션과 트레이트 사용
// 4. 생성자 없음 - 연관 함수로 대체 (관례: new, from_* 등)
// ============================================================================

pub fn run() {
    println!("\n=== 05. 구조체 ===\n");

    basic_struct();
    tuple_structs();
    unit_struct();
    methods();
    associated_functions();
}

// ----------------------------------------------------------------------------
// 기본 구조체
// ----------------------------------------------------------------------------

// C++:
// struct User {
//     bool active;
//     std::string username;
//     std::string email;
//     int sign_in_count;
// };

// Rust:
#[derive(Debug)]  // 디버그 출력을 위한 트레이트 자동 구현
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn basic_struct() {
    println!("--- 기본 구조체 ---");

    // 인스턴스 생성 - 모든 필드 초기화 필수
    // C++: User user1{true, "user1", "user1@example.com", 1};
    let mut user1 = User {
        active: true,
        username: String::from("user1"),
        email: String::from("user1@example.com"),
        sign_in_count: 1,
    };

    // 필드 접근 (dot notation)
    println!("사용자명: {}", user1.username);

    // 가변 인스턴스면 필드 수정 가능
    user1.email = String::from("new_email@example.com");
    println!("새 이메일: {}", user1.email);

    // 필드 초기화 단축 문법 (Field Init Shorthand)
    // 변수명과 필드명이 같으면 한 번만 작성
    let email = String::from("user2@example.com");
    let username = String::from("user2");

    let user2 = User {
        email,           // email: email 대신
        username,        // username: username 대신
        active: true,
        sign_in_count: 1,
    };

    println!("user2: {:?}", user2);

    // 구조체 업데이트 문법 (Struct Update Syntax)
    // C++에는 없는 기능
    let user3 = User {
        email: String::from("user3@example.com"),
        ..user2  // 나머지 필드는 user2에서 가져옴
    };

    println!("user3 이메일: {}", user3.email);
    // 주의: user2의 username이 이동됨! (String은 Copy가 아님)
    // println!("{}", user2.username);  // 에러!
    println!("user2 active: {}", user2.active);  // OK (bool은 Copy)
}

// ----------------------------------------------------------------------------
// 튜플 구조체
// ----------------------------------------------------------------------------

// 이름 있는 튜플 - 타입 구분을 위해 사용
// C++: using Color = std::tuple<int, int, int>; 와 비슷하지만 더 타입 안전

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn tuple_structs() {
    println!("\n--- 튜플 구조체 ---");

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // 같은 필드 구조여도 다른 타입!
    // let c: Color = origin;  // 컴파일 에러!

    // 인덱스로 접근
    println!("Color R: {}", black.0);
    println!("Point x: {}", origin.0);

    // 구조 분해
    let Color(r, g, b) = black;
    println!("RGB: {}, {}, {}", r, g, b);

    // Newtype 패턴 - 기존 타입을 감싸서 새 타입 생성
    struct Meters(f64);
    struct Kilometers(f64);

    let distance = Meters(100.0);
    // 실수로 다른 단위와 섞는 것을 방지
    // let km: Kilometers = distance;  // 컴파일 에러!
    println!("거리: {} 미터", distance.0);
}

// ----------------------------------------------------------------------------
// 유닛 구조체
// ----------------------------------------------------------------------------

// 필드가 없는 구조체 - 트레이트 구현에 주로 사용
// C++: struct Empty {}; 와 유사

struct AlwaysEqual;

fn unit_struct() {
    println!("\n--- 유닛 구조체 ---");

    let _subject = AlwaysEqual;

    // 주로 트레이트 구현할 때 사용
    // impl SomeTrait for AlwaysEqual { ... }
}

// ----------------------------------------------------------------------------
// 메서드
// ----------------------------------------------------------------------------

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// 메서드는 impl 블록에 정의
// C++과 달리 구조체 정의와 분리됨
impl Rectangle {
    // 메서드의 첫 번째 매개변수는 항상 self
    // &self = 불변 빌림 (C++: const 멤버 함수)
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // &mut self = 가변 빌림 (C++: 비const 멤버 함수)
    fn double_size(&mut self) {
        self.width *= 2;
        self.height *= 2;
    }

    // self = 소유권 획득 (C++: 이동 후 소멸)
    fn consume(self) -> u32 {
        self.width * self.height
        // self는 이 함수가 끝나면 drop됨
    }

    // 여러 매개변수를 가진 메서드
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn methods() {
    println!("\n--- 메서드 ---");

    let mut rect = Rectangle {
        width: 30,
        height: 50,
    };

    // 메서드 호출 - 자동 참조/역참조
    // Rust는 자동으로 &, &mut, * 를 추가
    // rect.area()는 (&rect).area()와 동일
    println!("넓이: {}", rect.area());

    // 가변 메서드
    rect.double_size();
    println!("두 배 후 넓이: {}", rect.area());

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    println!("rect가 rect2를 포함할 수 있나? {}", rect.can_hold(&rect2));

    // 소유권을 가져가는 메서드
    let final_area = rect.consume();
    println!("최종 넓이: {}", final_area);
    // println!("{:?}", rect);  // 에러! rect는 이동됨
}

// impl 블록은 여러 개 가능
impl Rectangle {
    fn is_square(&self) -> bool {
        self.width == self.height
    }
}

// ----------------------------------------------------------------------------
// 연관 함수 (Associated Functions)
// ----------------------------------------------------------------------------

impl Rectangle {
    // self가 없는 함수 = 연관 함수 (C++의 static 멤버 함수)
    // 생성자 패턴으로 주로 사용
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn associated_functions() {
    println!("\n--- 연관 함수 ---");

    // :: 문법으로 호출 (C++과 동일)
    let rect = Rectangle::new(30, 50);
    println!("새 사각형: {:?}", rect);

    let square = Rectangle::square(25);
    println!("정사각형: {:?}", square);
    println!("정사각형인가? {}", square.is_square());

    // C++ 비교:
    // class Rectangle {
    // public:
    //     static Rectangle create(int w, int h) { return Rectangle{w, h}; }
    //     int area() const { return width * height; }
    // private:
    //     int width, height;
    // };
    //
    // auto rect = Rectangle::create(30, 50);
    // rect.area();
}
