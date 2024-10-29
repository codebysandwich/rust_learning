// File       : main.rs
// Time       ：2024/10/8 09:59
// Author     ：sandwich
// version    ：V1.0
// Description：enum type and match mode


#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
#[allow(dead_code)]
enum UsState {
    Alabama,
    Alaska,
}

#[allow(dead_code)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[test]
fn test_match() {
    fn value_in_cents(coin: Coin) -> u32 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter form {:?}", state);
                25
            }
        }
    }
    value_in_cents(Coin::Quarter(UsState::Alabama));
}

#[test]
fn test_match_option() {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(x) => Some(x + 1),
        }
    }
    let five = Some(5);
    println!("plus Some(5): {:?}", plus_one(five));
    println!("plus None: {:?}", plus_one(None));
}

#[test]
fn test_general_match() {
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }
}

#[test]
fn test_if_let_else() {
    let some_u8_value = Some(3u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }
    // 等效 if let
    if let Some(3) = some_u8_value {
        println!("three");
    }
    let mut count = 0;
    // let coin = Coin::Quarter(UsState::Alabama);
    let coin = Coin::Penny;
    // match 也会发生所有权转移
    match &coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }
    println!("{}", count);
    if let Coin::Quarter(state) = &coin {
        println!("State quarter from {:?}", state);
    } else {
        count += 1;
    }
    println!("{}", count);
}

#[allow(dead_code)]
enum Color {
    Red,
    Green,
    Blue,
    Black,
    Unknown(String),
}

impl Color {
    #[allow(dead_code)]
    fn print_info(&self) {
        match self {
            Color::Red => println!("color is red"),
            Color::Green => println!("color is green"),
            Color::Blue => println!("color is blue"),
            Color::Black => println!("color is black"),
            Color::Unknown(s) => println!("color is {}", *s),
        }
    }

    fn print(my_color: Color) {
        match my_color {
            Color::Red => println!("Red"),
            Color::Green => println!("Green"),
            Color::Blue => println!("Blue"),
            _ => println!("Unknown!"),
        }
    }
}

#[allow(dead_code)]
fn check(x: i32) -> Option<String> {
    if x <= 5 {
        return Some("5".to_string());
    }
    None
}

#[test]
fn test_check() {
    let s = check(12).unwrap_or("Bad Input".to_owned());
    println!("{s}");
    let s = check(13).unwrap_or_default();
    println!("{s}");
    let s = check(23).unwrap_or_else(|| {
        println!("Bad Input Detected");
        "Bad Input".to_string()
    });
    println!("{s}");
}


fn main() {
    let my_color = Color::Red;
    Color::print(my_color);
    let my_color = Color::Black;
    Color::print(my_color); // 所有权已经转移
    let my_color = Color::Unknown("none".to_string());
    my_color.print_info();

    let x = 5;
    match x {
        1..5 => { println!("1"); }

        _ => {}
    }

    let v4 = IpAddrKind::V4;
    let v6 = IpAddrKind::V6;
    println!("{:?}, {:?}", v4, v6);
    let v4 = IpAddr::V4(127, 0, 0, 1);
    let v6 = IpAddr::V6("::1".to_string());
    println!("{:?}, {:?}", v4, v6);
    match v4 {
        IpAddr::V4(a, b, c, d) => {
            println!("{}.{}.{}.{}", a, b, c, d);
        }
        IpAddr::V6(s) => {
            println!("{}", s);
        }
    }
}