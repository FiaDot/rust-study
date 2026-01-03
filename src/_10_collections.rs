// ============================================================================
// 10. 컬렉션 (Collections)
// ============================================================================
// C++20과의 핵심 차이점:
// 1. Vec<T> = std::vector<T> - 거의 동일한 성능과 사용법
// 2. String = std::string - UTF-8 강제, 인덱싱 제한
// 3. HashMap = std::unordered_map - 유사하지만 기본 해셔가 다름
// 4. 소유권 규칙이 컬렉션에도 적용 - 더 안전한 이터레이션
// ============================================================================

use std::collections::HashMap;

pub fn run() {
    println!("\n=== 10. 컬렉션 ===\n");

    vectors();
    strings();
    hashmaps();
    other_collections();
}

// ----------------------------------------------------------------------------
// Vec<T> - 가변 길이 배열
// ----------------------------------------------------------------------------

fn vectors() {
    println!("--- Vec<T> ---");

    // 생성
    // C++: std::vector<int> v;
    let mut v: Vec<i32> = Vec::new();

    // vec! 매크로 - C++ initializer_list와 유사
    // C++: std::vector<int> v = {1, 2, 3};
    let mut v = vec![1, 2, 3];
    println!("초기 벡터: {:?}", v);

    // 요소 추가
    // C++: v.push_back(4);
    v.push(4);
    v.push(5);
    println!("push 후: {:?}", v);

    // 요소 접근
    // 인덱스 접근 - 범위 초과 시 panic
    let third = v[2];
    println!("세 번째 요소: {}", third);

    // get - Option 반환, 안전한 접근
    // C++: v.at(2) 또는 범위 체크 후 v[2]
    match v.get(2) {
        Some(value) => println!("get(2): {}", value),
        None => println!("인덱스 초과"),
    }

    match v.get(100) {
        Some(value) => println!("get(100): {}", value),
        None => println!("get(100): 범위 초과"),
    }

    // 이터레이션 - 불변 참조
    // C++: for (const auto& elem : v) { ... }
    print!("불변 이터레이션: ");
    for elem in &v {
        print!("{} ", elem);
    }
    println!();

    // 이터레이션 - 가변 참조
    // C++: for (auto& elem : v) { elem *= 2; }
    for elem in &mut v {
        *elem *= 2;
    }
    println!("두 배 후: {:?}", v);

    // 요소 제거
    // C++: v.pop_back();
    let last = v.pop(); // Option<T> 반환
    println!("pop: {:?}, 벡터: {:?}", last, v);

    // 특정 인덱스 제거
    // C++: v.erase(v.begin() + 1);
    let removed = v.remove(1);
    println!("remove(1): {}, 벡터: {:?}", removed, v);

    // 소유권과 벡터
    let v = vec![String::from("a"), String::from("b")];

    // 인덱스로 접근하면 참조를 얻음
    let first = &v[0];
    println!("첫 번째: {}", first);

    // 소유권을 가져오려면
    let mut v = vec![String::from("a"), String::from("b")];
    let owned = v.remove(0); // 벡터에서 제거하며 소유권 획득
    println!("소유: {}, 벡터: {:?}", owned, v);

    // 다양한 타입 저장 - enum 사용
    #[derive(Debug)]
    enum Cell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        Cell::Int(3),
        Cell::Float(10.5),
        Cell::Text(String::from("hello")),
    ];
    println!("혼합 벡터: {:?}", row);

    // 용량 관리
    // C++: v.capacity(), v.reserve(100)
    let mut v: Vec<i32> = Vec::with_capacity(100);
    println!("용량: {}, 길이: {}", v.capacity(), v.len());
    v.push(1);
    println!("push 후 용량: {}, 길이: {}", v.capacity(), v.len());
}

// ----------------------------------------------------------------------------
// String - UTF-8 문자열
// ----------------------------------------------------------------------------

fn strings() {
    println!("\n--- String ---");

    // Rust의 문자열 타입:
    // - String: 소유, 가변, 힙 할당, UTF-8
    // - &str: 빌림, 불변, 문자열 슬라이스

    // 생성
    let mut s = String::new();
    let s = String::from("안녕하세요");
    let s = "hello".to_string();

    println!("문자열: {}", s);

    // 문자열 추가
    let mut s = String::from("foo");

    // push_str: 문자열 슬라이스 추가
    s.push_str("bar");
    println!("push_str 후: {}", s);

    // push: 단일 문자 추가
    s.push('!');
    println!("push 후: {}", s);

    // + 연산자 (소유권 이동 주의!)
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1은 이동됨!
    // println!("{}", s1);  // 에러!
    println!("연결: {}", s3);

    // format! 매크로 - 소유권 이동 없음
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("format!: {}", s);
    // s1, s2, s3 모두 여전히 유효

    // UTF-8과 인덱싱
    // C++: s[0] 으로 바이트 접근 가능
    // Rust: 인덱싱 불가! UTF-8 문자는 가변 길이

    let hello = String::from("안녕");
    // let c = hello[0];  // 컴파일 에러!

    // 왜 인덱싱이 안 되나?
    // "안녕" = 6바이트 (한글은 3바이트씩)
    // hello[0]이 뭘 반환해야 할지 모호함
    println!("'안녕' 바이트 수: {}", hello.len()); // 6

    // 문자 단위 이터레이션
    print!("문자: ");
    for c in "안녕".chars() {
        print!("{} ", c);
    }
    println!();

    // 바이트 단위 이터레이션
    print!("바이트: ");
    for b in "안녕".bytes() {
        print!("{} ", b);
    }
    println!();

    // 슬라이싱 (바이트 경계 주의!)
    let s = "안녕하세요";
    let slice = &s[0..3]; // "안" (3바이트)
    println!("슬라이스: {}", slice);
    // let bad = &s[0..2];  // panic! 문자 중간을 자름

    // 문자 인덱스로 접근하려면
    let s = "안녕하세요";
    let second_char = s.chars().nth(1);
    println!("두 번째 문자: {:?}", second_char);

    // 유용한 메서드들
    let s = "  hello world  ";
    println!("trim: '{}'", s.trim());
    println!("contains: {}", s.contains("world"));
    println!("replace: {}", s.replace("world", "rust"));

    let s = "hello,world,rust";
    let parts: Vec<&str> = s.split(',').collect();
    println!("split: {:?}", parts);
}

// ----------------------------------------------------------------------------
// HashMap<K, V>
// ----------------------------------------------------------------------------

fn hashmaps() {
    println!("\n--- HashMap ---");

    // C++: std::unordered_map<std::string, int>

    // 생성
    let mut scores: HashMap<String, i32> = HashMap::new();

    // 삽입
    // C++: scores["Blue"] = 10;
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("점수: {:?}", scores);

    // 접근
    let team_name = String::from("Blue");
    let score = scores.get(&team_name); // Option<&V> 반환
    println!("Blue 팀: {:?}", score);

    // copied()로 Option<&i32> -> Option<i32>
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("Blue 팀 점수: {}", score);

    // 이터레이션
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // 소유권 - String 키는 이동됨
    let key = String::from("Red");
    let value = 25;
    scores.insert(key, value);
    // println!("{}", key);  // 에러! key는 이동됨

    // 참조 타입(&str)이면 이동 없음
    let mut map: HashMap<&str, i32> = HashMap::new();
    let key = "Green";
    map.insert(key, 30);
    println!("key still valid: {}", key);

    // 업데이트 패턴
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    // 덮어쓰기
    scores.insert(String::from("Blue"), 25);
    println!("덮어쓰기: {:?}", scores);

    // 없을 때만 삽입 (entry API)
    // C++: if (map.find(key) == map.end()) map[key] = value;
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50); // 이미 있으므로 무시
    println!("or_insert: {:?}", scores);

    // 기존 값 기반 업데이트
    let text = "hello world wonderful world";
    let mut word_count = HashMap::new();

    for word in text.split_whitespace() {
        let count = word_count.entry(word).or_insert(0);
        *count += 1;
    }
    println!("단어 수: {:?}", word_count);

    // 삭제
    scores.remove(&String::from("Blue"));
    println!("삭제 후: {:?}", scores);

    // 해셔 변경
    // 기본: SipHash (DoS 방어, 약간 느림)
    // 빠른 해셔 필요시: fnv 또는 ahash 크레이트 사용
    // use std::hash::BuildHasherDefault;
    // use fnv::FnvHasher;
    // let mut map: HashMap<i32, i32, BuildHasherDefault<FnvHasher>> = ...
}

// ----------------------------------------------------------------------------
// 기타 컬렉션
// ----------------------------------------------------------------------------

fn other_collections() {
    println!("\n--- 기타 컬렉션 ---");

    // VecDeque - 양방향 큐
    // C++: std::deque
    use std::collections::VecDeque;

    let mut deque = VecDeque::new();
    deque.push_back(1);
    deque.push_back(2);
    deque.push_front(0);
    println!("VecDeque: {:?}", deque);
    println!("pop_front: {:?}", deque.pop_front());

    // HashSet - 중복 없는 집합
    // C++: std::unordered_set
    use std::collections::HashSet;

    let mut set = HashSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(2); // 중복 무시
    println!("HashSet: {:?}", set);
    println!("contains(1): {}", set.contains(&1));

    // 집합 연산
    let a: HashSet<i32> = [1, 2, 3].iter().cloned().collect();
    let b: HashSet<i32> = [2, 3, 4].iter().cloned().collect();

    println!("합집합: {:?}", a.union(&b).collect::<Vec<_>>());
    println!("교집합: {:?}", a.intersection(&b).collect::<Vec<_>>());
    println!("차집합: {:?}", a.difference(&b).collect::<Vec<_>>());

    // BTreeMap - 정렬된 맵
    // C++: std::map
    use std::collections::BTreeMap;

    let mut btree = BTreeMap::new();
    btree.insert(3, "c");
    btree.insert(1, "a");
    btree.insert(2, "b");
    println!("BTreeMap (정렬됨): {:?}", btree);

    // BinaryHeap - 우선순위 큐 (최대 힙)
    // C++: std::priority_queue
    use std::collections::BinaryHeap;

    let mut heap = BinaryHeap::new();
    heap.push(3);
    heap.push(1);
    heap.push(4);
    heap.push(1);
    heap.push(5);

    println!("BinaryHeap max: {:?}", heap.peek());
    while let Some(value) = heap.pop() {
        print!("{} ", value); // 5 4 3 1 1
    }
    println!();
}
