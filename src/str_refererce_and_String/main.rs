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
}