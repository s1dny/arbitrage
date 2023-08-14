struct Arbitrage {
    bet: f32,
    odd_1: f32,
    odd_2: f32,
}

impl Arbitrage {
    fn stake_1(&self) -> f32 {
        self.odd_2 * self.bet / (self.odd_1 + self.odd_2)
    }

    fn stake_2(&self) -> f32 {
        self.odd_1  * self.bet / (self.odd_1 + self.odd_2) 
    }

    fn payout(&self) -> f32 {
        Arbitrage::stake_1(&self) * self.odd_1
    }

    fn profit(&self) -> f32 {
        Arbitrage::stake_1(&self) * self.odd_1 - self.bet
    }

    fn roi(&self) -> f32 {
        (Arbitrage::payout(&self) / self.bet - 1.0) * 100.0
    }
}


fn main() {
    let bet = Arbitrage{bet: 100.0, odd_1: 1.2, odd_2: 2.1};
    println!("Bet 1: ${:.2}\nBet 2: ${:.2}", bet.stake_1(), bet.stake_2());
   
    println!("Payout: ${:.2}\nProfit: ${:.2}", bet.payout(), bet.profit());

    println!("ROI: {:.2}%", bet.roi());
}