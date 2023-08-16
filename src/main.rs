use reqwest::blocking::Client;
use serde_json::Value;
use std::collections::HashMap;

struct Arbitrage {
    bet: f64,
    round: f64,
    odds: Vec<f64>,
    stakes: Vec<f64>
}

#[derive(Debug)]
struct HashArbitrage {
    bet: f64,
    round: f64,
    odds: HashMap<String, (String, f64)>,
    stakes: HashMap<String, (String, f64)>
}

impl HashArbitrage {
    fn payout(&self) -> f64 {
        let mut total = 0.0;
        for (bookmaker, game) in &self.odds{
            println!("{:?}", self.stakes.get(bookmaker).unwrap().1);
        }
        self.odds.values().next().unwrap().1 * self.stakes.values().next().unwrap().1 * self.bet
    }
}

impl Arbitrage {
    // total payout of the arbitrage including the initial bet
    fn payout(&self) -> f64 {
        let mut total = 0.0;
        for i in 0..self.stakes.len(){
            total += self.stakes[i] * self.odds[i]
        }
        total / self.stakes.len() as f64
    }

    // total payout minus the initial bet
    fn profit(&self) -> f64 {
        Arbitrage::payout(&self) - self.bet
    }

    // return on investment
    fn roi(&self) -> f64 {
        Arbitrage::payout(&self) * 100.0 / self.bet - 100.0
    }
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

    for (bookmaker, game) in &odds{
        sum += 1.0 / game.1
    }
    for (bookmaker, game) in &odds{
        stakes.insert(bookmaker.clone(), (game.0.to_string(), (1.0 / game.1) / sum));
    }
    stakes
}

fn main() {
    // let mut bet = Arbitrage{bet: 50.0,
    //                     round: 0.1,
    //                     odds: optimal.clone(),
    //                     stakes: stakes
    //                    };
    
    // bet.stakes();

    // println!("Odds to bet on: {:?}", optimal.clone());
    // println!("Amount to bet: {:?}", bet.stakes);

    // println!("\nExpected Results\n");

    // println!("Payout: ${:.2}\nProfit: ${:.2}", bet.payout(), bet.profit());
    // println!("ROI: {:.2}%", bet.roi());


    let client = Client::new();

    let bets = client
        .get("https://api.the-odds-api.com/v4/sports/aussierules_afl/odds/?regions=au&apiKey=d443ff82e9e449b15e401e238d5adc8a")
        .send()
        .unwrap()
        .json::<Value>()
        .unwrap();

    for i in 0..5 {
        let mut odds: HashMap<String, ((String, f64), (String, f64))> = HashMap::new();
        for j in 0..9 {
            let bookmaker = &bets[i]["bookmakers"][j]["markets"][0]["outcomes"];
            let name = &bets[i]["bookmakers"][j]["key"];

            if name != "betfair_ex_au"{
                odds.insert(
                    name.to_string(),
                    (
                        (
                            bookmaker[0]["name"].to_string(),
                            bookmaker[0]["price"].as_f64().unwrap()
                        ),
                        (
                            bookmaker[1]["name"].to_string(),
                            bookmaker[1]["price"].as_f64().unwrap()
                        )
                    )
                );
            }
        }

        let optimal = optimal(odds.clone());
        let stakes = stakes(optimal.clone());

        let bets = HashArbitrage{
                                     bet: 100.0,
                                     round: 5.0,
                                     odds: optimal.clone(),
                                     stakes: stakes
                                    };
                                
        println!("Payout: {}", bets.payout());

    }
}