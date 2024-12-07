/*
* File       : lib.rs
* Time       ：2024/12/6 14:44
* Author     ：sandwich
* version    ：V1.0
* Description：
*/

mod inner_mod;

pub struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    pub fn can_hold(&self, another: Self) -> bool {
        self.length > another.length && self.width > another.width
    }
}

#[allow(dead_code)]
struct Guess {
    value: u32,
}

impl Guess {
    #[allow(dead_code)]
    fn new(value: u32) -> Self {
        // if value < 1 || value > 100 {
        //     panic!("Guess value must be between 1 and 100, got {}.", value);
        // }
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}.",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value
            );
        }
        Guess { value }
    }
}

#[allow(dead_code)]
pub fn add_two(x: i32) -> i32 {
    x + 2
}

#[allow(dead_code)]
fn internal_adder(x: i32, y: i32) -> i32 {
    x + y
}

#[allow(dead_code)]
fn greeting(name: &str) -> String {
    format!("hello {}!", name)
}

#[allow(dead_code)]
fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}

#[cfg(test)]
mod lib_tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn it_works_result() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    #[test]
    fn exploration() {
        assert_eq!(2 + 1, 3);
    }

    #[test]
    #[should_panic]
    fn another() {
        panic!("Make this test fail");
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
        assert!(larger.can_hold(smaller));
        // borrowed
        let smaller = Rectangle {
            length: 5,
            width: 1,
        };
        assert!(!smaller.can_hold(larger));
    }

    #[test]
    fn test_add_two() {
        assert_eq!(add_two(2), 4);
    }

    #[test]
    #[should_panic]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Test"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn gather_than_100() {
        Guess::new(200);
        // Guess::new(0); not expected panic
    }

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

    #[test]
    #[ignore]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(5, value);
    }
}
