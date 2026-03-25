use std::env;

use anyhow::Result;
use dotenv::dotenv;

mod api;
use api::{fetch_events, fetch_sports, Outcome};

const BET_AMOUNT: f64 = 100.0;
const SKIP_BOOKMAKERS: &[&str] = &["betfair_ex_au"];

struct BestOdd {
    bookmaker: String,
    outcome: Outcome,
}

struct Arbitrage {
    bet: f64,
    odds: Vec<BestOdd>,
    stakes: Vec<f64>,
}

impl Arbitrage {
    fn new(odds: Vec<BestOdd>) -> Self {
        let stakes = compute_stakes(&odds);
        Self {
            bet: BET_AMOUNT,
            odds,
            stakes,
        }
    }

    fn payout(&self) -> f64 {
        let total: f64 = self
            .odds
            .iter()
            .zip(&self.stakes)
            .map(|(odd, &stake)| stake * odd.outcome.price)
            .sum();
        total / self.odds.len() as f64 * self.bet
    }

    fn profit(&self) -> f64 {
        self.payout() - self.bet
    }

    fn roi(&self) -> f64 {
        self.profit() * 100.0 / self.bet
    }
}

fn find_best_odds(bookmakers: &[api::Bookmaker]) -> Vec<BestOdd> {
    let outcomes = match bookmakers.iter().find(|b| !b.markets.is_empty()) {
        Some(b) => &b.markets[0].outcomes,
        None => return Vec::new(),
    };

    let mut best: Vec<BestOdd> = outcomes
        .iter()
        .map(|o| BestOdd {
            bookmaker: String::new(),
            outcome: o.clone(),
        })
        .collect();

    // Initialize with lowest possible prices so any real bookmaker wins
    for b in &mut best {
        b.outcome.price = 0.0;
    }

    for bookmaker in bookmakers {
        if SKIP_BOOKMAKERS.contains(&bookmaker.key.as_str()) {
            continue;
        }
        let outcomes = match bookmaker.markets.first() {
            Some(m) => &m.outcomes,
            None => continue,
        };
        for (i, outcome) in outcomes.iter().enumerate().take(best.len()) {
            if outcome.price > best[i].outcome.price {
                best[i].bookmaker = bookmaker.key.clone();
                best[i].outcome = outcome.clone();
            }
        }
    }

    best
}

fn compute_stakes(odds: &[BestOdd]) -> Vec<f64> {
    let sum: f64 = odds.iter().map(|o| 1.0 / o.outcome.price).sum();
    odds.iter().map(|o| (1.0 / o.outcome.price) / sum).collect()
}

fn main() -> Result<()> {
    dotenv().ok();
    let api_key = env::var("API_KEY").expect("API_KEY not set in .env");

    let mut sports = fetch_sports(&api_key)?;
    sports.retain(|s| !s.contains("winner"));

    for sport in &sports {
        println!("{sport}");

        let events = match fetch_events(&api_key, sport) {
            Ok(e) => e,
            Err(err) => {
                eprintln!("Error fetching {sport}: {err}");
                continue;
            }
        };

        for event in &events {
            let best_odds = find_best_odds(&event.bookmakers);
            if best_odds.is_empty() {
                continue;
            }

            let arb = Arbitrage::new(best_odds);
            let roi = arb.roi();

            if roi > 0.0 {
                let odds_summary: Vec<_> = arb
                    .odds
                    .iter()
                    .map(|o| format!("{}: {} @ {:.2}", o.bookmaker, o.outcome.name, o.outcome.price))
                    .collect();
                println!(
                    "ROI: \x1b[32m{roi:.2}%\x1b[0m PROFIT: \x1b[32m${:.2}\x1b[0m [{}]",
                    arb.profit(),
                    odds_summary.join(", ")
                );
            }
        }
    }

    Ok(())
}
