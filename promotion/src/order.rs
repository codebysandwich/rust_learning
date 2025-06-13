/*
* File       : promotion/src/order.rs
* Time       ：2025/6/11 11:45
* Author     ：sandwich
* version    ：V1.0
* Description：
*/
use std::fmt::Display;

#[derive(Clone)]
pub struct Customer {
    pub name: String,
    pub fidelity: i32,
}

impl Customer {
    pub fn new(name: String, fidelity: i32) -> Self {
        Self { name, fidelity }
    }
}

#[derive(Clone)]
pub struct LineItem {
    pub product: String,
    pub quantity: i32,
    pub price: f64,
}

impl LineItem {
    pub fn new(product: String, quantity: i32, price: f64) -> Self {
        Self {
            product,
            quantity,
            price,
        }
    }

    pub fn total(&self) -> f64 {
        self.price * self.quantity as f64
    }
}

pub struct Order {
    pub customer: Customer,
    pub cart: Vec<LineItem>,
    pub promotion: Option<fn(&Order) -> f64>,
}

impl Order {
    // pub fn new(
    //     customer: Customer,
    //     cart: Vec<LineItem>,
    //     promotion: Option<fn(&Order) -> f64>,
    // ) -> Self {
    //     Self {
    //         customer,
    //         cart,
    //         promotion,
    //     }
    // }

    pub fn total(&self) -> f64 {
        let mut total = 0.0;
        for item in self.cart.iter() {
            total += item.quantity as f64 * item.price;
        }
        total
    }

    pub fn due(&self) -> f64 {
        match self.promotion {
            None => self.total(),
            Some(promo) => self.total() - promo(self),
        }
    }
}

impl Display for Order {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            format!("<Order total: {0:.2} due: {1}>", self.total(), self.due())
        )
    }
}
