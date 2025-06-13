/*
* File       : promotion/src/main.rs
* Time       ：2025/6/11 10:30
* Author     ：sandwich
* version    ：V1.0
* Description：
*/

use promotion::order::{Customer, LineItem, Order};
use promotion::promo::{bulk_item_promo, fidelity_promo, large_order_promo};

fn main() {
    let joe = Customer {
        name: "John Doe".to_owned(),
        fidelity: 0,
    };
    let ann = Customer {
        name: "Ann smith".to_owned(),
        fidelity: 1100,
    };

    let cart = vec![
        LineItem {
            product: "banana".to_owned(),
            quantity: 4,
            price: 0.5,
        },
        LineItem {
            product: "apple".to_owned(),
            quantity: 10,
            price: 1.5,
        },
        LineItem {
            product: "watermellon".to_owned(),
            quantity: 5,
            price: 5.0,
        },
    ];

    println!(
        "{}",
        Order {
            customer: joe.clone(),
            cart: cart.clone(),
            promotion: Some(fidelity_promo)
        }
    );
    println!(
        "{}",
        Order {
            customer: ann,
            cart: cart.clone(),
            promotion: Some(fidelity_promo)
        }
    );

    let banana_cart = vec![
        LineItem {
            product: "banana".to_owned(),
            quantity: 30,
            price: 0.5,
        },
        LineItem {
            product: "apple".to_owned(),
            quantity: 10,
            price: 1.5,
        },
    ];
    println!(
        "{}",
        Order {
            customer: joe.clone(),
            cart: banana_cart,
            promotion: Some(bulk_item_promo)
        }
    );

    let large_order: Vec<LineItem> = (1..=10)
        .map(|i| LineItem {
            product: format!("{}", i),
            quantity: 1,
            price: 1.0,
        })
        .collect();

    println!(
        "{}",
        Order {
            customer: joe.clone(),
            cart: large_order.clone(),
            promotion: Some(large_order_promo)
        }
    );
    println!(
        "{}",
        Order {
            customer: joe.clone(),
            cart: cart.clone(),
            promotion: Some(large_order_promo)
        }
    );
}
