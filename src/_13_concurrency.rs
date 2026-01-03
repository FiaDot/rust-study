// ============================================================================
// 13. 동시성 (Concurrency)
// ============================================================================
// C++20과의 핵심 차이점:
// 1. 컴파일 타임에 데이터 레이스 방지 - Send/Sync 트레이트
// 2. Arc<Mutex<T>> = C++의 shared_ptr<T> + mutex
// 3. 채널(Channel) = C++ 없음 (직접 구현하거나 라이브러리 사용)
// 4. std::thread::spawn은 move 클로저 필수
// 5. Mutex 락은 RAII (C++과 동일)
// ============================================================================

use std::sync::{mpsc, Arc, Mutex, RwLock};
use std::thread;
use std::time::Duration;

pub fn run() {
    println!("\n=== 13. 동시성 ===\n");

    basic_threads();
    move_closures();
    channels();
    shared_state();
    rwlock_example();
    send_sync_traits();
}

// ----------------------------------------------------------------------------
// 기본 스레드
// ----------------------------------------------------------------------------

fn basic_threads() {
    println!("--- 기본 스레드 ---");

    // C++: std::thread t([] { ... });
    let handle = thread::spawn(|| {
        for i in 1..5 {
            println!("  스레드: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..3 {
        println!("메인: {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    // join으로 스레드 종료 대기
    // C++: t.join();
    handle.join().unwrap();

    println!("모든 스레드 완료");
}

// ----------------------------------------------------------------------------
// move 클로저
// ----------------------------------------------------------------------------

fn move_closures() {
    println!("\n--- move 클로저 ---");

    let v = vec![1, 2, 3];

    // move 없이는 컴파일 에러!
    // 스레드가 v보다 오래 살 수 있으므로
    let handle = thread::spawn(move || {
        println!("스레드에서 벡터: {:?}", v);
    });

    // println!("{:?}", v);  // 에러! v는 이동됨

    handle.join().unwrap();

    // C++ 비교:
    // std::vector<int> v = {1, 2, 3};
    // std::thread t([v = std::move(v)] { ... });  // 캡처에 move 필요
}

// ----------------------------------------------------------------------------
// 채널 (Message Passing)
// ----------------------------------------------------------------------------

fn channels() {
    println!("\n--- 채널 ---");

    // mpsc = Multiple Producer, Single Consumer
    // C++에는 없음, Go의 채널과 유사

    // 채널 생성
    let (tx, rx) = mpsc::channel();

    // 송신 스레드
    thread::spawn(move || {
        let val = String::from("안녕하세요");
        tx.send(val).unwrap();
        // println!("{}", val);  // 에러! val은 이동됨
    });

    // 수신 (블로킹)
    let received = rx.recv().unwrap();
    println!("수신: {}", received);

    // 여러 값 전송
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec!["a", "b", "c", "d"];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    // 이터레이터로 수신
    print!("수신: ");
    for received in rx {
        print!("{} ", received);
    }
    println!();

    // 여러 송신자
    let (tx, rx) = mpsc::channel();
    let tx2 = tx.clone();  // 송신자 복제

    thread::spawn(move || {
        tx.send("스레드1").unwrap();
    });

    thread::spawn(move || {
        tx2.send("스레드2").unwrap();
    });

    for _ in 0..2 {
        println!("다중 송신자: {}", rx.recv().unwrap());
    }

    // 비블로킹 수신
    let (tx, rx) = mpsc::channel::<i32>();
    drop(tx);  // 송신자 닫기

    match rx.try_recv() {
        Ok(val) => println!("값: {}", val),
        Err(mpsc::TryRecvError::Empty) => println!("데이터 없음"),
        Err(mpsc::TryRecvError::Disconnected) => println!("채널 닫힘"),
    }
}

// ----------------------------------------------------------------------------
// 공유 상태 (Shared State)
// ----------------------------------------------------------------------------

fn shared_state() {
    println!("\n--- 공유 상태 ---");

    // Mutex - 상호 배제
    // C++: std::mutex + std::lock_guard

    let m = Mutex::new(5);

    {
        // lock()은 MutexGuard 반환 (RAII)
        // C++: std::lock_guard<std::mutex> lock(m);
        let mut num = m.lock().unwrap();
        *num = 6;
        println!("Mutex 값: {}", *num);
    }  // MutexGuard가 drop되면서 자동 unlock

    println!("스코프 후: {:?}", m);

    // 스레드 간 공유 - Arc<Mutex<T>>
    // Arc = Atomic Reference Counted (멀티스레드용 Rc)
    // C++: std::shared_ptr + std::mutex

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("최종 카운터: {}", *counter.lock().unwrap());

    // Mutex 교착 상태 주의
    // C++과 마찬가지로 여러 Mutex 동시 락 시 순서 주의
}

// ----------------------------------------------------------------------------
// RwLock - 읽기/쓰기 락
// ----------------------------------------------------------------------------

fn rwlock_example() {
    println!("\n--- RwLock ---");

    // RwLock - 여러 읽기 또는 하나의 쓰기
    // C++: std::shared_mutex

    let lock = RwLock::new(5);

    // 여러 읽기 동시 가능
    {
        let r1 = lock.read().unwrap();
        let r2 = lock.read().unwrap();
        println!("읽기: {}, {}", *r1, *r2);
    }

    // 쓰기는 독점
    {
        let mut w = lock.write().unwrap();
        *w += 1;
        println!("쓰기 후: {}", *w);
    }

    // 멀티스레드에서 사용
    let data = Arc::new(RwLock::new(vec![1, 2, 3]));
    let mut handles = vec![];

    // 읽기 스레드들
    for i in 0..3 {
        let data = Arc::clone(&data);
        handles.push(thread::spawn(move || {
            let read = data.read().unwrap();
            println!("스레드 {} 읽기: {:?}", i, *read);
        }));
    }

    // 쓰기 스레드
    {
        let data = Arc::clone(&data);
        handles.push(thread::spawn(move || {
            let mut write = data.write().unwrap();
            write.push(4);
            println!("쓰기 스레드: {:?}", *write);
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

// ----------------------------------------------------------------------------
// Send와 Sync 트레이트
// ----------------------------------------------------------------------------

fn send_sync_traits() {
    println!("\n--- Send와 Sync 트레이트 ---");

    // Send: 스레드 간 소유권 이전 가능
    // Sync: 스레드 간 참조 공유 가능

    // 대부분의 타입은 자동으로 Send + Sync
    // Rc<T>는 Send/Sync 아님 (Arc 사용해야 함)
    // RefCell<T>는 Sync 아님 (Mutex 사용해야 함)
    // *const T, *mut T는 Send/Sync 아님

    // 예: Rc를 스레드에 보내려 하면 컴파일 에러
    // let rc = Rc::new(5);
    // thread::spawn(move || {
    //     println!("{}", rc);  // 에러! Rc는 Send가 아님
    // });

    // 마커 트레이트 - 직접 구현할 일은 드묾
    // unsafe impl Send for MyType {}
    // unsafe impl Sync for MyType {}

    println!("컴파일러가 스레드 안전성을 검증합니다!");

    // C++에서는 이런 버그가 런타임에 발생:
    // std::vector<int> v;
    // std::thread t1([&v] { v.push_back(1); });
    // std::thread t2([&v] { v.push_back(2); });
    // // 데이터 레이스! 정의되지 않은 동작

    // Rust에서는 컴파일 에러:
    // let mut v = vec![];
    // thread::spawn(|| v.push(1));  // 에러! &mut 참조를 여러 스레드에서 사용 불가
}
