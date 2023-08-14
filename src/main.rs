use thirtyfour::prelude::*;
use tokio;

struct Arbitrage {
    bet: f32,
    odds: Vec<[f32; 2]>,
}

impl Arbitrage {

    // calculates the stakes for each event
    fn stakes(&self) -> Vec<Vec<f32>> {
        let mut bookmaker_stakes = Vec::new();
        for i in 0..self.odds.len() {
            let mut event_stakes = Vec::new();
            for j in 0..self.odds[0].len() {

                let sum: f32 = self.odds.iter().map(|pair| 1.0/ pair[j]).sum();

                event_stakes.push((1.0 / self.odds[i][0]) / sum * self.bet);
            }
            bookmaker_stakes.push(event_stakes);
        }
        bookmaker_stakes
    }

    // chooses the optimal bookmaker for each event
    fn optimal(&self) -> Vec<f32> {
        let mut optimal = Vec::new();
        for i in 0..self.odds.len() {
            let mut choices = Vec::new();
            for j in 0..self.odds[0].len() {
                choices.push(self.odds[j][i])
            }
            optimal.push(choices.into_iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap());
        }
        optimal 
    }

    // total payout of the arbitrage including the initial bet
    fn payout(&self) -> f32 {
        Arbitrage::stakes(&self)[0][0] * self.odds[0][0]
    }

    // total payout minus the initial bet
    fn profit(&self) -> f32 {
        Arbitrage::payout(&self) - self.bet
    }

    // return on investment
    fn roi(&self) -> f32 {
        Arbitrage::profit(&self) / self.bet * 100.0
    }
}

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    let mut caps = DesiredCapabilities::chrome();
    caps.set_headless()?;
    let driver = WebDriver::new("http://localhost:9515", caps).await?;

    driver.goto("https://www.ladbrokes.com.au/racing/newcastle/4e4e11b4-25f9-4111-91d6-b60b11305c69").await?;

    let bet = Arbitrage{bet: 50.0, odds: vec![[8.0, 1.08], [7.5, 1.09]]};
    let optimal = bet.optimal();

    println!("{:?}", bet.stakes());
    println!("Payout: ${:.2}\nProfit: ${:.2}", bet.payout(), bet.profit());
    println!("ROI: {:.2}%", bet.roi());

    driver.quit().await?;

    Ok(())
}