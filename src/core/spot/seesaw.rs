// goal: to earn coinA by trading coinB
// hand: coinA
// sell: when coinA's price raise. coinA/coinB is greater than therahold1, sell coinA and buy coinB
// buy: when coinA's price fall.coinA/coinB is less than therahold2, sell coinB and buy coinA
//
// abstract: trade source to target by therahold
//
// question: how to forbid the trade when target price sudden fall? so do we need measure the price change rate?

use crate::base::token::token;
use crate::base::token::price;

// SeesawAbs is a strategy to trade source to target by abs
// the therahold is the distance of source - target
#[derive(Debug, Default)]
pub struct SeesawAbs<'a> {
    source_coin: &'a token::Token<'a>,
    target_coin: &'a token::Token<'a>,
    therahold: &'a price::Price<'a>,
}

impl<'a> SeesawAbs<'a>{
    pub fn new(therahold: &'a price::Price) -> SeesawAbs<'a>{
        SeesawAbs{
            therahold: therahold,
            ..Default::default()
        }
    }

    pub fn set_source_coin(&mut self, source_coin: &'a token::Token) {
        self.source_coin = source_coin;
    }

    pub fn set_target_coin(&mut self, target_coin: &'a token::Token){
        self.target_coin = target_coin;
    }

    pub fn boom(&self) -> i32{
        if self.source_coin.sub(self.target_coin).less(self.therahold){
            println!("buy {:?}, sell{:?}", self.source_coin, self.target_coin);
            1
        }else{
            0
        }
    }
}
