/*
* File       : rust_pl/src/deref_trait/main.rs
* Time       ：2025/2/6 14:10
* Author     ：sandwich
* version    ：V1.0
* Description：
*/

use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> Self {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}", name);
}

fn hello_(name: String) {
    println!("Hello, {}", name);
}

fn main() {
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    hello(m.deref());
    hello((*m).deref());
    hello_((*m).clone());
}

#[test]
fn test_general_ref() {
    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y);
}

#[test]
fn test_box_ref() {
    let x = 5;
    let y = Box::new(5);
    assert_eq!(5, x);
    assert_eq!(5, *y);
}

#[test]
fn test_my_box() {
    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);
}
