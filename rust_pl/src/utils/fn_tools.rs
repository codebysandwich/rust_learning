// File       : fn_tools.rs
// Time       ：2024/10/9 17:16
// Author     ：sandwich
// version    ：V1.0
// Description：

pub fn gcd(mut m: u64, mut n: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}
