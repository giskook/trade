pub mod base;
pub mod core;

use base::token::token;

fn main() {
    println!("Hello, world! {:?}", base::token::symbol::FIL);
    let fil = token::Token::new(base::token::symbol::USDT, 4.1);
    let dot = token::Token::new(base::token::symbol::USDT, 4.9);
    let diff = dot.sub(&fil);
    println!("Hello, world! {:?}", diff);
    let therahold = token::Token::new(base::token::symbol::USDT, 0.4);
    let mut stratage = core::spot::seesaw::SeesawAbs::new(&therahold);
    stratage.set_source_coin(&fil);
    stratage.set_target_coin(&dot);
    println!("Hello, stratage  {:?}", stratage);
    stratage.boom();
}
