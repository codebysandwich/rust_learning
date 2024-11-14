/*
* File       : main.rs
* Time       ：2024/10/31 09:33
* Author     ：sandwich
* version    ：V1.0
* Description：
*/
use std::collections::HashMap;

#[allow(dead_code)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

#[test]
fn test_new_vector() {
    let v = vec![1, 2, 3];
    println!("{:?}", v);
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    println!("{:?}", v);
}

#[test]
fn test_read_vector() {
    let v = vec![1, 2, 3, 4, 5];
    let third = v[2];
    println!("The third element is {}.", third);
    let third = &v[2];
    // 自动解开引用
    println!("The third element is {}.", third);
    println!("The third element is {}.", *third);
    match v.get(2) {
        None => println!("there is no element"),
        Some(third) => println!("The third element is {}.", third),
    }
}

#[test]
fn test_implicit_mut_reference() {
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0]; // immut borrowed here
    // v.push(6); // mut borrowed here(impl push(&mut self, value))
    println!("The first element is: {}", first); // immut clean here
    // after immut
    v.push(6);
    println!("v: {:?}", v);
}

#[test]
fn test_collection_loop() {
    let v = vec![100, 32, 57];
    for i in &v {
        // 自动解引用
        println!("{}", i);
    }
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    println!("after vec: {:?}", v);
}

#[test]
fn test_enum_in_vector() {
    let row = vec![SpreadsheetCell::Int(3),
                   SpreadsheetCell::Text(String::from("blue")),
                   SpreadsheetCell::Float(10.12)];
    for item in row {
        match item {
            SpreadsheetCell::Int(value) => println!("int: {}", value),
            SpreadsheetCell::Float(value) => println!("float: {}", value),
            SpreadsheetCell::Text(text) => println!("text: {}", text),
        }
    }
}

#[test]
fn test_creat_map() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("{:?}", scores);

    let teams = vec!["Blue".to_string(), "Yellow".to_string()];
    let initial_scores = vec![10, 99];
    let scores: HashMap<&String, &i32> = teams.iter().zip(initial_scores.iter()).collect();
    println!("{:?}", scores);
}

#[test]
fn test_ownership_in_map() {
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value); // moved here
    println!("map: {:?}", map);
    // println!("{}", field_value); // borrow moved value
}

#[test]
fn test_get_in_map() {
    let mut map = HashMap::new();
    map.insert("Blue".to_string(), 10);
    map.insert("Yellow".to_string(), 50);

    let team = "Blue".to_string();
    println!("{:?}", map.get(&team));

    for (key, value) in &map {
        println!("{}: {}", key, value);
    }
}

#[test]
fn test_update_map() {
    // 没有插入新值，存在则更新值
    let mut map = HashMap::new();
    map.insert("Blue".to_string(), 10);
    let old = map.insert("Blue".to_string(), 25);
    println!("old value is: {:?}", old);
    println!("map: {:?}", map);
    let mut map = HashMap::new();
    map.insert("Blue".to_string(), 10);

    // 只插入新值，返回现有值的可变引用
    let v1 = map.entry("Yellow".to_string()).or_insert(50);
    println!("{v1}");
    let v2 = map.entry("Blue".to_string()).or_insert(50);
    println!("{v2}");
    println!("scores: {:?}", map);

    // 词频统计
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("counter: {:?}", map);
}

fn main() {
    // new
    let mut map = HashMap::new();
    map.insert("Blue".to_string(), 10);
    map.insert("Yellow".to_string(), 50);
    println!("map: {:?}", map);

    // zip vector
    let teams = vec!["Blue", "Yellow"];
    let scores = vec![10, 50];
    let map: HashMap<_, _> = teams.iter().zip(scores.iter()).collect();
    println!("map: {:?}", map);

    // insert update
    let mut map: HashMap<String, Option<i32>> = HashMap::new();
    map.entry("Red".to_string()).or_insert(Some(30));
    let v = map.entry("Red".to_string()).or_insert(Some(100));
    println!("{v:?}");
    println!("map: {:?}", map);
}