// ============================================================================
// 11. 이터레이터와 클로저 (Iterators and Closures)
// ============================================================================
// C++20과의 핵심 차이점:
// 1. 이터레이터 = C++20 ranges와 매우 유사 (지연 평가)
// 2. 클로저가 환경 캡처하는 방식이 명시적 (move, &, &mut)
// 3. Fn, FnMut, FnOnce 트레이트로 클로저 타입 구분
// 4. 제로 코스트 추상화 - 수동 루프와 동일한 성능
// ============================================================================

pub fn run() {
    println!("\n=== 11. 이터레이터와 클로저 ===\n");

    closures_basics();
    closure_traits();
    iterator_basics();
    iterator_adaptors();
    iterator_consumers();
    custom_iterator();
}

// ----------------------------------------------------------------------------
// 클로저 기초
// ----------------------------------------------------------------------------

fn closures_basics() {
    println!("--- 클로저 기초 ---");

    // 클로저 = 익명 함수, 환경 캡처 가능
    // C++: [captures](params) { body }
    // Rust: |params| body

    // 기본 클로저
    let add_one = |x: i32| x + 1;
    println!("add_one(5) = {}", add_one(5));

    // 타입 추론 - 첫 사용에서 결정
    let add = |a, b| a + b;
    println!("add(2, 3) = {}", add(2, 3));

    // 블록 본문
    let complex = |x| {
        let y = x * 2;
        y + 1
    };
    println!("complex(5) = {}", complex(5));

    // 환경 캡처
    let x = 4;
    let equal_to_x = |z| z == x;  // x를 캡처
    println!("equal_to_x(4) = {}", equal_to_x(4));

    // C++ 비교:
    // auto equal_to_x = [x](int z) { return z == x; };

    // 캡처 방식 - Rust는 자동으로 최소 권한 캡처
    // 1. 불변 참조 (&T) - 기본
    // 2. 가변 참조 (&mut T) - 수정 필요시
    // 3. 소유권 (T) - 이동 필요시

    // 불변 참조 캡처
    let list = vec![1, 2, 3];
    let print_list = || println!("리스트: {:?}", list);
    print_list();
    println!("여전히 사용 가능: {:?}", list);

    // 가변 참조 캡처
    let mut count = 0;
    let mut increment = || {
        count += 1;  // &mut count 캡처
        println!("count = {}", count);
    };
    increment();
    increment();
    // println!("{}", count);  // increment가 살아있는 동안 에러
    drop(increment);  // 명시적 drop으로 빌림 해제
    println!("최종 count: {}", count);

    // move 키워드 - 소유권 강제 이동
    let data = vec![1, 2, 3];
    let owns_data = move || {
        println!("내 데이터: {:?}", data);
    };
    owns_data();
    // println!("{:?}", data);  // 에러! data는 이동됨

    // 스레드에 데이터 전달할 때 move 필수
    // std::thread::spawn(move || { ... });
}

// ----------------------------------------------------------------------------
// 클로저 트레이트
// ----------------------------------------------------------------------------

fn closure_traits() {
    println!("\n--- 클로저 트레이트 ---");

    // Rust 클로저는 세 가지 트레이트 중 하나 이상 구현:
    // - FnOnce: 한 번만 호출 가능 (소유권 이동)
    // - FnMut: 여러 번 호출, 가변 참조
    // - Fn: 여러 번 호출, 불변 참조

    // FnOnce 예제 - 캡처한 값을 소비
    fn call_once<F>(f: F)
    where
        F: FnOnce() -> String,
    {
        println!("결과: {}", f());
    }

    let s = String::from("hello");
    let consume = move || s;  // s를 반환하며 소비
    call_once(consume);
    // call_once(consume);  // 에러! 이미 소비됨

    // FnMut 예제 - 가변 상태 수정
    fn call_mut_twice<F>(mut f: F)
    where
        F: FnMut(),
    {
        f();
        f();
    }

    let mut total = 0;
    let mut add_ten = || total += 10;
    call_mut_twice(&mut add_ten);
    println!("total after FnMut: {}", total);

    // Fn 예제 - 순수 함수처럼 동작
    fn call_many_times<F>(f: F)
    where
        F: Fn() -> i32,
    {
        for _ in 0..3 {
            println!("호출: {}", f());
        }
    }

    let x = 10;
    let get_x = || x;  // x를 불변 참조로 캡처
    call_many_times(get_x);

    // 트레이트 계층:
    // Fn : FnMut : FnOnce
    // Fn을 구현하면 FnMut과 FnOnce도 자동 구현
}

// ----------------------------------------------------------------------------
// 이터레이터 기초
// ----------------------------------------------------------------------------

fn iterator_basics() {
    println!("\n--- 이터레이터 기초 ---");

    // Iterator 트레이트:
    // trait Iterator {
    //     type Item;
    //     fn next(&mut self) -> Option<Self::Item>;
    // }

    let v = vec![1, 2, 3];

    // iter() - 불변 참조 이터레이터
    let mut iter = v.iter();
    println!("next: {:?}", iter.next());  // Some(&1)
    println!("next: {:?}", iter.next());  // Some(&2)
    println!("next: {:?}", iter.next());  // Some(&3)
    println!("next: {:?}", iter.next());  // None

    // 세 가지 이터레이터 메서드:
    // iter()      - &T 이터레이터
    // iter_mut()  - &mut T 이터레이터
    // into_iter() - T 이터레이터 (소유권 이동)

    // C++ 비교:
    // iter()      ~ cbegin()/cend()
    // iter_mut()  ~ begin()/end()
    // into_iter() ~ std::make_move_iterator()

    let v = vec![1, 2, 3];

    // for 루프는 into_iter() 호출
    for val in v {  // v.into_iter()
        println!("소유: {}", val);
    }
    // println!("{:?}", v);  // 에러! v는 이동됨

    let v = vec![1, 2, 3];
    for val in &v {  // (&v).into_iter() = v.iter()
        println!("참조: {}", val);
    }
    println!("여전히 사용 가능: {:?}", v);

    // 범위 이터레이터
    for i in 0..5 {
        print!("{} ", i);
    }
    println!();
}

// ----------------------------------------------------------------------------
// 이터레이터 어댑터
// ----------------------------------------------------------------------------

fn iterator_adaptors() {
    println!("\n--- 이터레이터 어댑터 ---");

    // 어댑터 = 이터레이터를 다른 이터레이터로 변환
    // 지연 평가 (lazy) - 소비될 때까지 실행 안 됨
    // C++20 views와 유사

    let v = vec![1, 2, 3, 4, 5];

    // map - 각 요소 변환
    // C++: v | std::views::transform(func)
    let doubled: Vec<i32> = v.iter().map(|x| x * 2).collect();
    println!("map: {:?}", doubled);

    // filter - 조건에 맞는 요소만
    // C++: v | std::views::filter(pred)
    let evens: Vec<&i32> = v.iter().filter(|x| *x % 2 == 0).collect();
    println!("filter: {:?}", evens);

    // 체이닝
    let result: Vec<i32> = v
        .iter()
        .filter(|x| *x % 2 == 1)  // 홀수만
        .map(|x| x * x)            // 제곱
        .collect();
    println!("체이닝: {:?}", result);

    // enumerate - 인덱스 추가
    // C++: boost::adaptors::indexed 또는 수동
    for (i, val) in v.iter().enumerate() {
        println!("  [{}] = {}", i, val);
    }

    // zip - 두 이터레이터 결합
    // C++: std::views::zip (C++23)
    let a = [1, 2, 3];
    let b = ["a", "b", "c"];
    let zipped: Vec<_> = a.iter().zip(b.iter()).collect();
    println!("zip: {:?}", zipped);

    // take, skip
    let first_three: Vec<_> = (0..10).take(3).collect();
    let skip_five: Vec<_> = (0..10).skip(5).collect();
    println!("take(3): {:?}", first_three);
    println!("skip(5): {:?}", skip_five);

    // take_while, skip_while
    let before_five: Vec<_> = (0..10).take_while(|x| *x < 5).collect();
    println!("take_while(<5): {:?}", before_five);

    // flatten - 중첩 이터레이터 평탄화
    // C++: std::views::join
    let nested = vec![vec![1, 2], vec![3, 4], vec![5]];
    let flat: Vec<_> = nested.into_iter().flatten().collect();
    println!("flatten: {:?}", flat);

    // flat_map = map + flatten
    let words = vec!["hello", "world"];
    let chars: Vec<_> = words.iter().flat_map(|s| s.chars()).collect();
    println!("flat_map: {:?}", chars);

    // chain - 이터레이터 연결
    let combined: Vec<_> = (1..4).chain(10..13).collect();
    println!("chain: {:?}", combined);

    // peekable - 다음 요소 미리보기
    let mut iter = [1, 2, 3].iter().peekable();
    println!("peek: {:?}", iter.peek());
    println!("next: {:?}", iter.next());
    println!("peek: {:?}", iter.peek());
}

// ----------------------------------------------------------------------------
// 이터레이터 소비자
// ----------------------------------------------------------------------------

fn iterator_consumers() {
    println!("\n--- 이터레이터 소비자 ---");

    // 소비자 = 이터레이터를 소비하여 결과 생성
    // 호출하면 실제로 이터레이션 수행

    let v = vec![1, 2, 3, 4, 5];

    // collect - 컬렉션으로 수집
    let squared: Vec<i32> = v.iter().map(|x| x * x).collect();
    println!("collect: {:?}", squared);

    // sum, product
    let sum: i32 = v.iter().sum();
    let product: i32 = v.iter().product();
    println!("sum: {}, product: {}", sum, product);

    // fold - 누적 연산 (C++ std::accumulate)
    let sum = v.iter().fold(0, |acc, x| acc + x);
    let concat = v.iter().fold(String::new(), |acc, x| acc + &x.to_string());
    println!("fold sum: {}, concat: {}", sum, concat);

    // reduce - fold와 비슷하지만 초기값 없음
    let max = v.iter().copied().reduce(|a, b| if a > b { a } else { b });
    println!("reduce max: {:?}", max);

    // find - 조건에 맞는 첫 요소
    let first_even = v.iter().find(|x| *x % 2 == 0);
    println!("find even: {:?}", first_even);

    // position - 조건에 맞는 첫 인덱스
    let pos = v.iter().position(|x| *x == 3);
    println!("position of 3: {:?}", pos);

    // any, all
    let has_even = v.iter().any(|x| x % 2 == 0);
    let all_positive = v.iter().all(|x| *x > 0);
    println!("any even: {}, all positive: {}", has_even, all_positive);

    // count
    let count = v.iter().filter(|x| *x % 2 == 0).count();
    println!("even count: {}", count);

    // min, max
    println!("min: {:?}, max: {:?}", v.iter().min(), v.iter().max());

    // min_by, max_by - 커스텀 비교
    let strings = vec!["hello", "hi", "hey"];
    let shortest = strings.iter().min_by_key(|s| s.len());
    println!("shortest: {:?}", shortest);

    // partition - 두 그룹으로 분리
    let (evens, odds): (Vec<&i32>, Vec<&i32>) = v.iter().partition(|x| *x % 2 == 0);
    println!("evens: {:?}, odds: {:?}", evens, odds);

    // for_each - 부작용 수행
    print!("for_each: ");
    v.iter().for_each(|x| print!("{} ", x));
    println!();
}

// ----------------------------------------------------------------------------
// 커스텀 이터레이터
// ----------------------------------------------------------------------------

fn custom_iterator() {
    println!("\n--- 커스텀 이터레이터 ---");

    // Iterator 트레이트 구현
    struct Counter {
        count: u32,
        max: u32,
    }

    impl Counter {
        fn new(max: u32) -> Counter {
            Counter { count: 0, max }
        }
    }

    impl Iterator for Counter {
        type Item = u32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count < self.max {
                self.count += 1;
                Some(self.count)
            } else {
                None
            }
        }
    }

    // 사용
    let counter = Counter::new(5);
    println!("Counter: {:?}", counter.collect::<Vec<_>>());

    // 어댑터도 사용 가능
    let sum: u32 = Counter::new(5)
        .zip(Counter::new(5).skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    println!("복잡한 계산: {}", sum);

    // IntoIterator 트레이트 - for 루프 지원
    struct Range {
        start: i32,
        end: i32,
    }

    impl IntoIterator for Range {
        type Item = i32;
        type IntoIter = std::ops::Range<i32>;

        fn into_iter(self) -> Self::IntoIter {
            self.start..self.end
        }
    }

    let range = Range { start: 1, end: 5 };
    for i in range {
        print!("{} ", i);
    }
    println!();
}
