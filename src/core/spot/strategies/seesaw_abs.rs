struct FloatUp {
    src: token::Token,
    dst: token::Token,
    threshold: price::Price,
}

impl Strategy for FloatUp {
    fn execute(&self) {
        assert!(self.src.price.symbol == self.dst.price.symbol);
        assert!(self.src.price.symbol == self.threshold.symbol);

    }
}

impl FloatUp{
    pub fn set_src(&self, src token::Token){
        self.src = src;
    }
    
    pub fn set_dst(&self, dst token::Token){
        self.dst = dst;
    }

    pub fun set_threshold(&self, threshold price::Price){
        self.threshold = threshold;
    }
}
