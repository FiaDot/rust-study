// ============================================================================
// 07. 트레이트 (Traits)
// ============================================================================
// C++20과의 핵심 차이점:
// 1. 트레이트 = 인터페이스 + C++20 concepts의 조합
// 2. 상속 없이 다형성 구현 (컴포지션 선호)
// 3. impl Trait으로 정적 디스패치, dyn Trait으로 동적 디스패치
// 4. 기존 타입에 트레이트 구현 가능 (extension traits)
// 5. 연산자 오버로딩도 트레이트로 구현
// ============================================================================

use std::fmt::{Debug, Display};
use std::ops::Add;

pub fn run() {
    println!("\n=== 07. 트레이트 ===\n");

    basic_traits();
    default_implementations();
    trait_bounds();
    trait_objects();
    derive_traits();
    operator_overloading();
    supertraits();
}

// ----------------------------------------------------------------------------
// 기본 트레이트
// ----------------------------------------------------------------------------

// 트레이트 정의 - C++의 순수 가상 함수를 가진 추상 클래스와 유사
// C++:
// class Summary {
// public:
//     virtual std::string summarize() const = 0;
// };

trait Summary {
    fn summarize(&self) -> String;
}

struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

// 트레이트 구현
// C++: class NewsArticle : public Summary { ... };

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn basic_traits() {
    println!("--- 기본 트레이트 ---");

    let article = NewsArticle {
        headline: String::from("Rust 2.0 출시!"),
        location: String::from("서울"),
        author: String::from("홍길동"),
        content: String::from("대단한 내용..."),
    };

    let tweet = Tweet {
        username: String::from("user123"),
        content: String::from("Rust 최고!"),
        reply: false,
        retweet: false,
    };

    println!("기사: {}", article.summarize());
    println!("트윗: {}", tweet.summarize());
}

// ----------------------------------------------------------------------------
// 기본 구현
// ----------------------------------------------------------------------------

trait Greet {
    // 기본 구현 제공 - C++의 가상 함수 (순수 아닌)
    fn greet(&self) -> String {
        String::from("안녕하세요!")
    }

    // 다른 메서드 호출 가능
    fn greet_twice(&self) -> String {
        format!("{} {}", self.greet(), self.greet())
    }
}

struct Person {
    name: String,
}

struct Robot {
    id: u32,
}

// 기본 구현 그대로 사용
impl Greet for Person {}

// 기본 구현 오버라이드
impl Greet for Robot {
    fn greet(&self) -> String {
        format!("삐빅. 로봇 {} 입니다.", self.id)
    }
}

fn default_implementations() {
    println!("\n--- 기본 구현 ---");

    let person = Person {
        name: String::from("철수"),
    };
    let robot = Robot { id: 42 };

    println!("사람: {}", person.greet());
    println!("로봇: {}", robot.greet());
    println!("로봇 두 번: {}", robot.greet_twice());
}

// ----------------------------------------------------------------------------
// 트레이트 바운드
// ----------------------------------------------------------------------------

fn trait_bounds() {
    println!("\n--- 트레이트 바운드 ---");

    // 트레이트를 매개변수로 받기 (정적 디스패치)
    // C++20: template<typename T> requires std::derived_from<T, Summary>
    //        void notify(const T& item);

    // 방법 1: impl Trait 문법 (간단한 경우)
    fn notify_simple(item: &impl Summary) {
        println!("속보! {}", item.summarize());
    }

    // 방법 2: 트레이트 바운드 문법 (복잡한 경우)
    fn notify<T: Summary>(item: &T) {
        println!("속보! {}", item.summarize());
    }

    // 여러 트레이트 요구
    fn notify_display<T: Summary + Display>(item: &T) {
        println!("Display: {}, Summary: {}", item, item.summarize());
    }

    // where 절로 가독성 향상
    fn complex_function<T, U>(t: &T, u: &U) -> String
    where
        T: Summary + Clone,
        U: Debug,
    {
        format!("{} - {:?}", t.summarize(), u)
    }

    let tweet = Tweet {
        username: String::from("user123"),
        content: String::from("테스트"),
        reply: false,
        retweet: false,
    };

    notify_simple(&tweet);
    notify(&tweet);

    // 반환 타입으로 impl Trait
    fn create_summarizable() -> impl Summary {
        Tweet {
            username: String::from("bot"),
            content: String::from("자동 생성"),
            reply: false,
            retweet: false,
        }
    }

    let item = create_summarizable();
    println!("생성된 항목: {}", item.summarize());

    // 주의: impl Trait 반환은 단일 타입만 가능
    // fn random_summarizable() -> impl Summary {
    //     if true {
    //         NewsArticle { ... }  // 에러!
    //     } else {
    //         Tweet { ... }
    //     }
    // }
}

// ----------------------------------------------------------------------------
// 트레이트 객체 (동적 디스패치)
// ----------------------------------------------------------------------------

fn trait_objects() {
    println!("\n--- 트레이트 객체 ---");

    // dyn Trait = 런타임에 어떤 타입인지 결정
    // C++: Summary* 또는 std::unique_ptr<Summary>

    // 정적 디스패치 vs 동적 디스패치:
    // 정적: 컴파일 타임에 어떤 메서드 호출할지 결정 (인라인 가능)
    // 동적: vtable을 통해 런타임에 결정 (약간의 오버헤드)

    let article = NewsArticle {
        headline: String::from("제목"),
        location: String::from("위치"),
        author: String::from("저자"),
        content: String::from("내용"),
    };

    let tweet = Tweet {
        username: String::from("user"),
        content: String::from("내용"),
        reply: false,
        retweet: false,
    };

    // 다양한 타입을 하나의 벡터에 저장 (C++: vector<unique_ptr<Summary>>)
    let items: Vec<Box<dyn Summary>> = vec![Box::new(article), Box::new(tweet)];

    for item in items {
        println!("항목: {}", item.summarize());
    }

    // 트레이트 객체의 제한:
    // 1. 객체 안전(object-safe)한 트레이트만 가능
    // 2. 제네릭 메서드가 있으면 안 됨
    // 3. Self를 반환하면 안 됨 (Clone 등)
}

// ----------------------------------------------------------------------------
// 파생 트레이트 (Derive)
// ----------------------------------------------------------------------------

fn derive_traits() {
    println!("\n--- 파생 트레이트 ---");

    // #[derive]로 표준 트레이트 자동 구현
    // C++: 컴파일러가 생성하는 특수 멤버 함수와 유사

    #[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
    struct Point {
        x: i32,
        y: i32,
    }

    // Debug: {:?} 포맷팅
    let p = Point { x: 10, y: 20 };
    println!("Debug: {:?}", p);

    // Clone: 깊은 복사
    let p2 = p.clone();
    println!("Clone: {:?}", p2);

    // PartialEq, Eq: == 비교
    println!("같음: {}", p == p2);

    // Default: 기본값 생성
    let default_point: Point = Default::default();
    println!("Default: {:?}", default_point);

    // 주요 파생 트레이트:
    // Debug     - 디버그 출력
    // Clone     - 깊은 복사
    // Copy      - 비트 복사 (Clone 필요)
    // PartialEq - 부분 동등성 (== 연산자)
    // Eq        - 완전 동등성 (PartialEq 필요)
    // PartialOrd - 부분 순서 (< > 연산자)
    // Ord       - 완전 순서 (PartialOrd + Eq 필요)
    // Hash      - 해시 가능 (HashMap 키로 사용)
    // Default   - 기본값 생성

    // Copy 예제 - 스택 전용, 비용이 저렴한 복사
    #[derive(Debug, Copy, Clone)]
    struct SmallData {
        a: i32,
        b: i32,
    }

    let s1 = SmallData { a: 1, b: 2 };
    let s2 = s1; // Copy이므로 이동 대신 복사
    println!("s1: {:?}, s2: {:?}", s1, s2); // 둘 다 유효!
}

// ----------------------------------------------------------------------------
// 연산자 오버로딩
// ----------------------------------------------------------------------------

fn operator_overloading() {
    println!("\n--- 연산자 오버로딩 ---");

    // Rust의 연산자 오버로딩은 트레이트로 구현
    // std::ops 모듈의 트레이트들 사용

    // C++:
    // Point operator+(const Point& other) const {
    //     return Point{x + other.x, y + other.y};
    // }

    #[derive(Debug, Clone, Copy)]
    struct Point {
        x: i32,
        y: i32,
    }

    // Add 트레이트 구현
    impl Add for Point {
        type Output = Point; // 연관 타입 (결과 타입)

        fn add(self, other: Point) -> Point {
            Point {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }

    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 3, y: 4 };
    let p3 = p1 + p2; // Add::add(p1, p2) 호출
    println!("{:?} + {:?} = {:?}", p1, p2, p3);

    // 다른 타입과의 연산
    impl Add<i32> for Point {
        type Output = Point;

        fn add(self, scalar: i32) -> Point {
            Point {
                x: self.x + scalar,
                y: self.y + scalar,
            }
        }
    }

    let p4 = p1 + 10;
    println!("{:?} + 10 = {:?}", p1, p4);

    // 주요 연산자 트레이트:
    // Add, Sub, Mul, Div, Rem     - 산술 연산자
    // AddAssign, SubAssign, ...   - 복합 대입 (+=, -= 등)
    // Neg, Not                    - 단항 연산자
    // Index, IndexMut             - [] 연산자
    // Deref, DerefMut             - * 연산자
}

// ----------------------------------------------------------------------------
// 슈퍼트레이트
// ----------------------------------------------------------------------------

fn supertraits() {
    println!("\n--- 슈퍼트레이트 ---");

    // 트레이트가 다른 트레이트에 의존
    // C++의 상속과 유사하지만 구현 상속이 아닌 요구사항

    // Display를 요구하는 트레이트
    trait OutlinePrint: Display {
        fn outline_print(&self) {
            let output = self.to_string();
            let len = output.len();
            println!("{}", "*".repeat(len + 4));
            println!("* {} *", output);
            println!("{}", "*".repeat(len + 4));
        }
    }

    struct Point {
        x: i32,
        y: i32,
    }

    // Display 먼저 구현해야 함
    impl Display for Point {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }

    // 그 다음 OutlinePrint 구현 가능
    impl OutlinePrint for Point {}

    let p = Point { x: 1, y: 2 };
    p.outline_print();
}
