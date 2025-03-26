/*
* File       : rust_pl/src/rc_pointer/main.rs
* Time       ：2025/2/10 14:51
* Author     ：sandwich
* version    ：V1.0
* Description：
*/
use crate::List::{Cons, Nil};
use std::rc::Rc;

#[derive(Debug)]
#[allow(dead_code)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

#[test]
fn test_box_rc_pointer() {
    #[derive(Debug)]
    #[allow(dead_code)]
    enum MyList<'a> {
        Cons(i32, &'a MyList<'a>),
        Nil,
    }

    use MyList::{Cons, Nil};

    let t = Cons(10, &Nil);
    let a = Cons(5, &t);
    let b = Cons(4, &a);
    let c = Cons(3, &a);
    // let a = Cons(5, Box::new(&Cons(10, Box::new(&Nil))));
    // let b = Cons(3, Box::new(&a));
    // let c = Cons(4, &Box::new(a));

    println!("b: {:?}", b);
    println!("c: {:?}", c);
}

#[test]
fn test_mut() {
    let x = 5;
    // let y = &mut x; // panic
    let y = &x;
}

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let _c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));

    println!("b: {:?}", b);
}
