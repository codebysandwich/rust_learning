/*
* File       : main.rs
* Time       ：2024/11/5 09:32
* Author     ：sandwich
* version    ：V1.0
* Description：
*/

use std::fs::File;
#[allow(unused_imports)]
use std::io::{ErrorKind, Read, Error};

#[test]
fn test_panic() {
    // panic!("crash and burn");
    let v = vec![1, 2, 3];
    println!("{}", v[99]);
}

#[test]
fn test_file_open_panic() {
    let f = File::open("hello.txt");
    #[allow(unused_variables)]
    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("There was a problem opening the file: {:?}", error)
    };
}

#[test]
fn test_handle_err() {
    let f = File::open("hello.txt");
    #[allow(unused_variables)]
    let f = f.unwrap_or_else(|error| match error.kind() {
        ErrorKind::NotFound => match File::create("hello.txt") {
            Ok(file) => file,
            Err(error) => panic!("Tried to create file but there was a problem: {:?}",
                                 error),
        },
        other_error => panic!("There was a problem opening the file: {:?}",
                              other_error),
    });
}
#[test]
fn test_better_handle_err() {
    #[allow(unused_variables)]
    let f = File::open("hello.txt").map_err(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Tried to create file but there was a problem: {:?}", error)
            })
        } else {
            panic!("There was a problem opening the file: {:?}", error);
        }
    });
}

#[test]
fn test_unwrap_except() {
    // let f = File::open("hello.txt").unwrap();
    #[allow(unused_variables)]
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}

#[allow(dead_code)]
fn read_username_from_file() -> Result<String, Error> {
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(error) => return Err(error),
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(error) => Err(error),
    }
}

#[allow(dead_code)]
fn read_username_from_file_sample() -> Result<String, Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

#[allow(dead_code)]
fn read_username_from_file_chain() -> Result<String, Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

#[test]
fn test_return_err() {
    match read_username_from_file() {
        Ok(s) => { println!("user info: {}", s); }
        Err(err) => { println!("Err info: {}", err); }
    }

    match read_username_from_file_sample() {
        Ok(s) => { println!("user info: {}", s); }
        Err(err) => { println!("Err info: {}", err); }
    }

    match read_username_from_file_chain() {
        Ok(s) => { println!("user info: {}", s); }
        Err(err) => { println!("Err info: {}", err); }
    }
}

fn main() {}