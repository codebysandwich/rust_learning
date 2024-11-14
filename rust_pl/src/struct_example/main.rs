/*
* File       : main.rs
* Time       ：2024/10/24 22:28
* Author     ：sandwich
* version    ：V1.0
* Description：
*/

use rust_pl::utils::rectangle::Rectangle;

#[allow(dead_code)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[allow(dead_code)]
/// 元组结构体
struct Color(i32, i32, i32);
#[allow(dead_code)]
struct Point(i32, i32, i32);

#[allow(dead_code)]
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("rect1 is:\n {:#?}", rect1);
    println!("rect1's area is: {}", rect1.area());
}