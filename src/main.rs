extern crate time;

mod book;
use book::{Book};

fn main() {
    let book = Book::new();
   /*let t = time::now_utc(;);
   let bid_price = Bid(PriceUpdate{
       time: time::now_utc(),
       action: Message::New,
       price: 1.0,
       quantity: 100,
       order_count: 100,
       level: 1
    });

    let ask_price = Ask(PriceUpdate{
       time: time::now_utc(),
       action: Message::New,
       price: 1.0,
       quantity: 100,
       order_count: 100,
       level: 1
    });

   
     println!("{:?} {:?}", bid_price, ask_price);
    */
}
