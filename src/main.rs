use std::env;

use anyhow::Result;
use dotenv::dotenv;
use indicatif::{ProgressBar, ProgressStyle};

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

    let mut found_any = false;

    let pb = ProgressBar::new(sports.len() as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{bar:30.blue/dim} {pos}/{len} {msg:.dim}")
            .unwrap()
            .progress_chars("██░"),
    );

    for sport in &sports {
        pb.set_message(sport.clone());

        let events = match fetch_events(&api_key, sport) {
            Ok(e) => e,
            Err(err) => {
                pb.suspend(|| eprintln!("Error fetching {sport}: {err}"));
                pb.inc(1);
                continue;
            }
        };

        let mut sport_printed = false;

        for event in &events {
            let best_odds = find_best_odds(&event.bookmakers);
            if best_odds.is_empty() {
                continue;
            }

            let arb = Arbitrage::new(best_odds);
            let roi = arb.roi();

            let same_bookmaker = arb.odds.windows(2).all(|w| w[0].bookmaker == w[1].bookmaker);

            if roi > 0.0 && !same_bookmaker {
                if !sport_printed {
                    pb.suspend(|| println!("\n\x1b[1;36m{sport}\x1b[0m"));
                    sport_printed = true;
                    found_any = true;
                }

                pb.suspend(|| {
                    println!(
                        "  \x1b[1m{} vs {}\x1b[0m",
                        event.home_team, event.away_team
                    );
                    println!(
                        "  ROI \x1b[1;32m{roi:.2}%\x1b[0m  |  Profit \x1b[1;32m${:.2}\x1b[0m on ${:.0} bet",
                        arb.profit(),
                        arb.bet
                    );
                    for (odd, &stake) in arb.odds.iter().zip(&arb.stakes) {
                        println!(
                            "    \x1b[33m{}\x1b[0m  {}: \x1b[1m{:.2}\x1b[0m  (stake ${:.2})",
                            odd.bookmaker, odd.outcome.name, odd.outcome.price, stake * arb.bet
                        );
                    }
                    println!();
                });
            }
        }

        pb.inc(1);
    }

    pb.finish_and_clear();
    if !found_any {
        println!("No arbitrage opportunities found.");
    }

    Ok(())
}
