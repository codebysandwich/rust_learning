/*
* File       : main.rs
* Time       ：2024/11/13 09:38
* Author     ：sandwich
* version    ：V1.0
* Description：
*/
use std::fmt::Display;

///generics define
///

/// 结构体中定义不同的泛型
///
/// x: T
/// y: U
///
/// T, U 为两种不通的泛型
#[derive(Debug)]
#[allow(dead_code)]
struct Point<T, U> {
    x: T,
    y: U,
}

///
impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

struct Point1<T> {
    x: T,
    y: T,
}

impl<T> Point1<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point1<f64> {
    fn distance_of_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

//
struct Pair<T> {
    x: T,
    y: T,
}

// 所有泛型T实现方法
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Pair { x, y }
    }
}

// 为实现特定trait的泛型实现方法
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x > self.y {
            println!("the largest number is {}", self.x);
        } else {
            println!("the largest number is {}", self.y);
        }
    }
}

fn main() {
    let x = [4, 3, 0, 9];
    println!("largest: {}", largest(&x));

    let point = Point { x: 3, y: 3.6 };
    println!("point: {:?}", point);

    let point = Point1 { x: 3, y: 4 };
    println!("x: {}", point.x());
    let point = Point1 { x: 4.0, y: 3.0 };
    println!("distance of origin: {:.1}", point.distance_of_origin());

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    let p = Pair::new(3, 5);
    // i32 实现了Display 和 PartialOrd
    p.cmp_display();
}