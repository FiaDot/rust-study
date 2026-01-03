// ============================================================================
// 14. 모듈 시스템 (Module System)
// ============================================================================
// C++20과의 핵심 차이점:
// 1. 헤더 파일 없음 - 모듈이 인터페이스와 구현을 함께 관리
// 2. pub 키워드로 가시성 제어 (C++의 public/private)
// 3. use로 경로 단축 (C++의 using namespace와 유사하지만 더 세밀)
// 4. Cargo.toml로 의존성 관리 (C++에는 표준 패키지 관리자 없음)
// 5. mod.rs 또는 파일명으로 모듈 선언 (C++20 모듈과 유사)
// ============================================================================

pub fn run() {
    println!("\n=== 14. 모듈 시스템 ===\n");

    module_basics();
    visibility_rules();
    use_keyword();
    module_file_structure();
}

// ----------------------------------------------------------------------------
// 모듈 기초
// ----------------------------------------------------------------------------

fn module_basics() {
    println!("--- 모듈 기초 ---");

    // 모듈은 코드를 그룹화하고 캡슐화
    // C++: namespace와 유사하지만 가시성 규칙이 다름

    // 인라인 모듈 정의
    mod front_of_house {
        // 기본적으로 private
        pub mod hosting {
            pub fn add_to_waitlist() {
                println!("대기 명단에 추가");
            }

            fn seat_at_table() {
                println!("테이블 배정");
            }
        }

        mod serving {
            fn take_order() {}
            fn serve_order() {}
            fn take_payment() {}
        }
    }

    // 상대 경로로 호출 (같은 함수 내 모듈이므로)
    front_of_house::hosting::add_to_waitlist();

    // 한 번 더 호출해서 확인
    front_of_house::hosting::add_to_waitlist();

    // 비공개 함수는 접근 불가
    // front_of_house::hosting::seat_at_table();  // 에러!
    // front_of_house::serving::take_order();     // 에러! serving 모듈이 비공개
}

// ----------------------------------------------------------------------------
// 가시성 규칙
// ----------------------------------------------------------------------------

fn visibility_rules() {
    println!("\n--- 가시성 규칙 ---");

    mod outer {
        pub mod inner {
            pub fn public_function() {
                println!("공개 함수");
                private_function();  // 같은 모듈 내에서는 접근 가능
            }

            fn private_function() {
                println!("비공개 함수");
            }

            // 구조체의 필드는 별도로 pub 지정 필요
            pub struct Breakfast {
                pub toast: String,      // 공개
                seasonal_fruit: String, // 비공개
            }

            impl Breakfast {
                // 생성자 패턴 - 비공개 필드가 있으면 필수
                pub fn summer(toast: &str) -> Breakfast {
                    Breakfast {
                        toast: String::from(toast),
                        seasonal_fruit: String::from("복숭아"),
                    }
                }
            }
        }

        // 부모 모듈은 자식의 비공개 항목 접근 불가
        pub fn demo() {
            inner::public_function();
            // inner::private_function();  // 에러!
        }
    }

    outer::inner::public_function();

    let mut meal = outer::inner::Breakfast::summer("호밀");
    meal.toast = String::from("밀");  // 공개 필드 수정 가능
    // meal.seasonal_fruit = String::from("블루베리");  // 에러! 비공개

    // 열거형은 pub이면 모든 variant가 공개
    mod menu {
        pub enum Appetizer {
            Soup,      // 자동으로 공개
            Salad,     // 자동으로 공개
        }
    }

    let _order1 = menu::Appetizer::Soup;
    let _order2 = menu::Appetizer::Salad;

    // pub(crate), pub(super), pub(in path) - 세밀한 가시성 제어
    mod levels {
        pub(crate) fn crate_visible() {}      // 크레이트 내에서만
        pub(super) fn parent_visible() {}     // 부모 모듈에서만
        // pub(in crate::levels) fn specific() {}  // 특정 경로에서만
    }

    levels::crate_visible();  // 같은 크레이트이므로 OK
}

// ----------------------------------------------------------------------------
// use 키워드
// ----------------------------------------------------------------------------

fn use_keyword() {
    println!("\n--- use 키워드 ---");

    // use로 경로 단축
    // C++: using namespace와 유사

    mod shapes {
        pub mod circle {
            pub fn area(r: f64) -> f64 {
                std::f64::consts::PI * r * r
            }
        }

        pub mod rectangle {
            pub fn area(w: f64, h: f64) -> f64 {
                w * h
            }
        }
    }

    // 모듈 가져오기 (관용적)
    use shapes::circle;
    println!("원 넓이: {}", circle::area(5.0));

    // 함수 직접 가져오기 (비추천 - 출처 불명확)
    use shapes::rectangle::area as rect_area;  // 별칭으로 충돌 방지
    println!("사각형 넓이: {}", rect_area(4.0, 5.0));

    // 여러 항목 한 번에
    use std::collections::{HashMap, HashSet};
    let _map: HashMap<i32, i32> = HashMap::new();
    let _set: HashSet<i32> = HashSet::new();

    // 중첩 경로
    // use std::io;
    // use std::io::Write;
    // 대신:
    use std::io::{self, Write};

    // 글롭 연산자 (*) - 모든 공개 항목 가져오기
    // use std::collections::*;  // 테스트나 prelude에서 주로 사용

    // 재내보내기 (re-export)
    mod internal {
        pub fn helper() {}
    }
    pub use internal::helper;  // 외부에서 internal::helper 대신 helper로 접근

    // Prelude 패턴 예시 (실제 프로젝트에서는 루트 레벨에 정의)
    // mod prelude {
    //     pub use super::shapes::circle;
    //     pub use super::shapes::rectangle;
    // }
    // 사용자는 prelude만 가져오면 됨
    // use prelude::*;

    // shapes 모듈 직접 사용
    let _ = shapes::rectangle::area(3.0, 4.0);
}

// ----------------------------------------------------------------------------
// 모듈 파일 구조
// ----------------------------------------------------------------------------

fn module_file_structure() {
    println!("\n--- 모듈 파일 구조 ---");

    // 파일 시스템과 모듈 매핑
    //
    // 방법 1: 단일 파일
    // src/
    // ├── main.rs (또는 lib.rs)
    // └── garden.rs         // mod garden; 으로 선언
    //
    // 방법 2: 디렉터리 (구버전)
    // src/
    // ├── main.rs
    // └── garden/
    //     ├── mod.rs        // garden 모듈 정의
    //     └── vegetables.rs // garden::vegetables 서브모듈
    //
    // 방법 3: 디렉터리 (신버전, 권장)
    // src/
    // ├── main.rs
    // ├── garden.rs         // garden 모듈 정의
    // └── garden/
    //     └── vegetables.rs // garden::vegetables 서브모듈

    // main.rs 또는 lib.rs 예:
    // mod garden;  // garden.rs 또는 garden/mod.rs 로드
    //
    // use garden::vegetables;  // 서브모듈 사용

    // garden.rs 예:
    // pub mod vegetables;  // garden/vegetables.rs 로드
    //
    // pub fn plant() { ... }

    // vegetables.rs 예:
    // pub fn grow() { ... }

    println!("현재 프로젝트 구조:");
    println!("  src/");
    println!("  ├── main.rs");
    println!("  ├── 01_basics.rs");
    println!("  ├── 02_ownership.rs");
    println!("  └── ... (각 모듈 파일)");

    // Cargo.toml로 외부 의존성 관리
    // [dependencies]
    // serde = "1.0"
    // tokio = {{ version = "1", features = ["full"] }}

    // 외부 크레이트 사용
    // use serde::{{Serialize, Deserialize}};
    //
    // #[derive(Serialize, Deserialize)]
    // struct Config {{ ... }}

    // 워크스페이스 - 여러 패키지 관리
    // workspace/
    // ├── Cargo.toml     // [workspace] members = ["lib1", "app"]
    // ├── lib1/
    // │   └── Cargo.toml
    // └── app/
    //     └── Cargo.toml  // [dependencies] lib1 = {{ path = "../lib1" }}
}

// C++ 모듈(C++20)과의 비교:
//
// C++20:
// // math.ixx
// export module math;
// export int add(int a, int b) { return a + b; }
//
// // main.cpp
// import math;
// int main() { return add(1, 2); }
//
// Rust:
// // math.rs
// pub fn add(a: i32, b: i32) -> i32 { a + b }
//
// // main.rs
// mod math;
// fn main() { math::add(1, 2); }
//
// 주요 차이:
// - Rust는 별도의 모듈 인터페이스 파일 불필요
// - Rust는 기본적으로 private, C++20 모듈은 export 명시
// - Rust는 Cargo로 빌드/의존성 통합 관리
