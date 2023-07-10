use crate::base;
use crate::base::token::price;

#[derive(Debug, Default)]
pub struct Token {
    symbol: String,
    price: price::Price,
}

impl Token {
    pub fn new(symbol: String, price: price::Price) -> Token{
        Token{symbol, price}
    }

    pub fn add(&self, other: &Token) -> price::Price {
        assert!(self.price.symbol == other.price.symbol);
        price::Price::new(self.price.symbol, self.price.denom + other.price.denom)
    }

    pub fn sub(&self, other: &Token) -> price::Price {
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

impl Default for Token {
    fn default() -> Self {
        &Token {
            symbol: base::token::symbol::USDT.to_string(),
            price: price::Price{
                symbol: base::token::symbol::USDT.to_string(), 
                denom: 0.0
            },
        }
    }
}
