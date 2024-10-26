/*
* File       : main.rs
* Time       ：2024/10/17 22:11
* Author     ：sandwich
* version    ：V1.0
* Description：reference in rust
*/

use std::collections::HashMap;

type Table = HashMap<String, Vec<String>>;

///打印Table信息
fn show(table: &Table) {
    for (artist, works) in table {
        println!("Works by {}", artist);
        for work in works {
            println!("    {}", work);
        }
    }
}

fn sort_works(table: &mut Table) {
    for (_, works) in table {
        works.sort();
    }
}

#[test]
fn test_reference() {
    let x = 10;
    let r = &x;
    assert_eq!(*r, 10);
}

#[test]
#[allow(dead_code)]
fn test_implicit_dereference() {
    struct Anime {
        name: &'static str,
        bechdel_pass: bool,
    }
    let aria = Anime {
        name: "Aria: The Animation",
        bechdel_pass: true,
    };
    let anime_ref = &aria;
    assert_eq!(anime_ref.name, "Aria: The Animation");
    // 两个等价写法，隐式解引用
    assert_eq!((*anime_ref).name, "Aria: The Animation");

    let mut v = vec![1973, 1968];
    v.sort();
    // 等价
    (&mut v).sort();
    println!("{:?}", v);

    // ref->ref
    struct Point {
        x: i32,
        y: i32,
    }
    let point = Point { x: 1000, y: 729 };
    let r = &point;
    let rr = &r;
    let rrr = &rr;
    assert_eq!(rrr.y, 729); // . 准确自动解引用
    assert_eq!((***rrr).y, 729);

    // 比较运算自动解引用
    let x = 10;
    let y = 10;
    let rx = &x;
    let ry = &y;
    let rrx = &rx;
    let rry = &ry;
    assert_eq!(rrx, rry);
    assert!(rrx <= rry);
    // 比较值
    assert_eq!(rx, ry);
    // 比较地址
    assert!(!std::ptr::eq(rx, ry));
}

#[test]
fn test_expr_ref() {
    fn factorial(n: usize) -> usize {
        (1..n + 1).fold(1, |x, y| { x * y })
    }
    let r = &factorial(6);
    assert_eq!(r + &1009, 1729);
}

#[test]
fn test_ref_safe() {
    /*
    let r: &i32;
    {
        let x = 1;
        r = &x; // x的生命周期不足以传递给r,r的生命周期更长
    }
    assert_eq!(*r, 1); // 外部一旦使用r，r的作用域就不局限在上面的子作用域内了，r就悬空了
    */
    // 满足生命周期条件的
    let x = 1;
    {
        let r = &x;
        assert_eq!(*r, 1);
    }
}

/// 引用作为参数
#[test]
fn test_ref_param() {
    static mut STASH: &i32 = &128;
    fn f(p: &'static i32) {
        // p引用传递给STASH，p的生命周期必须>=static (这里只能是static)
        unsafe {
            STASH = p;
            println!("p value {}", *p);
            println!("STASH value {}", *STASH);
        }
    }
    f(&33);
    static WORTH_POINTING_AT: i32 = 1000;
    f(&WORTH_POINTING_AT);
    unsafe {
        println!("final STASH is {}", *STASH);
    }
}

/// 返回引用
#[test]
fn test_return_ref() {
    fn find_smallest(v: &[i32]) -> &i32 {
        let mut s = &v[0];
        for r in &v[1..] {
            if r < s {
                s = r;
            }
        }
        s
    }

    let v = [3, 9, 1, 5];
    println!("smallest value in {:?} is {}", v, find_smallest(&v));
}

/// 结构体中的引用
#[test]
fn test_ref_in_struct() {
    struct S<'a> {
        r: &'a i32,
    }
    let x = 10;
    {
        // r(&x)的生命周期必须<=x的生命周期
        let s = S { r: &x };
        assert_eq!(*(s.r), 10);
    }
}

fn main() {
    let mut table = Table::new();
    table.insert("Gesualdo".to_string(), vec!["many madrigals".to_string(),
                                              "Tenebrae Responsoria".to_string()]);
    table.insert("Caravaggio".to_string(), vec!["The Musicians".to_string(),
                                                "The Calling of St. Matthew".to_string()]);
    table.insert("Cellini".to_string(), vec!["Perseus with the head of Medusa".to_string(),
                                             "a salt cellar".to_string()]);
    // show(table); // 转移所有权
    show(&table);
    sort_works(&mut table);
    show(&table);
}