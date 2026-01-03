// ============================================================================
// 16. Unsafe Rust
// ============================================================================
// C++20과의 핵심 차이점:
// 1. Rust는 기본적으로 안전함 - unsafe는 명시적으로 선언
// 2. unsafe 블록 내에서만 특정 작업 가능 - C++는 모든 곳에서 가능
// 3. unsafe는 "컴파일러를 신뢰해줘"라는 의미 - 버그 있으면 정의되지 않은 동작
// 4. FFI(외부 함수 인터페이스)로 C 코드와 상호작용
// 5. 안전한 추상화로 unsafe 코드를 감싸는 것이 관례
// ============================================================================

use std::slice;

pub fn run() {
    println!("\n=== 16. Unsafe Rust ===\n");

    unsafe_basics();
    raw_pointers();
    unsafe_functions();
    safe_abstractions();
    ffi_example();
    static_mut_variables();
    unsafe_traits();
}

// ----------------------------------------------------------------------------
// Unsafe 기초
// ----------------------------------------------------------------------------

fn unsafe_basics() {
    println!("--- Unsafe 기초 ---");

    // unsafe로 할 수 있는 5가지:
    // 1. raw 포인터 역참조
    // 2. unsafe 함수 또는 메서드 호출
    // 3. 가변 정적 변수 접근 또는 수정
    // 4. unsafe 트레이트 구현
    // 5. union 필드 접근

    // unsafe는 빌림 검사기를 끄지 않음!
    // 여전히 소유권 규칙은 적용됨

    // 왜 unsafe가 필요한가?
    // - 하드웨어 직접 제어
    // - 성능 최적화
    // - 다른 언어(C/C++)와 상호작용
    // - 컴파일러가 증명할 수 없는 안전한 코드

    println!("unsafe 블록은 '이 코드가 안전함을 내가 보장한다'는 의미입니다.");
}

// ----------------------------------------------------------------------------
// Raw 포인터
// ----------------------------------------------------------------------------

fn raw_pointers() {
    println!("\n--- Raw 포인터 ---");

    // Raw 포인터 타입:
    // *const T - 불변 raw 포인터 (C++: const T*)
    // *mut T   - 가변 raw 포인터 (C++: T*)

    let mut num = 5;

    // 참조에서 raw 포인터 생성 - 안전함
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // raw 포인터 생성은 안전하지만, 역참조는 unsafe
    println!("r1 주소: {:?}", r1);
    println!("r2 주소: {:?}", r2);

    // 역참조는 unsafe 블록 내에서만 가능
    unsafe {
        println!("r1 값: {}", *r1);
        println!("r2 값: {}", *r2);

        // 가변 포인터로 수정
        *r2 = 10;
        println!("수정 후 r2 값: {}", *r2);
    }

    // C++와의 차이:
    // C++: int* ptr = &num; *ptr = 10;  // 어디서든 가능
    // Rust: unsafe 블록 필요

    // 임의의 주소에 포인터 생성 (매우 위험!)
    let address = 0x012345usize;
    let _r = address as *const i32;
    // unsafe { println!("{}", *_r); }  // 거의 확실히 크래시!

    // raw 포인터의 특징:
    // - null 가능
    // - 자동 해제 없음
    // - 빌림 규칙 무시 가능
    // - 유효성 보장 없음

    // 가변/불변 포인터 동시 존재 가능 (일반 참조에서는 불가)
    let mut value = 42;
    let ptr1 = &value as *const i32;
    let ptr2 = &mut value as *mut i32;

    unsafe {
        // 둘 다 접근 가능하지만, 동시 수정은 정의되지 않은 동작!
        println!("ptr1: {}, ptr2: {}", *ptr1, *ptr2);
    }
}

// ----------------------------------------------------------------------------
// Unsafe 함수
// ----------------------------------------------------------------------------

// unsafe 함수 선언
unsafe fn dangerous() {
    println!("이 함수는 unsafe입니다!");
}

// 안전한 함수 내부에서 unsafe 사용
fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    // 표준 라이브러리의 split_at_mut과 동일한 구현
    // 빌림 검사기는 같은 슬라이스에서 두 개의 가변 참조를 만드는 것을 허용하지 않음
    // 하지만 우리는 겹치지 않는 두 부분을 가리키므로 안전함
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

fn unsafe_functions() {
    println!("\n--- Unsafe 함수 ---");

    // unsafe 함수 호출
    unsafe {
        dangerous();
    }

    // 안전한 추상화 사용
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let (left, right) = split_at_mut(&mut v, 3);

    println!("left: {:?}", left);
    println!("right: {:?}", right);

    // 슬라이스 수정
    left[0] = 100;
    right[0] = 200;
    println!("수정 후 v: {:?}", v);
}

// ----------------------------------------------------------------------------
// 안전한 추상화
// ----------------------------------------------------------------------------

// 안전하지 않은 내부 구현을 안전한 API로 감싸기
mod safe_wrapper {
    use std::ptr;

    pub struct MyVec<T> {
        ptr: *mut T,
        len: usize,
        cap: usize,
    }

    impl<T> MyVec<T> {
        pub fn new() -> Self {
            MyVec {
                ptr: ptr::null_mut(),
                len: 0,
                cap: 0,
            }
        }

        pub fn len(&self) -> usize {
            self.len
        }

        pub fn is_empty(&self) -> bool {
            self.len == 0
        }

        // 안전한 API - 내부적으로 unsafe 사용
        pub fn push(&mut self, value: T) {
            if self.len == self.cap {
                self.grow();
            }

            unsafe {
                ptr::write(self.ptr.add(self.len), value);
            }
            self.len += 1;
        }

        pub fn get(&self, index: usize) -> Option<&T> {
            if index < self.len {
                unsafe { Some(&*self.ptr.add(index)) }
            } else {
                None
            }
        }

        fn grow(&mut self) {
            let new_cap = if self.cap == 0 { 1 } else { self.cap * 2 };
            let new_layout = std::alloc::Layout::array::<T>(new_cap).unwrap();

            let new_ptr = if self.cap == 0 {
                unsafe { std::alloc::alloc(new_layout) as *mut T }
            } else {
                let old_layout = std::alloc::Layout::array::<T>(self.cap).unwrap();
                unsafe {
                    std::alloc::realloc(self.ptr as *mut u8, old_layout, new_layout.size())
                        as *mut T
                }
            };

            self.ptr = new_ptr;
            self.cap = new_cap;
        }
    }

    impl<T> Drop for MyVec<T> {
        fn drop(&mut self) {
            if self.cap > 0 {
                // 요소들 drop
                for i in 0..self.len {
                    unsafe {
                        ptr::drop_in_place(self.ptr.add(i));
                    }
                }
                // 메모리 해제
                let layout = std::alloc::Layout::array::<T>(self.cap).unwrap();
                unsafe {
                    std::alloc::dealloc(self.ptr as *mut u8, layout);
                }
            }
        }
    }
}

fn safe_abstractions() {
    println!("\n--- 안전한 추상화 ---");

    use safe_wrapper::MyVec;

    let mut v = MyVec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    println!("MyVec 길이: {}", v.len());
    println!("인덱스 1: {:?}", v.get(1));
    println!("인덱스 10: {:?}", v.get(10));

    // 사용자는 unsafe 없이 안전하게 사용
    // 내부 구현의 정확성은 라이브러리 작성자가 보장
}

// ----------------------------------------------------------------------------
// FFI (Foreign Function Interface)
// ----------------------------------------------------------------------------

// C 표준 라이브러리 함수 선언
extern "C" {
    fn abs(input: i32) -> i32;
    fn strlen(s: *const i8) -> usize;
}

// Rust 함수를 C에서 호출 가능하게 만들기
#[no_mangle]
pub extern "C" fn rust_function(x: i32) -> i32 {
    x * 2
}

fn ffi_example() {
    println!("\n--- FFI (외부 함수 인터페이스) ---");

    // C 함수 호출
    unsafe {
        println!("C abs(-3) = {}", abs(-3));

        // 문자열을 C 스타일로 변환
        let s = "Hello\0";  // null 종료 문자열
        let len = strlen(s.as_ptr() as *const i8);
        println!("C strlen(\"Hello\") = {}", len);
    }

    // C++와의 상호운용:
    // - extern "C"로 C ABI 사용
    // - #[repr(C)]로 C 호환 메모리 레이아웃
    // - bindgen 크레이트로 C 헤더에서 자동 바인딩 생성

    // C 호환 구조체
    #[repr(C)]
    struct CPoint {
        x: i32,
        y: i32,
    }

    let point = CPoint { x: 10, y: 20 };
    println!("C 호환 구조체: ({}, {})", point.x, point.y);

    // 호출 규약:
    // extern "C"     - C 호출 규약 (기본)
    // extern "system" - Windows API 호출 규약
    // extern "stdcall" - Windows stdcall
}

// ----------------------------------------------------------------------------
// 정적 가변 변수
// ----------------------------------------------------------------------------

static mut COUNTER: u32 = 0;

fn add_to_counter(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn static_mut_variables() {
    println!("\n--- 정적 가변 변수 ---");

    // 가변 정적 변수 접근은 항상 unsafe
    // 멀티스레드에서 데이터 레이스 가능성

    add_to_counter(3);
    add_to_counter(5);

    unsafe {
        println!("COUNTER = {}", COUNTER);
    }

    // 더 안전한 대안: AtomicU32, Mutex 등 사용
    use std::sync::atomic::{AtomicU32, Ordering};

    static SAFE_COUNTER: AtomicU32 = AtomicU32::new(0);

    SAFE_COUNTER.fetch_add(1, Ordering::SeqCst);
    SAFE_COUNTER.fetch_add(2, Ordering::SeqCst);

    println!("SAFE_COUNTER = {}", SAFE_COUNTER.load(Ordering::SeqCst));
}

// ----------------------------------------------------------------------------
// Unsafe 트레이트
// ----------------------------------------------------------------------------

// unsafe 트레이트 - 구현자가 불변 조건을 보장해야 함
unsafe trait UnsafeTrait {
    fn do_something(&self);
}

struct SafeType;

// unsafe 트레이트 구현
unsafe impl UnsafeTrait for SafeType {
    fn do_something(&self) {
        println!("SafeType이 UnsafeTrait을 구현했습니다.");
    }
}

fn unsafe_traits() {
    println!("\n--- Unsafe 트레이트 ---");

    let s = SafeType;
    s.do_something();

    // 대표적인 unsafe 트레이트:
    // Send - 스레드 간 소유권 이전 가능
    // Sync - 스레드 간 참조 공유 가능

    // 대부분의 타입은 자동으로 Send/Sync 구현
    // raw 포인터, Rc 등은 구현 안 됨

    println!("\nSend/Sync 트레이트:");
    println!("- 컴파일러가 자동 구현 추론");
    println!("- unsafe impl로 수동 구현 가능");
    println!("- 잘못 구현하면 데이터 레이스 가능");
}
