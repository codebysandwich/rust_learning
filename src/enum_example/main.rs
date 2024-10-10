// File       : main.rs
// Time       ：2024/10/8 09:59
// Author     ：sandwich
// version    ：V1.0
// Description：enum type and match mode

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


fn main() {
    let my_color = Color::Red;
    Color::print(my_color);
    let my_color = Color::Black;
    Color::print(my_color); // 所有权已经转移
    let my_color = Color::Unknown("none".to_string());
    my_color.print_info();
}