use crate::base;

#[derive(Debug, Default)]
pub struct Token<'a> {
    symbol: &'a str,
    denom: f64,
}

impl<'a> Token<'a>{
    pub fn new(symbol: &str, denom: f64) -> Token{
        Token{symbol, denom}
    }

    pub fn add(&self, other: &Token) -> Token{
        assert!(self.symbol == other.symbol);
        Token::new(self.symbol, self.denom + other.denom)
    }

    pub fn sub(&self, other: &Token) -> Token{
        assert!(self.symbol == other.symbol);
        Token::new(self.symbol, self.denom - other.denom)
    }

    pub fn greater(&self, other: &Token) -> bool{
        assert!(self.symbol == other.symbol);
        if self.denom > other.denom{
            true
        } else{
            false
        }
    }

    pub fn less(&self, other: &Token) -> bool{
        assert!(self.symbol == other.symbol);
        if self.denom < other.denom{
            true
        }else{
            false
        }
    }

    pub fn equal(&self, other: &Token) -> bool{
        assert!(self.symbol == other.symbol);
        if self.denom == other.denom{
            true
        }else{
            false
        }
    }
}

impl<'a> Default for &'a Token<'a> {
    fn default() -> Self {
        // Return the desired default value for the reference to Token<'a>
        &Token {symbol: base::token::symbol::USDT, denom: 0.0}
    }
}
