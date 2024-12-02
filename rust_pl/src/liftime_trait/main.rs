/*
* File       : main.rs
* Time       ：2024/11/14 20:57
* Author     ：sandwich
* version    ：V1.0
* Description：
*/
use std::fmt::Display;

#[test]
fn test_dangling_reference() {
    // let r: &i32;
    // {
    //     let x = 32;
    //     let r = &x;
    // }
    // println!("value is {}", r);

    // fix up
    let x = 5;
    let r = &x;
    println!("value is {}", r);
}

#[derive(Debug)]
#[allow(dead_code)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

#[allow(dead_code)]
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn longest_with_an_announcement<'a, T: Display>(x: &'a str, y: &'a str, ann: T) -> &'a str {
    println!("Announcement!: {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let a = String::from("hello");
    let b = "abc";
    println!("longest is: {}", longest(&a, b));
    {
        let longest_str = longest(&a, b);
        println!("longest inner: {}", longest_str);
    }

    let longest_out: &str;
    {
        let a_inner = "test inner";
        let b_inner = "hello";
        longest_out = longest(a_inner, b_inner);
    }
    println!("longest out: {}", longest_out);

    // 结构体中的生命周期
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");
    let i = ImportantExcerpt { part: first_sentence };
    println!("i: {:?}", i);

    let announcement = "Notice";
    println!("{}", longest_with_an_announcement(&a, b, announcement));
}