// ============================================================================
// 12. 스마트 포인터 (Smart Pointers)
// ============================================================================
// C++20과의 핵심 차이점:
// 1. Box<T> ≈ std::unique_ptr<T> - 단일 소유권, 힙 할당
// 2. Rc<T> ≈ std::shared_ptr<T> - 참조 카운팅 (단일 스레드)
// 3. Arc<T> ≈ std::shared_ptr<T> - 참조 카운팅 (멀티 스레드)
// 4. RefCell<T> - 런타임 빌림 검사 (C++에 없음)
// 5. Weak<T> ≈ std::weak_ptr<T> - 순환 참조 방지
// ============================================================================

use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub fn run() {
    println!("\n=== 12. 스마트 포인터 ===\n");

    box_pointer();
    deref_trait();
    drop_trait();
    rc_pointer();
    refcell_pointer();
    interior_mutability();
    reference_cycles();
}

// ----------------------------------------------------------------------------
// Box<T> - 힙 할당 단일 소유권
// ----------------------------------------------------------------------------

fn box_pointer() {
    println!("--- Box<T> ---");

    // Box = 힙에 데이터 저장
    // C++: std::unique_ptr<int> ptr = std::make_unique<int>(5);
    let b = Box::new(5);
    println!("Box: {}", b);

    // Box 사용 이유:
    // 1. 컴파일 타임에 크기를 알 수 없는 타입
    // 2. 큰 데이터의 소유권 이전 (복사 방지)
    // 3. 트레이트 객체

    // 재귀 타입 정의에 필수
    // 이것은 컴파일 에러:
    // enum List { Cons(i32, List), Nil }  // 무한 크기!

    // Box로 해결:
    #[derive(Debug)]
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }

    use List::{Cons, Nil};

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("List: {:?}", list);

    // Box는 스택처럼 사용 가능 (Deref)
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);  // 역참조
    println!("Box 역참조: {}", *y);
}

// ----------------------------------------------------------------------------
// Deref 트레이트 - 역참조 연산자 오버로딩
// ----------------------------------------------------------------------------

fn deref_trait() {
    println!("\n--- Deref 트레이트 ---");

    // Deref 트레이트로 * 연산자 커스터마이즈

    use std::ops::Deref;

    struct MyBox<T>(T);

    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }

    impl<T> Deref for MyBox<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, *y);  // *(y.deref()) 로 변환됨
    println!("MyBox 역참조: {}", *y);

    // 역참조 강제 변환 (Deref Coercion)
    // &String -> &str 자동 변환이 이것 때문

    fn hello(name: &str) {
        println!("Hello, {}!", name);
    }

    let m = MyBox::new(String::from("Rust"));
    hello(&m);  // &MyBox<String> -> &String -> &str 자동 변환

    // DerefMut - 가변 역참조
    // impl<T> DerefMut for MyBox<T> { ... }
}

// ----------------------------------------------------------------------------
// Drop 트레이트 - 소멸자
// ----------------------------------------------------------------------------

fn drop_trait() {
    println!("\n--- Drop 트레이트 ---");

    // Drop = C++ 소멸자
    // 스코프 벗어날 때 자동 호출

    struct CustomSmartPointer {
        data: String,
    }

    impl Drop for CustomSmartPointer {
        fn drop(&mut self) {
            println!("CustomSmartPointer 해제: {}", self.data);
        }
    }

    {
        let _c = CustomSmartPointer {
            data: String::from("my stuff"),
        };
        let _d = CustomSmartPointer {
            data: String::from("other stuff"),
        };
        println!("CustomSmartPointers 생성됨");
    }  // d 먼저, 그 다음 c (역순)

    println!("스코프 종료 후");

    // 조기 해제 - std::mem::drop 사용
    let c = CustomSmartPointer {
        data: String::from("조기 해제"),
    };
    println!("조기 해제 전");
    drop(c);  // 여기서 해제
    // c.drop();  // 이건 에러! drop()은 직접 호출 불가
    println!("조기 해제 후");
}

// ----------------------------------------------------------------------------
// Rc<T> - 참조 카운팅 (단일 스레드)
// ----------------------------------------------------------------------------

fn rc_pointer() {
    println!("\n--- Rc<T> ---");

    // Rc = Reference Counted
    // C++: std::shared_ptr (단일 스레드 전용)

    // 여러 소유자가 필요한 경우
    #[derive(Debug)]
    enum List {
        Cons(i32, Rc<List>),
        Nil,
    }

    use List::{Cons, Nil};

    // 공유 리스트
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("a 생성 후 카운트: {}", Rc::strong_count(&a));

    // Rc::clone은 얕은 복사 (카운트만 증가)
    let b = Cons(3, Rc::clone(&a));
    println!("b 생성 후 카운트: {}", Rc::strong_count(&a));

    {
        let c = Cons(4, Rc::clone(&a));
        println!("c 생성 후 카운트: {}", Rc::strong_count(&a));
    }

    println!("c 해제 후 카운트: {}", Rc::strong_count(&a));

    // Rc는 불변! 데이터 수정 불가
    // 가변이 필요하면 Rc<RefCell<T>> 사용

    // 주의: Rc는 단일 스레드 전용!
    // 멀티스레드에서는 Arc<T> 사용
}

// ----------------------------------------------------------------------------
// RefCell<T> - 런타임 빌림 검사
// ----------------------------------------------------------------------------

fn refcell_pointer() {
    println!("\n--- RefCell<T> ---");

    // RefCell = 런타임에 빌림 규칙 검사
    // 컴파일 타임에 안전성 증명 어려울 때 사용

    // Box<T>: 컴파일 타임 빌림, 가변/불변 소유권
    // Rc<T>: 컴파일 타임 빌림, 불변 공유 소유권
    // RefCell<T>: 런타임 빌림, 가변/불변 단일 소유권

    let data = RefCell::new(5);

    // borrow() - 불변 참조 (Ref<T>)
    {
        let r1 = data.borrow();
        let r2 = data.borrow();  // 여러 불변 참조 OK
        println!("불변 참조: {}, {}", *r1, *r2);
    }

    // borrow_mut() - 가변 참조 (RefMut<T>)
    {
        let mut r = data.borrow_mut();
        *r += 10;
        println!("가변 참조로 수정: {}", *r);
    }

    println!("최종 값: {}", data.borrow());

    // 런타임 패닉 예제 (주석 해제하면 패닉)
    // let r1 = data.borrow();
    // let r2 = data.borrow_mut();  // 패닉! 불변 참조 있는데 가변 참조 시도
}

// ----------------------------------------------------------------------------
// 내부 가변성 패턴
// ----------------------------------------------------------------------------

fn interior_mutability() {
    println!("\n--- 내부 가변성 ---");

    // 불변 참조를 통해 내부 데이터 수정 가능
    // "눈속임" 가변성 - 외부에서는 불변으로 보임

    // Rc<RefCell<T>> 조합 - 여러 소유자 + 가변성
    #[derive(Debug)]
    struct Node {
        value: i32,
        children: RefCell<Vec<Rc<Node>>>,
    }

    let leaf = Rc::new(Node {
        value: 3,
        children: RefCell::new(vec![]),
    });

    let branch = Rc::new(Node {
        value: 5,
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    // 불변 참조지만 children 수정 가능
    branch.children.borrow_mut().push(Rc::new(Node {
        value: 10,
        children: RefCell::new(vec![]),
    }));

    println!("트리: {:?}", branch);

    // Mock 객체 예제
    pub trait Messenger {
        fn send(&self, msg: &str);
    }

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,  // 내부 가변성
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {  // &self인데도
            self.sent_messages.borrow_mut().push(String::from(message));  // 수정 가능
        }
    }

    let mock = MockMessenger::new();
    mock.send("테스트 메시지");
    println!("전송된 메시지: {:?}", mock.sent_messages.borrow());
}

// ----------------------------------------------------------------------------
// 순환 참조와 Weak<T>
// ----------------------------------------------------------------------------

fn reference_cycles() {
    println!("\n--- 순환 참조 방지 ---");

    // Rc로 순환 참조 만들면 메모리 누수!
    // Weak<T>로 해결 (C++ weak_ptr과 동일)

    // Weak 특징:
    // - strong_count에 영향 없음
    // - 참조 대상이 해제될 수 있음
    // - 사용하려면 upgrade() -> Option<Rc<T>>

    #[derive(Debug)]
    struct TreeNode {
        value: i32,
        parent: RefCell<Weak<TreeNode>>,      // 부모는 Weak로
        children: RefCell<Vec<Rc<TreeNode>>>, // 자식은 Rc로
    }

    let leaf = Rc::new(TreeNode {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong: {}, weak: {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );

    {
        let branch = Rc::new(TreeNode {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        // leaf의 부모를 branch로 설정
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong: {}, weak: {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch)
        );

        println!(
            "leaf strong: {}, weak: {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf)
        );

        // 부모 접근
        if let Some(parent) = leaf.parent.borrow().upgrade() {
            println!("leaf의 부모 값: {}", parent.value);
        }
    }  // branch 해제됨

    // branch 해제 후 부모 접근 시도
    println!(
        "branch 해제 후 leaf strong: {}, weak: {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );

    let parent_upgrade = leaf.parent.borrow().upgrade();
    match parent_upgrade {
        Some(parent) => println!("부모: {}", parent.value),
        None => println!("부모가 이미 해제됨"),
    }
}
