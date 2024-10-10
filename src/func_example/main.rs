// File       : main.rs
// Time       ：2024/10/8 15:38
// Author     ：sandwich
// version    ：V1.0
// Description：function in rust

use std::io::Write;
use std::str::FromStr;
use rust_learning::func_example::fn_tools::gcd;
// 单元测试, cargo test 调用
#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);
    assert_eq!(gcd(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19), 3 * 11);
}

fn main() {
    let m: u64 = 319;
    let n: u64 = 377;
    println!("{}", gcd(m, n));
    // 获取命令行参数
    let mut numbers = Vec::new();

    for arg in std::env::args().skip(1) {
        numbers.push(u64::from_str(&arg).expect("error parsing argument"));
    }

    if numbers.len() == 0 {
        writeln!(std::io::stderr(), "Usage: gcd NUMBER...").unwrap();
        std::process::exit(1);
    }
    let mut d = numbers[0];
    for &m in &numbers[1..] {
        d = gcd(d, m);
    }
    println!("The greatest common divisor of {:?} is {}", numbers, d);
}