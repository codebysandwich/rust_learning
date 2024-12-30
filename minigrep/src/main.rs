/*
* File       : minigrep/src/main.rs
* Time       ：2024/12/8 02:00
* Author     ：sandwich
* version    ：V1.0
* Description：mini grep project main entrance
*/

use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

// fn parse_config(args: &[String]) -> Config {
//     let query = args[1].clone();
//     let filename = args[2].clone();
//     Config { query, filename }
// }

#[test]
fn test_something() {
    let v = vec!["hello".to_string(), "test".to_string()];
    let v1 = &v[1];
    println!("v1: {v1}");
    for i in v {
        println!("i: {i:?}");
    }

    let c = Some("hello".to_string());
    match c {
        None => {}
        Some(val) => if val == "hello" {},
    }
}
