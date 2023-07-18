use crate::base::token;

struct FloatUp {
    src: token::Token,
    dst: token::Token,
    threshold: price::Price,
    sell: token::trade::sell_market_price,
    buy: token::trade:buy_market_price,
}

impl Strategy for FloatUp {
    fn execute(&self) {
        assert!(self.src.price.symbol == self.dst.price.symbol);
        assert!(self.src.price.symbol == self.threshold.symbol);
        let distance = self.dst.sub(self.src);
        if distance.less(self.threshold) {
            self.do();
        }
    }
}

impl FloatUp{
    pub fn set_src(&self, src token::Token){
        self.src = src;
    }
    
    pub fn set_dst(&self, dst token::Token){
        self.dst = dst;
    }

    pub fn set_threshold(&self, threshold price::Price){
        self.threshold = threshold;
    }

    fn do(&self){
        self.sell(self.src.price.symbol);
        self.buy(self.dst.price.symbol);
    }
}
