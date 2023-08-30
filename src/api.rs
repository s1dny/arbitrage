use reqwest::blocking::Client;
use serde_json::Value;

pub fn fetch_sports(api_key: String) -> Vec<String> {
    let client = Client::new();

    let sports = client.get(format!("https://api.the-odds-api.com/v4/sports/?apiKey={}", api_key))
        .send()
        .unwrap()
        .json::<Value>()
        .unwrap();

    let mut sports_list = Vec::new();

    for j in 0..sports.as_array().unwrap().len() {
        sports_list.push(sports[j]["key"].to_string().replace('"', ""));
    }

    sports_list
}

pub fn fetch_bets(api_key: String, sport: String) -> Value {
    let client = Client::new();

    let bets = client
        .get(format!("https://api.the-odds-api.com/v4/sports/{}/odds/?regions=au&apiKey={}", sport, api_key))
        .send()
        .unwrap()
        .json::<Value>()
        .unwrap();

    bets
}