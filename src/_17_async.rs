// ============================================================================
// 17. 비동기 프로그래밍 (Async/Await)
// ============================================================================
// C++20과의 핵심 차이점:
// 1. Rust의 Future는 lazy - poll될 때만 실행 (C++ coroutine도 유사)
// 2. 런타임이 언어에 포함되지 않음 - tokio, async-std 등 선택
// 3. async fn은 impl Future를 반환
// 4. .await는 Future가 완료될 때까지 현재 태스크를 양보
// 5. Send 바운드로 스레드 간 이동 가능 여부 결정
// ============================================================================

// tokio 런타임 사용
use std::time::Duration;
use tokio::time::sleep;

pub fn run() {
    println!("\n=== 17. 비동기 프로그래밍 ===\n");

    // 비동기 코드 실행을 위해 tokio 런타임 생성
    let rt = tokio::runtime::Runtime::new().unwrap();

    rt.block_on(async {
        async_basics().await;
        futures_explained().await;
        concurrent_tasks().await;
        channels_async().await;
        select_example().await;
        error_handling_async().await;
    });

    sync_vs_async_comparison();
}

// ----------------------------------------------------------------------------
// Async 기초
// ----------------------------------------------------------------------------

// async fn은 Future를 반환하는 함수
async fn say_hello() {
    println!("안녕하세요!");
}

async fn delayed_message(msg: &str, delay_ms: u64) {
    sleep(Duration::from_millis(delay_ms)).await;
    println!("{}", msg);
}

async fn add_async(a: i32, b: i32) -> i32 {
    // 비동기 계산 (여기서는 단순 예시)
    sleep(Duration::from_millis(10)).await;
    a + b
}

async fn async_basics() {
    println!("--- Async 기초 ---");

    // async 함수 호출 - Future 반환
    let future = say_hello();

    // .await로 Future 실행
    future.await;

    // 인자와 반환값이 있는 async 함수
    let result = add_async(5, 3).await;
    println!("5 + 3 = {}", result);

    // async 블록 - 익명 Future 생성
    let value = async {
        let a = add_async(1, 2).await;
        let b = add_async(3, 4).await;
        a + b
    }
    .await;

    println!("(1+2) + (3+4) = {}", value);

    // C++20 코루틴과 비교:
    // C++:
    // task<int> add_async(int a, int b) {
    //     co_await some_delay();
    //     co_return a + b;
    // }
    //
    // Rust:
    // async fn add_async(a: i32, b: i32) -> i32 {
    //     some_delay().await;
    //     a + b
    // }
}

// ----------------------------------------------------------------------------
// Future 설명
// ----------------------------------------------------------------------------

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

// 커스텀 Future 구현
struct CountdownFuture {
    count: u32,
}

impl Future for CountdownFuture {
    type Output = String;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.count == 0 {
            Poll::Ready(String::from("발사!"))
        } else {
            println!("카운트다운: {}", self.count);
            self.count -= 1;
            // 즉시 다시 poll하도록 waker 호출
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

async fn futures_explained() {
    println!("\n--- Future 설명 ---");

    // Future 트레이트:
    // trait Future {
    //     type Output;
    //     fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
    // }

    // Poll 열거형:
    // enum Poll<T> {
    //     Ready(T),    // 완료됨
    //     Pending,     // 아직 완료 안 됨
    // }

    println!("Future는 poll될 때만 진행됩니다 (lazy)");

    // 커스텀 Future 실행
    let countdown = CountdownFuture { count: 3 };
    let result = countdown.await;
    println!("결과: {}", result);

    // async/await는 컴파일러가 상태 머신으로 변환
    // 각 .await 지점이 상태 전환점

    println!("\n비동기의 핵심:");
    println!("1. Future 생성 (실행 X)");
    println!("2. 런타임이 poll 호출");
    println!("3. Pending이면 나중에 다시 poll");
    println!("4. Ready면 결과 반환");
}

// ----------------------------------------------------------------------------
// 동시 태스크
// ----------------------------------------------------------------------------

async fn fetch_data(id: u32) -> String {
    println!("데이터 {} 요청 시작", id);
    sleep(Duration::from_millis(100)).await;
    println!("데이터 {} 요청 완료", id);
    format!("데이터_{}", id)
}

async fn concurrent_tasks() {
    println!("\n--- 동시 태스크 ---");

    // 순차 실행 - 총 300ms
    println!("순차 실행:");
    let start = std::time::Instant::now();
    let _d1 = fetch_data(1).await;
    let _d2 = fetch_data(2).await;
    let _d3 = fetch_data(3).await;
    println!("순차 실행 시간: {:?}", start.elapsed());

    // 동시 실행 - tokio::join!
    println!("\n동시 실행 (join!):");
    let start = std::time::Instant::now();
    let (d1, d2, d3) = tokio::join!(fetch_data(1), fetch_data(2), fetch_data(3));
    println!("결과: {}, {}, {}", d1, d2, d3);
    println!("동시 실행 시간: {:?}", start.elapsed());

    // 태스크 스폰 - 별도 태스크로 실행
    println!("\n태스크 스폰:");
    let handle1 = tokio::spawn(async {
        fetch_data(10).await
    });

    let handle2 = tokio::spawn(async {
        fetch_data(20).await
    });

    // 결과 대기
    let result1 = handle1.await.unwrap();
    let result2 = handle2.await.unwrap();
    println!("스폰 결과: {}, {}", result1, result2);

    // C++과 비교:
    // C++: std::async, std::future
    // Rust: tokio::spawn, Future
    // 차이점: Rust는 런타임이 태스크를 효율적으로 스케줄링
}

// ----------------------------------------------------------------------------
// 비동기 채널
// ----------------------------------------------------------------------------

async fn channels_async() {
    println!("\n--- 비동기 채널 ---");

    use tokio::sync::mpsc;

    // 다중 생산자, 단일 소비자 채널
    let (tx, mut rx) = mpsc::channel::<String>(32);

    // 생산자 태스크
    let tx1 = tx.clone();
    tokio::spawn(async move {
        for i in 0..3 {
            tx1.send(format!("생산자1: {}", i)).await.unwrap();
            sleep(Duration::from_millis(10)).await;
        }
    });

    let tx2 = tx.clone();
    tokio::spawn(async move {
        for i in 0..3 {
            tx2.send(format!("생산자2: {}", i)).await.unwrap();
            sleep(Duration::from_millis(15)).await;
        }
    });

    // 원본 tx drop (중요!)
    drop(tx);

    // 소비자
    while let Some(msg) = rx.recv().await {
        println!("수신: {}", msg);
    }

    println!("채널 종료");

    // oneshot 채널 - 단일 값 전송
    use tokio::sync::oneshot;

    let (tx, rx) = oneshot::channel();

    tokio::spawn(async move {
        sleep(Duration::from_millis(50)).await;
        tx.send("완료!").unwrap();
    });

    let result = rx.await.unwrap();
    println!("oneshot 결과: {}", result);
}

// ----------------------------------------------------------------------------
// select! 매크로
// ----------------------------------------------------------------------------

async fn select_example() {
    println!("\n--- select! 매크로 ---");

    use tokio::sync::mpsc;

    let (tx, mut rx) = mpsc::channel::<i32>(10);

    tokio::spawn(async move {
        for i in 0..5 {
            sleep(Duration::from_millis(100)).await;
            let _ = tx.send(i).await;
        }
    });

    let timeout = sleep(Duration::from_millis(250));
    tokio::pin!(timeout);

    loop {
        tokio::select! {
            // 채널에서 수신
            Some(msg) = rx.recv() => {
                println!("수신: {}", msg);
            }
            // 타임아웃
            _ = &mut timeout => {
                println!("타임아웃!");
                break;
            }
        }
    }

    // select!는 여러 Future 중 먼저 완료되는 것 선택
    // C++에는 직접적인 대응이 없음 (직접 구현 필요)
}

// ----------------------------------------------------------------------------
// 비동기 에러 처리
// ----------------------------------------------------------------------------

async fn might_fail(succeed: bool) -> Result<String, String> {
    sleep(Duration::from_millis(10)).await;
    if succeed {
        Ok(String::from("성공!"))
    } else {
        Err(String::from("실패!"))
    }
}

async fn error_handling_async() {
    println!("\n--- 비동기 에러 처리 ---");

    // ? 연산자 사용
    async fn process() -> Result<(), String> {
        let result = might_fail(true).await?;
        println!("결과: {}", result);
        Ok(())
    }

    match process().await {
        Ok(_) => println!("process 성공"),
        Err(e) => println!("process 에러: {}", e),
    }

    // try_join! - 모든 Future 성공해야 함
    let result = tokio::try_join!(might_fail(true), might_fail(true));

    match result {
        Ok((a, b)) => println!("try_join 성공: {}, {}", a, b),
        Err(e) => println!("try_join 실패: {}", e),
    }

    // 하나라도 실패하면 에러
    let result = tokio::try_join!(might_fail(true), might_fail(false));

    match result {
        Ok((a, b)) => println!("try_join 성공: {}, {}", a, b),
        Err(e) => println!("try_join 실패: {}", e),
    }
}

// ----------------------------------------------------------------------------
// 동기 vs 비동기 비교
// ----------------------------------------------------------------------------

fn sync_vs_async_comparison() {
    println!("\n--- 동기 vs 비동기 비교 ---");

    println!("
┌─────────────────────────────────────────────────────────────┐
│                    동기 (Synchronous)                       │
├─────────────────────────────────────────────────────────────┤
│ - 블로킹 I/O                                                │
│ - 스레드당 하나의 작업                                       │
│ - 간단한 코드 흐름                                          │
│ - 많은 동시 연결 시 스레드 수 증가                           │
│                                                             │
│ C++: std::thread + 블로킹 I/O                               │
│ Rust: std::thread + std::io                                 │
└─────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────┐
│                   비동기 (Asynchronous)                     │
├─────────────────────────────────────────────────────────────┤
│ - 논블로킹 I/O                                              │
│ - 소수의 스레드로 많은 작업                                  │
│ - async/await로 동기 코드처럼 작성                          │
│ - I/O 바운드 작업에 적합                                    │
│                                                             │
│ C++20: co_await + coroutines                                │
│ Rust: async/await + tokio/async-std                         │
└─────────────────────────────────────────────────────────────┘
");

    println!("언제 비동기를 사용할까?");
    println!("✓ 네트워크 I/O (HTTP 서버, 클라이언트)");
    println!("✓ 파일 I/O (많은 파일 동시 처리)");
    println!("✓ 타이머, 지연");
    println!("✓ 많은 동시 연결");
    println!();
    println!("언제 동기를 사용할까?");
    println!("✓ CPU 바운드 작업");
    println!("✓ 간단한 스크립트");
    println!("✓ 동시성이 필요 없는 경우");
}
