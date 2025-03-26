/*
* File       : rust_pl/src/refcell_pointer/main.rs
* Time       ：2025/3/14 16:59
* Author     ：sandwich
* version    ：V1.0
* Description：
*/

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

mod lib;

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

fn main() {
    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    println!("{:?}", *value);

    // println!("a after = {:?}", a);
    // println!("b after = {:?}", b);
    // println!("c after = {:?}", c);
}
