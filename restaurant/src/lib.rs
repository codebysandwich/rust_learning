// pub fn add(left: u64, right: u64) -> u64 {
//     left + right
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }

use self::front_of_house::hosting;

#[allow(dead_code)]
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }
    pub mod serving {
        fn take_order() {}
        pub fn serve_order() {}
        fn take_payment() {}
    }
}

#[allow(dead_code)]
mod back_of_house {
    use crate::eat_at_restaurant;

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }
    impl Breakfast {
        pub fn summer(toast: &str) -> Self {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: "peaches".to_string(),
            }
        }
    }
    // 枚举变体无需单独制定访问权限
    pub enum Appetizer {
        Soup,
        Salad,
    }
    fn fix_incorrect_order() {
        cook_order();
        super::front_of_house::serving::serve_order();
        eat_at_restaurant();
    }
    fn cook_order() {}
}

pub fn eat_at_restaurant() {
    // 绝对路径
    // crate::front_of_house::hosting::add_to_waitlist();
    // 相对路径
    // self::front_of_house::hosting::add_to_waitlist();

    //通过use引入
    hosting::add_to_waitlist();

    // 黑麦面包作为夏季早餐
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // 修改公共字段
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_breakfast() {
        eat_at_restaurant();
    }
}