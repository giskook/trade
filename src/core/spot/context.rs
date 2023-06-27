struct Context {
    strategy: Box<dyn Strategy>,
}

impl Context {
    fn new(strategy: Box<dyn Strategy>) -> Self {
        Context { strategy }
    }
    
    fn execute_strategy(&self) {
        self.strategy.execute();
    }
}


// // 定义策略接口
// trait Strategy {
//     fn execute(&self);
// }
// 
// // 实现具体的策略1
// struct ConcreteStrategy1;
// 
// impl Strategy for ConcreteStrategy1 {
//     fn execute(&self) {
//         println!("Executing strategy 1");
//     }
// }
// 
// // 实现具体的策略2
// struct ConcreteStrategy2;
// 
// impl Strategy for ConcreteStrategy2 {
//     fn execute(&self) {
//         println!("Executing strategy 2");
//     }
// }
// 
// // 定义上下文对象
// struct Context {
//     strategy: Box<dyn Strategy>,
// }
// 
// impl Context {
//     fn new(strategy: Box<dyn Strategy>) -> Self {
//         Context { strategy }
//     }
//     
//     fn execute_strategy(&self) {
//         self.strategy.execute();
//     }
// }
// 
// fn main() {
//     // 创建具体策略对象
//     let strategy1 = ConcreteStrategy1;
//     let strategy2 = ConcreteStrategy2;
// 
//     // 创建上下文对象并设置具体策略
//     let context1 = Context::new(Box::new(strategy1));
//     let context2 = Context::new(Box::new(strategy2));
// 
//     // 执行策略
//     context1.execute_strategy();
//     context2.execute_strategy();
// }
// 
