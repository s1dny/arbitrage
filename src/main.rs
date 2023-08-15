use thirtyfour::prelude::*;
use tokio;

struct Arbitrage {
    bet: f32,
    round: f32,
    odds: Vec<f32>,
    stakes: Vec<f32>
}

impl Arbitrage {
    fn stakes(&mut self) -> Vec<f32> {
        let mut stakes = Vec::new();
        for i in 0..self.stakes.len() {
            stakes.push((self.stakes[i] * self.bet / self.round).round() * self.round);
        }
        self.stakes = stakes;
        Vec::new()
    }

    // total payout of the arbitrage including the initial bet
    fn payout(&self) -> f32 {
        let mut total = 0.0;
        for i in 0..self.stakes.len(){
            total += self.stakes[i] * self.odds[i]
        }
        total / self.stakes.len() as f32
    }

    // total payout minus the initial bet
    fn profit(&self) -> f32 {
        Arbitrage::payout(&self) - self.bet
    }

    // return on investment
    fn roi(&self) -> f32 {
        Arbitrage::payout(&self) * 100.0 / self.bet - 100.0
    }
}

fn optimal(odds: Vec<Vec<f32>>) -> Vec<f32> {
    let mut optimal = Vec::new();
    for i in 0..odds[0].len() {
        let mut choices = Vec::new();
        for j in 0..odds.len() {
            choices.push(odds[j][i])
        }
        optimal.push(choices.into_iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap());
    }
    optimal
}

fn stakes(odds: Vec<f32>) -> Vec<f32> {
    let mut stakes: Vec<f32> = Vec::new();

    for i in 0..odds.len() {
        let sum: f32 = odds.iter().map(|odd| 1.0 / odd).sum();
        stakes.push((1.0 / odds[i]) / sum);
    }

    stakes
}

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    let mut caps = DesiredCapabilities::chrome();
    caps.set_headless()?;
    let driver = WebDriver::new("http://localhost:9515", caps).await?;

    driver.goto("https://www.ladbrokes.com.au/racing/newcastle/4e4e11b4-25f9-4111-91d6-b60b11305c69").await?;

    let odds = vec![
                    vec![1.28, 3.70], 
                    vec![2.28, 4.15], 
                   ];

    let optimal = optimal(odds);
    let stakes = stakes(optimal.clone());


    let mut bet = Arbitrage{bet: 50.0,
                        round: 5.0,
                        odds: optimal.clone(),
                        stakes: stakes
                       };
    
    bet.stakes();
    println!("Odds to bet on: {:?}", optimal.clone());
    println!("Amount to bet: {:?}", bet.stakes);

    println!("\nExpected Results\n");

    println!("Payout: ${:.2}\nProfit: ${:.2}", bet.payout(), bet.profit());
    println!("ROI: {:.2}%", bet.roi());

    driver.quit().await?;

    Ok(())
}
