/*
* File       : counter.rs
* Time       ：2024/10/21 21:56
* Author     ：sandwich
* version    ：V1.0
* Description：lear mut/reference and Association in struct
*/

#[allow(dead_code)]
pub struct Counter {
    number: i32,
}

#[allow(dead_code)]
/// 实现结构体的关联函数
impl Counter {
    /// 关联函数,用以初始化新的对象
    fn new(number: i32) -> Self {
        Self { number }
    }
    /// 获取当前对象的不可变引用，访问数据
    fn get_number(&self) -> i32 {
        self.number
    }
    /// 通过当前对象的可变引用，修改数据
    fn add(&mut self, increment: i32) {
        self.number += increment;
    }
    /// 获取当前对象的所有权，并进行释放
    fn free(self) {
        println!("free counter number is {}", self.number);
    }
    /// 获取left与right的所有权并生成新的对象
    fn combine(left: Self, right: Self) -> Self {
        Self {
            number: left.number + right.number
        }
    }
}

#[test]
fn test_counter() {
    let mut count = Counter::new(0);
    println!("count number is {}", count.get_number());
    // 修改数据
    count.add(10);
    println!("count number is {}", count.get_number());
    count.free(); // 交出所有权
    // println!("count number is {}", count.get_number()); // borrowed
    let c1 = Counter::new(3);
    let c2 = Counter::new(5);
    let c3 = Counter::combine(c1, c2);
    println!("count number is {}", c3.get_number());
}