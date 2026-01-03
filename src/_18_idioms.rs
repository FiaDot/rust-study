// ============================================================================
// 18. 실무 Rust Idiom
// ============================================================================
// C++20과의 핵심 차이점:
// 1. 빌더 패턴이 소유권과 결합되어 더 안전함
// 2. 타입 스테이트 패턴으로 컴파일 타임 상태 검증
// 3. From/Into 트레이트로 일관된 타입 변환
// 4. Newtype으로 제로 코스트 타입 안전성
// 5. RAII가 언어 레벨에서 강제됨
// ============================================================================

use std::fmt;
use std::ops::Deref;

pub fn run() {
    println!("\n=== 18. 실무 Rust Idiom ===\n");

    builder_pattern();
    newtype_pattern();
    typestate_pattern();
    from_into_pattern();
    default_pattern();
    deref_coercion();
    raii_pattern();
    error_handling_best_practices();
}

// ============================================================================
// 1. 빌더 패턴 (Builder Pattern)
// ============================================================================

fn builder_pattern() {
    println!("--- 빌더 패턴 ---");

    // 복잡한 객체를 단계별로 생성
    // C++: 빌더 클래스 + 메서드 체이닝

    #[derive(Debug)]
    struct Server {
        host: String,
        port: u16,
        max_connections: u32,
        timeout_secs: u64,
        tls_enabled: bool,
    }

    // 빌더 구조체
    #[derive(Default)]
    struct ServerBuilder {
        host: Option<String>,
        port: Option<u16>,
        max_connections: Option<u32>,
        timeout_secs: Option<u64>,
        tls_enabled: Option<bool>,
    }

    impl ServerBuilder {
        fn new() -> Self {
            Self::default()
        }

        // 각 메서드는 self를 소비하고 Self를 반환 (소유권 이동)
        fn host(mut self, host: impl Into<String>) -> Self {
            self.host = Some(host.into());
            self
        }

        fn port(mut self, port: u16) -> Self {
            self.port = Some(port);
            self
        }

        fn max_connections(mut self, max: u32) -> Self {
            self.max_connections = Some(max);
            self
        }

        fn timeout(mut self, secs: u64) -> Self {
            self.timeout_secs = Some(secs);
            self
        }

        fn tls(mut self, enabled: bool) -> Self {
            self.tls_enabled = Some(enabled);
            self
        }

        // 최종 빌드 - 필수 필드 검증
        fn build(self) -> Result<Server, &'static str> {
            Ok(Server {
                host: self.host.ok_or("host is required")?,
                port: self.port.ok_or("port is required")?,
                max_connections: self.max_connections.unwrap_or(100),
                timeout_secs: self.timeout_secs.unwrap_or(30),
                tls_enabled: self.tls_enabled.unwrap_or(false),
            })
        }
    }

    // 사용
    let server = ServerBuilder::new()
        .host("localhost")
        .port(8080)
        .max_connections(1000)
        .tls(true)
        .build()
        .unwrap();

    println!("서버 설정: {:?}", server);

    // 필수 필드 누락 시 에러
    let result = ServerBuilder::new().host("localhost").build();
    println!("필수 필드 누락: {:?}", result);

    // C++ 빌더와의 차이:
    // - Rust는 소유권으로 빌더 재사용 방지 가능
    // - Option으로 선택적 필드 명확히 표현
    // - Result로 빌드 실패 처리
}

// ============================================================================
// 2. Newtype 패턴
// ============================================================================

fn newtype_pattern() {
    println!("\n--- Newtype 패턴 ---");

    // 기존 타입을 감싸서 새로운 타입 생성
    // 컴파일 타임에 타입 구분, 런타임 오버헤드 없음

    // 단위 구분
    #[derive(Debug, Clone, Copy)]
    struct Meters(f64);

    #[derive(Debug, Clone, Copy)]
    struct Kilometers(f64);

    impl Meters {
        fn to_kilometers(self) -> Kilometers {
            Kilometers(self.0 / 1000.0)
        }
    }

    impl Kilometers {
        fn to_meters(self) -> Meters {
            Meters(self.0 * 1000.0)
        }
    }

    let distance_m = Meters(5000.0);
    let distance_km = distance_m.to_kilometers();

    println!("{:?} = {:?}", distance_m, distance_km);

    // 실수 방지 - 다른 타입끼리 연산 불가
    // let wrong = distance_m.0 + distance_km.0;  // 의도적 에러 유발 가능

    // ID 타입 구분
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    struct UserId(u64);

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    struct OrderId(u64);

    fn get_user_orders(_user_id: UserId) -> Vec<OrderId> {
        vec![OrderId(1001), OrderId(1002)]
    }

    let user = UserId(42);
    let _order = OrderId(1001);

    // get_user_orders(order);  // 컴파일 에러! OrderId는 UserId가 아님
    let orders = get_user_orders(user);
    println!("사용자 {:?}의 주문: {:?}", user, orders);

    // Deref로 내부 타입 노출
    struct Email(String);

    impl Deref for Email {
        type Target = str;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    let email = Email(String::from("user@example.com"));
    println!("이메일 길이: {}", email.len()); // str 메서드 사용 가능

    // C++와의 비교:
    // C++: using UserId = uint64_t;  // 타입 별칭, 실제로 같은 타입
    // Rust: struct UserId(u64);      // 완전히 다른 타입
}

// ============================================================================
// 3. 타입 스테이트 패턴 (Type State Pattern)
// ============================================================================

fn typestate_pattern() {
    println!("\n--- 타입 스테이트 패턴 ---");

    // 컴파일 타임에 상태 전이를 강제
    // 잘못된 상태에서 메서드 호출 방지

    // 상태를 나타내는 마커 타입
    struct Draft;
    struct Published;

    struct Post<State> {
        content: String,
        _state: std::marker::PhantomData<State>,
    }

    // Draft 상태에서만 사용 가능한 메서드
    impl Post<Draft> {
        fn new(content: impl Into<String>) -> Self {
            Post {
                content: content.into(),
                _state: std::marker::PhantomData,
            }
        }

        fn edit(&mut self, new_content: impl Into<String>) {
            self.content = new_content.into();
        }

        // 상태 전이: Draft -> Published
        fn publish(self) -> Post<Published> {
            println!("게시물 발행!");
            Post {
                content: self.content,
                _state: std::marker::PhantomData,
            }
        }
    }

    // Published 상태에서만 사용 가능한 메서드
    impl Post<Published> {
        fn view(&self) -> &str {
            &self.content
        }

        // 상태 전이: Published -> Draft
        fn unpublish(self) -> Post<Draft> {
            println!("게시물 비공개!");
            Post {
                content: self.content,
                _state: std::marker::PhantomData,
            }
        }
    }

    // 사용
    let mut draft = Post::<Draft>::new("초안 내용");
    draft.edit("수정된 초안");

    // draft.view();  // 컴파일 에러! Draft 상태에서는 view 없음

    let published = draft.publish();
    println!("내용: {}", published.view());

    // published.edit("...");  // 컴파일 에러! Published 상태에서는 edit 없음

    let _draft_again = published.unpublish();

    // 실무 예: HTTP 연결 상태
    // Connection<Disconnected> -> Connection<Connected> -> Connection<Authenticated>

    // C++에서는 런타임 상태 체크가 필요:
    // if (state == State::Draft) { edit(); }
    // Rust는 컴파일 타임에 강제!
}

// ============================================================================
// 4. From/Into 트레이트 활용
// ============================================================================

fn from_into_pattern() {
    println!("\n--- From/Into 패턴 ---");

    // From 트레이트 구현하면 Into는 자동 구현
    // 타입 변환의 표준 방법

    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }

    // 튜플에서 Point로 변환
    impl From<(i32, i32)> for Point {
        fn from(tuple: (i32, i32)) -> Self {
            Point {
                x: tuple.0,
                y: tuple.1,
            }
        }
    }

    // 배열에서 Point로 변환
    impl From<[i32; 2]> for Point {
        fn from(arr: [i32; 2]) -> Self {
            Point { x: arr[0], y: arr[1] }
        }
    }

    // From 사용
    let p1 = Point::from((10, 20));
    let p2: Point = (30, 40).into();
    let p3: Point = [50, 60].into();

    println!("p1: {:?}, p2: {:?}, p3: {:?}", p1, p2, p3);

    // 함수 매개변수에서 활용
    fn process_point(p: impl Into<Point>) {
        let point: Point = p.into();
        println!("처리: {:?}", point);
    }

    process_point((1, 2));
    process_point([3, 4]);
    process_point(Point { x: 5, y: 6 });

    // 에러 변환에 활용
    #[derive(Debug)]
    struct CustomError {
        message: String,
    }

    impl From<std::io::Error> for CustomError {
        fn from(err: std::io::Error) -> Self {
            CustomError {
                message: err.to_string(),
            }
        }
    }

    impl From<std::num::ParseIntError> for CustomError {
        fn from(err: std::num::ParseIntError) -> Self {
            CustomError {
                message: err.to_string(),
            }
        }
    }

    // ? 연산자가 자동으로 From 호출
    fn parse_and_read() -> Result<i32, CustomError> {
        let num: i32 = "42".parse()?; // ParseIntError -> CustomError
        Ok(num)
    }

    println!("파싱 결과: {:?}", parse_and_read());

    // C++ 비교:
    // C++: explicit 변환 생성자, 변환 연산자
    // Rust: From/Into 트레이트로 일관된 패턴
}

// ============================================================================
// 5. Default 트레이트 활용
// ============================================================================

fn default_pattern() {
    println!("\n--- Default 패턴 ---");

    // 타입의 기본값 정의

    #[derive(Debug)]
    struct Config {
        debug: bool,
        log_level: String,
        max_threads: usize,
        timeout_ms: u64,
    }

    impl Default for Config {
        fn default() -> Self {
            Config {
                debug: false,
                log_level: String::from("info"),
                max_threads: 4,
                timeout_ms: 5000,
            }
        }
    }

    // 기본값 사용
    let config1 = Config::default();
    println!("기본 설정: {:?}", config1);

    // 일부만 커스터마이즈 (구조체 업데이트 문법)
    let config2 = Config {
        debug: true,
        max_threads: 8,
        ..Default::default()
    };
    println!("커스텀 설정: {:?}", config2);

    // derive로 자동 구현 (모든 필드가 Default 구현 시)
    #[derive(Debug, Default)]
    struct Stats {
        count: u32,      // 0
        total: f64,      // 0.0
        name: String,    // ""
    }

    let stats = Stats::default();
    println!("기본 통계: {:?}", stats);

    // Option<T>의 unwrap_or_default
    let maybe_value: Option<i32> = None;
    let value = maybe_value.unwrap_or_default(); // 0
    println!("기본값: {}", value);

    // Vec의 기본값은 빈 벡터
    let items: Vec<i32> = Default::default();
    println!("빈 벡터: {:?}", items);
}

// ============================================================================
// 6. Deref 강제 변환 (Deref Coercion)
// ============================================================================

fn deref_coercion() {
    println!("\n--- Deref 강제 변환 ---");

    // &T에서 &U로 자동 변환 (T: Deref<Target=U>)

    // String -> &str
    fn print_str(s: &str) {
        println!("문자열: {}", s);
    }

    let owned = String::from("Hello");
    print_str(&owned); // &String -> &str 자동 변환

    // Box<T> -> &T
    fn print_value(v: &i32) {
        println!("값: {}", v);
    }

    let boxed = Box::new(42);
    print_value(&boxed); // &Box<i32> -> &i32 자동 변환

    // Vec<T> -> &[T]
    fn print_slice(s: &[i32]) {
        println!("슬라이스: {:?}", s);
    }

    let vec = vec![1, 2, 3];
    print_slice(&vec); // &Vec<i32> -> &[i32] 자동 변환

    // 커스텀 스마트 포인터
    struct MyBox<T>(T);

    impl<T> MyBox<T> {
        fn new(value: T) -> Self {
            MyBox(value)
        }
    }

    impl<T> Deref for MyBox<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    let my_box = MyBox::new(String::from("Rust"));
    print_str(&my_box); // &MyBox<String> -> &String -> &str

    // 변환 체인:
    // &MyBox<String> -> &String (Deref)
    // &String -> &str (Deref)

    println!("Deref 체인 동작 확인");
}

// ============================================================================
// 7. RAII 패턴
// ============================================================================

fn raii_pattern() {
    println!("\n--- RAII 패턴 ---");

    // Resource Acquisition Is Initialization
    // C++과 동일한 개념, Rust에서는 Drop 트레이트로 구현

    struct FileHandle {
        name: String,
    }

    impl FileHandle {
        fn new(name: &str) -> Self {
            println!("파일 열기: {}", name);
            FileHandle {
                name: name.to_string(),
            }
        }

        fn write(&self, data: &str) {
            println!("'{}' 쓰기: {}", self.name, data);
        }
    }

    impl Drop for FileHandle {
        fn drop(&mut self) {
            println!("파일 닫기: {}", self.name);
        }
    }

    {
        let file = FileHandle::new("test.txt");
        file.write("Hello, RAII!");
        // 스코프 끝에서 자동으로 drop 호출
    }
    println!("스코프 종료 후");

    // 뮤텍스 가드도 RAII
    use std::sync::Mutex;

    let data = Mutex::new(0);
    {
        let mut guard = data.lock().unwrap();
        *guard += 1;
        println!("락 획득, 값: {}", *guard);
        // guard가 스코프를 벗어나면 자동으로 unlock
    }
    println!("락 해제됨");

    // 파일 자동 닫기
    // std::fs::File은 Drop 구현으로 자동 닫힘

    // C++와의 차이:
    // - Rust는 소유권으로 리소스 해제 시점이 더 명확
    // - 이동 후에는 원본에서 drop 호출 안 됨
}

// ============================================================================
// 8. 에러 처리 Best Practices
// ============================================================================

fn error_handling_best_practices() {
    println!("\n--- 에러 처리 Best Practices ---");

    // 1. 커스텀 에러 타입 정의

    #[derive(Debug)]
    enum AppError {
        NotFound { resource: String },
        InvalidInput { field: String, message: String },
        Io(std::io::Error),
        Parse(std::num::ParseIntError),
    }

    // Display 구현 (사용자 친화적 메시지)
    impl fmt::Display for AppError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                AppError::NotFound { resource } => {
                    write!(f, "리소스를 찾을 수 없음: {}", resource)
                }
                AppError::InvalidInput { field, message } => {
                    write!(f, "잘못된 입력 - {}: {}", field, message)
                }
                AppError::Io(err) => write!(f, "IO 에러: {}", err),
                AppError::Parse(err) => write!(f, "파싱 에러: {}", err),
            }
        }
    }

    // std::error::Error 구현
    impl std::error::Error for AppError {
        fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
            match self {
                AppError::Io(err) => Some(err),
                AppError::Parse(err) => Some(err),
                _ => None,
            }
        }
    }

    // From 구현으로 ? 연산자 지원
    impl From<std::io::Error> for AppError {
        fn from(err: std::io::Error) -> Self {
            AppError::Io(err)
        }
    }

    impl From<std::num::ParseIntError> for AppError {
        fn from(err: std::num::ParseIntError) -> Self {
            AppError::Parse(err)
        }
    }

    // 2. 사용 예시

    fn find_user(id: u64) -> Result<String, AppError> {
        if id == 0 {
            return Err(AppError::NotFound {
                resource: format!("user/{}", id),
            });
        }
        Ok(format!("User_{}", id))
    }

    fn validate_age(age_str: &str) -> Result<u32, AppError> {
        let age: u32 = age_str.parse()?; // ParseIntError -> AppError

        if age > 150 {
            return Err(AppError::InvalidInput {
                field: String::from("age"),
                message: String::from("나이는 150 이하여야 함"),
            });
        }

        Ok(age)
    }

    // 테스트
    println!("find_user(1): {:?}", find_user(1));
    println!("find_user(0): {:?}", find_user(0));
    println!("validate_age(\"25\"): {:?}", validate_age("25"));
    println!("validate_age(\"abc\"): {:?}", validate_age("abc"));
    println!("validate_age(\"200\"): {:?}", validate_age("200"));

    // 3. 에러 체이닝 (context 추가)
    // 실무에서는 anyhow::Context 트레이트 사용

    fn process_user(id: &str) -> Result<String, AppError> {
        let user_id: u64 = id.parse()?;
        let user = find_user(user_id)?;
        Ok(format!("처리됨: {}", user))
    }

    println!("process_user(\"5\"): {:?}", process_user("5"));
    println!("process_user(\"abc\"): {:?}", process_user("abc"));

    // 4. thiserror 스타일 (실제로는 매크로 사용)
    // #[derive(thiserror::Error, Debug)]
    // enum MyError {
    //     #[error("not found: {0}")]
    //     NotFound(String),
    //     #[error("io error")]
    //     Io(#[from] std::io::Error),
    // }

    // 5. anyhow 스타일 (동적 에러)
    // fn main() -> anyhow::Result<()> {
    //     let result = operation().context("작업 실패")?;
    //     Ok(())
    // }

    println!("\n실무 에러 처리 권장사항:");
    println!("1. 라이브러리: 구체적인 에러 타입 (thiserror)");
    println!("2. 애플리케이션: 동적 에러 (anyhow)");
    println!("3. 에러 체인으로 컨텍스트 보존");
    println!("4. Display로 사용자 메시지, Debug로 개발자 정보");
}
