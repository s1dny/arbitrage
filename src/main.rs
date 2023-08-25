use reqwest::blocking::Client;
use serde_json::Value;
use std::collections::HashMap;

struct Arbitrage {
    bet: f64,
    round: f64,
    odds: HashMap<String, (String, f64)>,
    stakes: HashMap<String, (String, f64)>
}


impl Arbitrage {
    fn payout(&self) -> f64 {
        if (self.odds.len() < 2) || (self.stakes.len() < 2) {
            return 0.0;
        }
        let mut total = 0.0;
        for (bookmaker, _) in &self.odds{
            total += self.stakes.get(bookmaker).unwrap().1 * self.odds.get(bookmaker).unwrap().1
        }
        total / self.odds.len() as f64 * self.bet
    }

    fn profit(&self) -> f64 {
        Arbitrage::payout(&self) - self.bet
    }

    fn roi(&self) -> f64 {
        Arbitrage::profit(&self) * 100.0 / Arbitrage::payout(&self)
    }
}

fn optimal2(odds: Vec<(String, Vec<(String, f64)>)>) -> Vec<(String, (String, f64))> {
    let mut optimal = vec![(String::new(), (String::new(), 0.0)); odds[0].1.len()];

    for i in 0..odds.len() {
        let bookmaker = &odds[i];
        for j in 0..bookmaker.1.len() {
            if bookmaker.1[j].1 > optimal[j].1.1 {
                optimal[j].0 = bookmaker.0.clone();
                optimal[j].1 = bookmaker.1[j].clone();
            }
        }
    }

    optimal
}

fn optimal(odds: HashMap<String, ((String, f64), (String, f64))>) -> HashMap<String, (String, f64)> {
    let mut team_1 = HashMap::from([(String::new(), (String::new(), 0.0))]);
    let mut team_2 = HashMap::from([(String::new(), (String::new(), 0.0))]);

    for (bookmaker, game) in &odds {
        if game.0.1 > team_1.values().next().unwrap().1 {
            team_1 = HashMap::from([(bookmaker.clone(), (game.0.0.clone(), game.0.1))]);
        }
        if game.1.1 > team_2.values().next().unwrap().1 {
            team_2 = HashMap::from([(bookmaker.clone(), (game.1.0.clone(), game.1.1))]);
        }
    }

    let mut combined = HashMap::new();
    combined.extend(team_1);
    combined.extend(team_2);

    combined
}


fn stakes(odds: HashMap<String, (String, f64)>) -> HashMap<String, (String, f64)>  {
    let mut stakes: HashMap<String, (String, f64)> = HashMap::new();
    let mut sum = 0.0;

    for (_, game) in &odds{
        sum += 1.0 / game.1
    }
    for (bookmaker, game) in &odds{
        stakes.insert(bookmaker.clone(), (game.0.to_string(), (1.0 / game.1) / sum));
    }
    stakes
}

fn main() {
    let client = Client::new();
    // rugbyleague_nrl
    // aussierules_afl 
    let bets = client
        .get("https://api.the-odds-api.com/v4/sports/boxing_boxing/odds/?regions=au&apiKey=d443ff82e9e449b15e401e238d5adc8a")
        .send()
        .unwrap()
        .json::<Value>()
        .unwrap();

    for i in 0..1 {
        let mut odds: HashMap<String, ((String, f64), (String, f64))> = HashMap::new();

        let mut odds2: Vec<(String, Vec<(String, f64)>)> = Vec::new();
        for j in 0..10 {
            let bookmaker = &bets[i]["bookmakers"][j]["markets"][0]["outcomes"];
            let name = &bets[i]["bookmakers"][j]["key"];

            let mut event: Vec<(String, f64)> = Vec::new();

                if name != "betfair_ex_au" {
                    if *bookmaker != Value::Null {
                        for i in 0..bookmaker.as_array().unwrap().len() {
                            match bookmaker[i]["name"].as_str() {
                                Some(outcome) => {
                                    if let Some(price) = bookmaker[i]["price"].as_f64() {
                                        event.push((outcome.to_string(), price));
                                    }
                                }
                                None => {
                                    eprintln!("Error: Failed to parse name at index {}", i);
                                }
                            }
                        }
                    }

                    let outcome_0 = bookmaker[0]["name"].as_str();
                    let price_0 = bookmaker[0]["price"].as_f64();

                    let outcome_1 = bookmaker[1]["name"].as_str();
                    let price_1 = bookmaker[1]["price"].as_f64();

                    if let (Some(outcome_0), Some(price_0), Some(outcome_1), Some(price_1)) = (outcome_0, price_0, outcome_1, price_1) {
                        odds.insert(
                            name.to_string(),
                            (
                                (outcome_0.to_string(), price_0),
                                (outcome_1.to_string(), price_1)
                            )
                        );
                    } else {
                        continue;
                    }
                }
            odds2.push((name.to_string(), event));
        }

        let optimal2 = optimal2(odds2.clone());

        println!("{:?}", optimal2);

        let optimal = optimal(odds.clone());
        let stakes = stakes(optimal.clone());

        let bets = Arbitrage{
                                bet: 500.0,
                                round: 5.0,
                                odds: optimal.clone(),
                                stakes: stakes
                            };

        let roi = bets.roi();
        let color_code = if roi > 0.0 { "\x1b[32m" } else { "\x1b[31m" };

        println!("ROI: {}{:.2}%\x1b[0m PROFIT: {}${:.2}\x1b[0m", color_code, roi, color_code, bets.profit());
    }
}