// File       : main.rs
// Time       ：2024/9/30 14:21
// Author     ：sandwich
// version    ：V1.0
// Description：

fn get_string(s: String) {
    println!("{s}");
}

fn main() {
    // 基本数据类型，array，tuple赋值时都是执行copy特性
    let mut arr = [1, 3, 5];
    let mut arr_1 = arr;
    arr[0] = 3;
    arr_1[0] = 99;
    println!("after change arr: {:?}", arr);
    println!("after change arr_1: {:?}", arr_1);

    for item in &arr_1 {
        println!("element: {item}");
    }

    // 复杂类型赋值执行move操作
    let s = String::from("hello");
    let s1 = s; // move
    // println!("{s}"); // borrow of moved value
    println!("{s1}");

    get_string(s1);
    // println!("{s1}"); // s1 move to fn get_string
}