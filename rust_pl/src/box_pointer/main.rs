/*
* File       : rust_pl/src/box_pointer/main.rs
* Time       ：2025/1/20 17:38
* Author     ：sandwich
* version    ：V1.0
* Description：
*/
use crate::List::{Cons, Nil};

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
    detail: String,
}

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

/// 递归函数处理递归`数据结构`
fn match_box_recursion(list: List) {
    match list {
        Cons(i, list_item) => {
            println!("number: {}", i);
            match_box_recursion(*list_item);
        }
        Nil => {
            println!("Done");
        }
    }
}

fn main() {
    let b = Box::new(5);
    println!("b = {:?}", b);

    let point = Point {
        x: 0,
        y: 0,
        detail: "hello".to_string(),
    };
    let _x = point.x;
    let _s = &point.detail;
    println!("point x: {:?}, y: {:?}", point.x, point.y);
    println!("point s: {:?}", point.detail);

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    match_box_recursion(list);
}

#[test]
fn test_box_deref() {
    use std::ops::Deref;
    trait Person {
        fn get_name(&self) -> String;
    }

    struct Employee {
        name: String,
    }
    impl Employee {
        fn new(name: String) -> Self {
            Self { name }
        }
    }

    impl Person for Employee {
        fn get_name(&self) -> String {
            self.name.clone()
        }
    }

    fn print_name(person: &dyn Person) {
        println!("hello {}", person.get_name());
    }

    let person: Box<dyn Person> =
        Box::new(Employee::new("sandwich".to_string()));

    print_name(person.deref());
}
