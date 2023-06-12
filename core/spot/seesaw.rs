// goal: to earn coinA by trading coinB
// hand: coinA
// sell: when coinA's price raise. coinA/coinB is greater than therahold1, sell coinA and buy coinB
// buy: when coinA's price fall.coinA/coinB is less than therahold2, sell coinB and buy coinA
//
// abstract: trade source to target by therahold
//
// question: how to forbid the trade when target price sudden fall? so do we need measure the price change rate?

use crate::strategy::Strategy;

// SeesawRatio is a strategy to trade source to target by ratio
// the therahold is the ratio of source/target
struct SeesawRatio {
    source: f64,
    target: f64,
    therahold: f64,
}

// SeesawAbs is a strategy to trade source to target by abs
// the therahold is the distance of source - target
struct SeesawAbs {
    source: f64,
    target: f64,
    therahold: f64,
}
