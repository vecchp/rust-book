use std::cmp::Ordering;

extern crate time;

#[derive(Debug)]
pub enum Message {
    New,
    Change,
    Delete,
    DeleteThru,
    DeleteFrom
}

#[derive(Debug)]
pub struct PriceUpdate {
    pub time: time::Tm,
    pub action: Message,
    pub price: f64,
    pub quantity: i32,
    pub order_count: i32,
    pub level: i8
}

impl Eq for PriceUpdate {}


impl Ord for PriceUpdate {
    fn cmp(&self, other: &PriceUpdate) -> Ordering {
        self.price.cmp(&other.price)
    }
}

impl PartialOrd for PriceUpdate {
    fn partial_cmp(&self, other: &PriceUpdate) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for PriceUpdate {
    fn eq(&self, other: &PriceUpdate) -> bool {
        self.price == other.price
    }
}

/*
#[derive(Debug, Eq, PartialEq)]
pub struct Bid(pub PriceUpdate);

#[derive(Debug, Eq)]
pub struct Ask(pub PriceUpdate);

impl Ord for Ask {
    fn cmp(&self, other: &Ask) -> Ordering {
        self.price < &other.price
    }
}

impl PartialOrd for Ask {
    fn partial_cmp(&self, other: &Ask) -> Option<Ordering> {
        Some(self.price < &other.price)
    }
}

impl PartialEq for Ask {
    fn eq(&self, other: &Ask) -> bool {
        self.price == other.price
    }
}
*/