use crate::base;
use crate::base::token::price;

#[derive(Debug, Default)]
pub struct Token<'a> {
    symbol: &'a str,
    price: &'a price::Price<'a>,
}

impl<'a> Token<'a>{
    pub fn new(symbol: &'a str, price: &'a price::Price<'a>) -> Token<'a>{
        Token{symbol, price}
    }

    pub fn add(&self, other: &Token) -> price::Price<'a> {
        assert!(self.price.symbol == other.price.symbol);
        price::Price::new(self.price.symbol, self.price.denom + other.price.denom)
    }

    pub fn sub(&self, other: &Token) -> price::Price<'a>{
        assert!(self.price.symbol == other.price.symbol);
        price::Price::new(self.price.symbol, self.price.denom - other.price.denom)
    }

    pub fn greater(&self, other: &Token) -> bool{
        assert!(self.price.symbol == other.price.symbol);
        if self.price.denom > other.price.denom{
            true
        } else{
            false
        }
    }

    pub fn less(&self, other: &Token) -> bool{
        assert!(self.price.symbol == other.price.symbol);
        if self.price.denom < other.price.denom{
            true
        }else{
            false
        }
    }

    pub fn equal(&self, other: &Token) -> bool{
        assert!(self.price.symbol == other.price.symbol);
        if self.price.denom == other.price.denom{
            true
        }else{
            false
        }
    }
}

impl<'a> Default for &'a Token<'a> {
    fn default() -> Self {
        &Token {
            symbol: base::token::symbol::USDT,
            price: &price::Price{
                symbol: base::token::symbol::USDT, 
                denom: 0.0
            },
        }
    }
}
