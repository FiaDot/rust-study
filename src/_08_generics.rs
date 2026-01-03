// ============================================================================
// 08. 제네릭 (Generics)
// ============================================================================
// C++20과의 핵심 차이점:
// 1. 단형화(Monomorphization) - C++ 템플릿과 동일, 제로 코스트 추상화
// 2. 트레이트 바운드로 제약 - C++ concepts와 유사하지만 더 강제적
// 3. 연관 타입으로 타입 멤버 정의
// 4. const generics로 컴파일 타임 상수 매개변수
// 5. Turbofish ::<>로 타입 명시
// ============================================================================

use std::fmt::Display;

pub fn run() {
    println!("\n=== 08. 제네릭 ===\n");

    generic_functions();
    generic_structs();
    generic_enums();
    generic_methods();
    associated_types();
    const_generics();
    phantom_data();
}

// ----------------------------------------------------------------------------
// 제네릭 함수
// ----------------------------------------------------------------------------

fn generic_functions() {
    println!("--- 제네릭 함수 ---");

    // C++ 템플릿:
    // template<typename T>
    // T largest(const std::vector<T>& list) {
    //     return *std::max_element(list.begin(), list.end());
    // }

    // Rust: 트레이트 바운드 필수
    fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
        let mut largest = list[0];
        for &item in list {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    let numbers = vec![34, 50, 25, 100, 65];
    println!("가장 큰 수: {}", largest(&numbers));

    let chars = vec!['y', 'm', 'a', 'q'];
    println!("가장 큰 문자: {}", largest(&chars));

    // 여러 타입 매개변수
    fn pair<T, U>(a: T, b: U) -> (T, U) {
        (a, b)
    }

    let p = pair(1, "hello");
    println!("쌍: {:?}", p);

    // 터보피시(Turbofish) - 타입 명시
    // C++: function<int>() 대신 function::<int>()
    let parsed = "42".parse::<i32>().unwrap();
    println!("파싱됨: {}", parsed);

    let collected: Vec<i32> = (0..5).collect();
    // 또는
    let collected = (0..5).collect::<Vec<i32>>();
    println!("수집됨: {:?}", collected);
}

// ----------------------------------------------------------------------------
// 제네릭 구조체
// ----------------------------------------------------------------------------

fn generic_structs() {
    println!("\n--- 제네릭 구조체 ---");

    // C++: template<typename T> struct Point { T x, y; };
    #[derive(Debug)]
    struct Point<T> {
        x: T,
        y: T,
    }

    let int_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.0, y: 4.0 };
    println!("정수 점: {:?}", int_point);
    println!("실수 점: {:?}", float_point);

    // 다른 타입의 x, y
    #[derive(Debug)]
    struct MixedPoint<T, U> {
        x: T,
        y: U,
    }

    let mixed = MixedPoint { x: 5, y: 4.0 };
    println!("혼합 점: {:?}", mixed);
}

// ----------------------------------------------------------------------------
// 제네릭 열거형
// ----------------------------------------------------------------------------

fn generic_enums() {
    println!("\n--- 제네릭 열거형 ---");

    // 표준 라이브러리의 Option과 Result가 대표적 예

    // enum Option<T> {
    //     Some(T),
    //     None,
    // }

    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }

    // 커스텀 제네릭 열거형
    #[derive(Debug)]
    enum BinaryTree<T> {
        Leaf(T),
        Node {
            left: Box<BinaryTree<T>>,
            right: Box<BinaryTree<T>>,
            value: T,
        },
    }

    let tree = BinaryTree::Node {
        value: 5,
        left: Box::new(BinaryTree::Leaf(3)),
        right: Box::new(BinaryTree::Leaf(7)),
    };
    println!("트리: {:?}", tree);
}

// ----------------------------------------------------------------------------
// 제네릭 메서드
// ----------------------------------------------------------------------------

fn generic_methods() {
    println!("\n--- 제네릭 메서드 ---");

    #[derive(Debug)]
    struct Point<T> {
        x: T,
        y: T,
    }

    // 모든 T에 대한 메서드
    impl<T> Point<T> {
        fn x(&self) -> &T {
            &self.x
        }

        fn y(&self) -> &T {
            &self.y
        }
    }

    // 특정 타입에만 적용되는 메서드
    impl Point<f64> {
        fn distance_from_origin(&self) -> f64 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }

    // 트레이트 바운드가 있는 메서드
    impl<T: Display> Point<T> {
        fn print(&self) {
            println!("Point: ({}, {})", self.x, self.y);
        }
    }

    let p1 = Point { x: 5, y: 10 };
    let p2 = Point { x: 5.0, y: 10.0 };

    println!("p1.x = {}", p1.x());
    // p1.distance_from_origin();  // 에러! i32에는 없음
    println!("p2 원점 거리: {}", p2.distance_from_origin());
    p1.print();
    p2.print();

    // 메서드에 추가 타입 매개변수
    #[derive(Debug)]
    struct Wrapper<T> {
        value: T,
    }

    impl<T> Wrapper<T> {
        // 메서드 자체에 새 타입 매개변수
        fn mixup<U>(self, other: Wrapper<U>) -> Wrapper<(T, U)> {
            Wrapper {
                value: (self.value, other.value),
            }
        }
    }

    let w1 = Wrapper { value: "hello" };
    let w2 = Wrapper { value: 42 };
    let mixed = w1.mixup(w2);
    println!("혼합: {:?}", mixed);
}

// ----------------------------------------------------------------------------
// 연관 타입
// ----------------------------------------------------------------------------

fn associated_types() {
    println!("\n--- 연관 타입 ---");

    // 연관 타입 = 트레이트 내의 타입 별칭
    // 제네릭 매개변수와 비슷하지만 구현 시 결정

    // 표준 Iterator 트레이트 예:
    // trait Iterator {
    //     type Item;  // 연관 타입
    //     fn next(&mut self) -> Option<Self::Item>;
    // }

    // 제네릭 vs 연관 타입:
    // 제네릭: trait Container<T> { ... }
    //   - 같은 타입에 여러 구현 가능
    //   - 사용 시 타입 지정 필요: Container<i32>
    //
    // 연관 타입: trait Container { type Item; ... }
    //   - 타입당 하나의 구현
    //   - 사용 시 타입 지정 불필요

    struct Counter {
        count: u32,
    }

    impl Counter {
        fn new() -> Counter {
            Counter { count: 0 }
        }
    }

    impl Iterator for Counter {
        type Item = u32; // 연관 타입 지정

        fn next(&mut self) -> Option<Self::Item> {
            if self.count < 5 {
                self.count += 1;
                Some(self.count)
            } else {
                None
            }
        }
    }

    let mut counter = Counter::new();
    while let Some(n) = counter.next() {
        print!("{} ", n);
    }
    println!();
}

// ----------------------------------------------------------------------------
// Const Generics (컴파일 타임 상수 매개변수)
// ----------------------------------------------------------------------------

fn const_generics() {
    println!("\n--- Const Generics ---");

    // C++: template<typename T, size_t N>
    //      struct Array { T data[N]; };

    // Rust 1.51+에서 안정화
    #[derive(Debug)]
    struct Array<T, const N: usize> {
        data: [T; N],
    }

    impl<T, const N: usize> Array<T, N> {
        fn len(&self) -> usize {
            N
        }
    }

    impl<T: Default + Copy, const N: usize> Array<T, N> {
        fn new() -> Self {
            Array {
                data: [T::default(); N],
            }
        }
    }

    let arr: Array<i32, 5> = Array::new();
    println!("배열 길이: {}", arr.len());
    println!("배열: {:?}", arr);

    // 배열 비교 - 같은 크기만 비교 가능
    fn compare_arrays<T: PartialEq, const N: usize>(a: &[T; N], b: &[T; N]) -> bool {
        a == b
    }

    let a1 = [1, 2, 3];
    let a2 = [1, 2, 3];
    let a3 = [1, 2, 4];
    // let a4 = [1, 2, 3, 4];  // 크기가 다르면 비교 불가

    println!("a1 == a2: {}", compare_arrays(&a1, &a2));
    println!("a1 == a3: {}", compare_arrays(&a1, &a3));
}

// ----------------------------------------------------------------------------
// PhantomData - 컴파일러 힌트용 타입
// ----------------------------------------------------------------------------

fn phantom_data() {
    println!("\n--- PhantomData ---");

    use std::marker::PhantomData;

    // 타입 매개변수를 사용하지 않지만 "소유"하는 것처럼 표시
    // 주로 안전한 API 설계, 수명 추적에 사용

    // 예: 타입 레벨에서 단위 구분
    struct Meters;
    struct Kilometers;

    struct Distance<Unit> {
        value: f64,
        _unit: PhantomData<Unit>,
    }

    impl<Unit> Distance<Unit> {
        fn new(value: f64) -> Self {
            Distance {
                value,
                _unit: PhantomData,
            }
        }
    }

    let meters: Distance<Meters> = Distance::new(100.0);
    let kilometers: Distance<Kilometers> = Distance::new(1.5);

    // 다른 단위끼리 실수로 연산하는 것을 방지
    // let total = meters.value + kilometers.value;  // 논리적 버그!

    println!("거리: {} 미터, {} 킬로미터", meters.value, kilometers.value);

    // PhantomData<T>는 T를 "소유"하는 것처럼 행동
    // - Send/Sync 트레이트 전파
    // - Drop 검사에 영향
    // - 수명 매개변수 연결
}
