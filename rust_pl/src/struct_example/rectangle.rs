/*
* File       : rectangle.rs
* Time       ：2024/10/25 14:36
* Author     ：sandwich
* version    ：V1.0
* Description：Rectangle struct
*/

use super::point;

#[derive(Debug)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

/// 实现Rectangle关联函数,方法
impl Rectangle {
    pub fn square(size: u32) -> Self {
        Self { width: size, height: size }
    }
    pub fn area(&self) -> u32 {
        self.width * self.height
    }
    pub fn can_hold(&self, other: &Self) -> bool {
        self.width > other.width && self.height > other.height
    }
}