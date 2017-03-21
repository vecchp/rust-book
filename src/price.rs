extern crate time;

#[derive(Debug)]
enum Message {
    New,
    Change,
    Delete,
    DeleteThru,
    DeleteFrom
}

#[derive(Debug)]
struct PriceUpdate {
    time: time::Tm,
    action: Message,
    price: f64,
    quantity: i32,
    order_count: i32,
    level: i8
}

#[derive(Debug)]
struct Bid(PriceUpdate);

#[derive(Debug)]
struct Ask(PriceUpdate);
