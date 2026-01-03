// ============================================================================
// Rust 학습 - C++20 개발자를 위한 가이드
// ============================================================================
// 이 프로젝트는 C++20 개발자가 Rust의 문법과 idiom을 빠르게 익힐 수 있도록
// 설계된 예제 모음입니다.
//
// 각 모듈은 C++ 코드와 비교하며 Rust의 핵심 개념을 설명합니다.
// 실행: cargo run
// 특정 모듈만 실행하려면 main() 함수에서 원하는 모듈만 호출하세요.
// ============================================================================

// 모듈 선언 - 각 파일이 하나의 모듈
mod _01_basics;
mod _02_ownership;
mod _03_borrowing;
mod _04_lifetimes;
mod _05_structs;
mod _06_enums;
mod _07_traits;
mod _08_generics;
mod _09_error_handling;
mod _10_collections;
mod _11_iterators;
mod _12_smart_pointers;
mod _13_concurrency;
mod _14_modules;

fn main() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║     Rust 학습 가이드 - C++20 개발자를 위한 예제 모음         ║");
    println!("╚══════════════════════════════════════════════════════════════╝");

    // 각 모듈 실행 - 필요한 것만 주석 해제하여 실행
    _01_basics::run();
    _02_ownership::run();
    _03_borrowing::run();
    _04_lifetimes::run();
    _05_structs::run();
    _06_enums::run();
    _07_traits::run();
    _08_generics::run();
    _09_error_handling::run();
    _10_collections::run();
    _11_iterators::run();
    _12_smart_pointers::run();
    _13_concurrency::run();
    _14_modules::run();

    println!("\n╔══════════════════════════════════════════════════════════════╗");
    println!("║                    모든 예제 실행 완료!                       ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
}
