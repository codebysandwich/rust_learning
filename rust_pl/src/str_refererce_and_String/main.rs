// File       : main.rs
// Time       ：2024/10/6 17:31
// Author     ：sandwich
// version    ：V1.0
// Description：

fn print(message: &str) {
    println!("{message}");
}

fn print_string(message: &String) {
    println!("{message}");
}

fn get_first_word(sentence: &str) -> &str {
    for (i, item) in sentence.bytes().enumerate() {
        if item == b' ' {
            return &sentence[0..i];
        }
    }
    &sentence[..]
}

fn get_str() -> &'static str {
    return "this is test";
}

#[test]
fn test_addr_with_string() {
    let s = String::from("hello");
    println!("addr of s is: {:p}", &s);
    let p_s = &s;
    println!("addr value in &s is: {:p}", p_s as *const String);
    println!("addr of &s is: {:p}", &p_s);
}
#[test]
fn test_push_str() {
    let mut s = "foo".to_string();
    s.push_str("bar");
    println!("{s}");
    s.push_str(&"fxx".to_string());
    println!("{s}");

    let s1 = String::from("hello, ");
    let s2 = String::from("world");
    let s3 = s1 + &s2;
    println!("{s3}");
    // println!("{s1}"); // s1已经转移
}

fn main() {
    let s = "hello";
    let s1 = "hello rust".to_string();
    print(s);
    print(&s1);

    // print_String(s);  // excepted &String error
    print_string(&s1);

    let res = get_first_word(&s1);
    println!("{res}");

    // let s2 = s1;  // s1 has borrowed

    println!("{}", get_str());

    let len = String::from("Здравствуйте").len();
    println!("{len}");
    let len = String::from("3дравствуйте").len();
    println!("{len}");
}