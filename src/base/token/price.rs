use crate::base::token::symbol;

#[derive(Debug, Default)]
pub struct Price {
    pub symbol: String,
    pub denom: f64,
}

impl Price {
    pub fn new(symbol: String, denom: f64) -> Price<'a> {
        Price{symbol, denom}
    }

    pub fn less(&self, other: &Price) -> bool{
        assert!(self.symbol == other.symbol);
        if self.denom < other.denom{
            true
        }else{
            false
        }
    }

}

impl Default for Price {
    fn default() -> Self {
        // Return the desired default value for the reference to Price<'a>
        &Price {symbol: symbol::USDT.to_string(), denom: 0.0}
    }
}
