/*
* File       : integration_test.rs
* Time       ：2024/12/6 14:32
* Author     ：sandwich
* version    ：V1.0
* Description：
*/

use test_example;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(test_example::add_two(2), 4);
}
