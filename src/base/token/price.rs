use crate::base::token::symbol;

#[derive(Debug, Default)]
pub struct Price<'a> {
    pub symbol: &'a str,
    pub denom: f64,
}

impl<'a> Price<'a> {
    pub fn new(symbol: &'a str, denom: f64) -> Price<'a> {
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

impl<'a> Default for &'a Price<'a> {
    fn default() -> Self {
        // Return the desired default value for the reference to Price<'a>
        &Price {symbol: symbol::USDT, denom: 0.0}
    }
}
