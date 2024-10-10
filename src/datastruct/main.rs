/**
 * File              : main.rs
 * Author            : sandwich
 * Date              : 2024-02-23 09:07:49
 * Last Modified Date: 2024-08-02 14:04:07
 * Last Modified By  : sandwich
 */

/// hello

fn main() {
    // array 声明与初始化
    let arr1: [i32; 5] = [1, 3, 5, 7, 9];
    println!("{arr1:?}");
    let arr2 = [3; 5];
    println!("{arr2:?}");
    // 索引访问
    println!("arr element: {}", arr1[2]);

    let tup = (1, "str");
    println!("{tup:?}");
    // 访问元组
    println!("tup element: {}", tup.1);
    // 元组模式匹配
    let (x, s) = tup;
    println!("匹配结果: {}, {}", x, s);
}
