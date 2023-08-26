use reqwest::blocking::Client;
use serde_json::Value;

struct Arbitrage {
    bet: f64,
    round: f64,
    odds: Vec<(String, (String, f64))>,
    stakes: Vec<(String, (String, f64))>
}

impl Arbitrage {
    fn payout(&self) -> f64 {
        let mut total = 0.0;
        for i in 0..self.odds.len(){
            total += self.stakes[i].1.1 * self.odds[i].1.1;
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

fn optimal(odds: Vec<(String, Vec<(String, f64)>)>) -> Vec<(String, (String, f64))> {
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

fn stakes(odds: Vec<(String, (String, f64))>) -> Vec<(String, (String, f64))> {
    let mut stakes: Vec<(String, (String, f64))> = Vec::new();
    let mut sum = 0.0;

    for (_, game) in &odds{
        sum += 1.0 / game.1
    }
    for (bookmaker, game) in &odds{
        stakes.push((bookmaker.clone(), (game.0.to_string(), (1.0 / game.1) / sum)));
    }
    stakes
}

fn main() {
    let client = Client::new();

    const API_KEY: &str = "be2a7f3768f69b180ee7e91f43f5afde";

    let sports = client.get(format!("https://api.the-odds-api.com/v4/sports/?apiKey={}", API_KEY))
        .send()
        .unwrap()
        .json::<Value>()
        .unwrap();
    
    let mut sports_list = Vec::new();

    for j in 0..sports.as_array().unwrap().len() {
        sports_list.push(sports[j]["key"].to_string().replace('"', ""));
    }

    sports_list.retain(|word| !word.contains("winner"));

    for sport in sports_list {
        println!("{}", sport);
        let bets = client
            .get(format!("https://api.the-odds-api.com/v4/sports/{}/odds/?regions=au&apiKey={}", sport, API_KEY))
            .send()
            .unwrap()
            .json::<Value>()
            .unwrap();

        let bets_len = bets.as_array().unwrap().len();

        for i in 0..bets_len {
            let mut odds: Vec<(String, Vec<(String, f64)>)> = Vec::new();
            for j in 0..10 {
                let bookmaker = &bets[i]["bookmakers"][j]["markets"][0]["outcomes"];
                let name = &bets[i]["bookmakers"][j]["key"];

                let mut event: Vec<(String, f64)> = Vec::new();

                    if name == "betfair_ex_au" { continue };

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

                odds.push((name.to_string(), event));
            }

            let optimal = optimal(odds.clone());
            let stakes = stakes(optimal.clone());

            let bets = Arbitrage {
                                    bet: 100.0,
                                    round: 5.0,
                                    odds: optimal.clone(),
                                    stakes: stakes.clone()
            };
            let roi = bets.roi();
            let color_code = if roi > 0.0 { "\x1b[32m" } else { "\x1b[31m" };
            if roi > 0.0 { println!("ROI: {}{:.2}%\x1b[0m PROFIT: {}${:.2}\x1b[0m {:?}", color_code, roi, color_code, bets.profit(), optimal.clone()); }
        }
    }
}