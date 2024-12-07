/*
* File       : inner_test.rs
* Time       ：2024/12/6 15:11
* Author     ：sandwich
* version    ：V1.0
* Description：
*/

#[allow(dead_code)]
fn inner_mod_func() -> i32 {
    100
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_inner_mod_func() {
        assert_eq!(inner_mod_func(), 100);
    }
}
