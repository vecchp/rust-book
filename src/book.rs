use std::collections::BinaryHeap;

extern crate price;
use self::price::{Message, Bid, Ask, PriceUpdate};

pub struct Book {
    orders: BinaryHeap<PriceUpdate>
}

impl Book {
    pub fn new() -> Book {
        Book{
            orders: BinaryHeap::new()
        }
    }
}
