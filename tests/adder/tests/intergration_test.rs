// 여러개의 테스트는 기본적으로 병렬로 처리됨
// cargo test -- --test-threads=1; // 해당 커멘드는 테스트를 병렬로 처리하지 않음을 설정
// cargo test -- --test-threads=1 --nocapture; // 테스트의 출력 값 보기.
// cargo test -- --ignored // 제외한 테스트만 실행

extern crate adder;
use adder::Rectangle::Rectangle;

#[test]
fn this_test_will_pass() {
    let value = adder::prints_and_returns_10(4);
    assert_eq!(10, value);
}

#[test]
fn this_test_will_fail() {
    let value = adder::prints_and_returns_10(8);
    assert_eq!(5, value);
}

#[test]
#[should_panic(expected = "Guess value must be less than or equal to 100")] // 해당 Attribute가 있으면 panic이 발생해야 통과이다.
fn greater_than_100() {
    adder::Guess::new(200);
}

#[test]
fn greeting_contains_name() {
    let result = adder::greeting("Carol");
    assert!(
        result.contains("Carol"),
        "Greeting did not contain name, value was `{}`",
        result
    );
}

#[test]
#[ignore]
fn it_adds_two() {
    assert_eq!(4, adder::add_two(2));
}

#[test]
fn larger_can_hold_smaller() {
    let larger = Rectangle {
        length: 8,
        width: 7,
    };
    let smaller = Rectangle {
        length: 5,
        width: 1,
    };

    assert!(larger.can_hold(&smaller));
}

#[test]
fn smaller_cannot_hold_larger() {
    let larger = Rectangle {
        length: 8,
        width: 7,
    };
    let smaller = Rectangle {
        length: 5,
        width: 1,
    };

    assert!(!smaller.can_hold(&larger));
}
