pub mod base;
pub mod core;

use base::token::token;
use base::token::price;

fn main() {
    println!("Hello, world! {:?}", base::token::symbol::FIL);
    let fil_price = price::Price::new(base::token::symbol::USDT, 4.1);
    let dot_price = price::Price::new(base::token::symbol::USDT, 4.9);
    let fil = token::Token::new(base::token::symbol::FIL, &fil_price);
    println!("Hello, world! {:?}", fil);
    let dot = token::Token::new(base::token::symbol::DOT, &dot_price);
    let diff = dot.sub(&fil);
    println!("Hello, world! {:?}", diff);
    let therahold = price::Price::new(base::token::symbol::USDT, 0.4);
    let mut stratage = core::spot::seesaw::SeesawAbs::new(&therahold);
    stratage.set_source_coin(&fil);
    stratage.set_target_coin(&dot);
    println!("Hello, stratage  {:?}", stratage);
    stratage.boom();
}
