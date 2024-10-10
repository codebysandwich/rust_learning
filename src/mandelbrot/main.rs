// File       : main.rs
// Time       ：2024/10/10 12:03
// Author     ：sandwich
// version    ：V1.0
// Description：

extern crate num;

use std::str::FromStr;
use num::Complex;

fn square_loop(mut x: f64) {
    loop {
        x = x * x;
    }
}

fn square_add_loop(c: f64) {
    let mut x = 0.;
    loop {
        x = x * x + c;
    }
}

fn complex_square_add_loop(c: Complex<f64>) {
    let mut z = Complex { re: 0., im: 0. };
    loop {
        z = z * z + c;
    }
}

/// 确定c是否属于曼德布洛特集合，最多循环limit次
///
/// 如果c不是成员，返回Some(i),i是在z离开以原点为圆心、半径为2的圆时的循环次数
/// 如果c是成员()，返回None
fn escape_time(c: Complex<f64>, limlt: u32) -> Option<u32> {
    let mut z = Complex { re: 0., im: 0. };
    for i in 0..limlt {
        z = z * z + c;
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
    }
    None
}

fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => {
            match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
                (Ok(l), Ok(r)) => Some((l, r)),
                _ => None,
            }
        }
    }
}

fn parse_complex(s: &str) -> Option<Complex<f64>> {
    match parse_pair(s, ',') {
        Some((re, im)) => Some(Complex { re, im }),
        _ => None,
    }
}

fn pixel_to_point(bounds: (usize, usize), pixel: (usize, usize),
                  upper_left: Complex<f64>, lower_right: Complex<f64>) -> Complex<f64> {
    let (width, height) = (lower_right.re - upper_left.re, 
                           upper_left.im - lower_right.im);
    Complex {
        re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
        im: upper_left.im - pixel.0 as f64 * height / bounds.1 as f64 
    }
}

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("", ','), None);
    assert_eq!(parse_pair::<i32>("10,", ','), None);
}

#[test]
fn test_parse_complex() {}

fn main() {}