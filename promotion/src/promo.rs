/*
* File       : promotion/src/promo.rs
* Time       ：2025/6/11 14:19
* Author     ：sandwich
* version    ：V1.0
* Description：
*/
use crate::order::Order;
use std::collections::HashSet;

pub fn fidelity_promo(order: &Order) -> f64 {
    if order.customer.fidelity >= 1000 {
        order.total() * 0.05
    } else {
        0.0
    }
}

pub fn bulk_item_promo(order: &Order) -> f64 {
    let mut discount = 0.0;
    // for item in order.cart.iter() {
    //     if item.quantity >= 20 {
    //         discount += item.total() * 0.1;
    //     }
    // }
    discount += order
        .cart
        .iter()
        .filter(|item| item.quantity >= 20)
        .map(|item| item.total() * 0.1)
        .sum::<f64>();

    discount
}

pub fn large_order_promo(order: &Order) -> f64 {
    let distinct_items: HashSet<&String> =
        order.cart.iter().map(|item| &item.product).collect();
    if distinct_items.len() >= 10 {
        return order.total() * 0.07;
    }
    0.0
}

pub fn best_promo(order: &Order) -> f64 {
    *(vec![
        fidelity_promo(order),
        bulk_item_promo(order),
        large_order_promo(order),
    ]
    .iter()
    .max_by(|a, b| a.partial_cmp(b).unwrap())
    .unwrap())
}
