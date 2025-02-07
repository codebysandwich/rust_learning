/*
* File       : rust_pl/src/iterator_and_closure/main.rs
* Time       ：2024/12/24 16:19
* Author     ：sandwich
* version    ：V1.0
* Description：
*/

use std::thread;
use std::time::Duration;

#[allow(dead_code)]
fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    caculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(caculation: T) -> Cacher<T> {
        Cacher {
            caculation,
            value: None,
        }
    }
    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            None => {
                let v = (self.caculation)(arg);
                self.value = Some(v);
                v
            }
            Some(v) => v,
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    // let expensive_result =
    //     simulated_expensive_calculation(intensity);

    // let expensive_closure = |num| -> u32 {
    //     println!("calculating slowly...");
    //     thread::sleep(Duration::from_secs(2));
    //     num
    // };
    // let expensive_closure_in = expensive_closure(intensity);

    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_result.value(intensity)
        );
        println!(
            "do {} situps",
            expensive_result.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

#[derive(PartialEq, Debug)]
#[allow(dead_code)]
struct Shoe {
    size: u32,
    style: String,
}

#[allow(dead_code)]
fn shoes_in_my_size(
    shoes: Vec<Shoe>,
    shoe_size: u32,
) -> Vec<Shoe> {
    shoes
        .into_iter()
        .filter(|s| s.size == shoe_size)
        .collect()
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Self {
        Self { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < 6 {
            return Some(self.count);
        } else {
            None
        }
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;
    generate_workout(
        simulated_user_specified_value,
        simulated_random_number,
    );
}

#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);
    let _v1 = c.value(1);
    let v2 = c.value(2);
    assert_eq!(v2, 1);
}

#[test]
fn closure_inner_catch() {
    let x = 4;
    let equal_to_x = |z| z == x;
    let y = 4;
    assert!(equal_to_x(y));
}

#[test]
fn test_move_in_closure() {
    let x = vec![1, 2, 3];
    let equal_to_x = move |z| z == x;
    // println!("can't use x here: {:?}", x);
    let y = vec![1, 2, 3];
    assert!(equal_to_x(y));
}

#[test]
fn test_iterator() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    for val in v1_iter {
        println!("Got: {}", val);
    }
}

#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter();
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
    for item in v1_iter {
        // next 已经消耗了迭代器
        println!("Got: {}", item);
    }
    for item in v1.iter() {
        println!("Got: {}", item);
    }

    // 获取v1 所有权进行迭代
    for item in v1.into_iter() {
        println!("Got: {}", item);
    }

    // 对迭代原对象进行修改
    let mut v1 = vec![1, 2, 3];
    for item in v1.iter_mut() {
        *item += 1;
    }
    println!("{:?}", v1);
}

#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    assert_eq!(total, 6);
}

#[test]
fn iterator_map() {
    let v1 = vec![1, 2, 3];
    let v2: Vec<i32> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);
}

#[test]
fn filters_by_size() {
    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];
    let in_my_size = shoes_in_my_size(shoes, 10);
    assert_eq!(
        in_my_size,
        vec![
            Shoe {
                size: 10,
                style: String::from("sneaker")
            },
            Shoe {
                size: 10,
                style: String::from("boot")
            }
        ]
    );
}

#[test]
fn calling_next_directly() {
    let mut count = Counter::new();
    assert_eq!(count.next(), Some(1));
    assert_eq!(count.next(), Some(2));
    assert_eq!(count.next(), Some(3));
    assert_eq!(count.next(), Some(4));
    assert_eq!(count.next(), Some(5));
    assert_eq!(count.next(), None);
}

#[test]
fn using_other_iterator_trait_methods() {
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    for (a, b) in Counter::new().zip(Counter::new().skip(1))
    {
        println!("{} * {} = {}", a, b, a * b);
    }
    assert_eq!(18, sum);
}
