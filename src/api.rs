use anyhow::{Context, Result};
use reqwest::blocking::Client;
use serde::Deserialize;

const BASE_URL: &str = "https://api.the-odds-api.com/v4/sports";

#[derive(Deserialize)]
pub struct Sport {
    pub key: String,
}

#[derive(Deserialize)]
pub struct Event {
    pub bookmakers: Vec<Bookmaker>,
}

#[derive(Deserialize)]
pub struct Bookmaker {
    pub key: String,
    pub markets: Vec<Market>,
}

#[derive(Deserialize)]
pub struct Market {
    pub outcomes: Vec<Outcome>,
}

#[derive(Deserialize, Clone)]
pub struct Outcome {
    pub name: String,
    pub price: f64,
}

pub fn fetch_sports(api_key: &str) -> Result<Vec<String>> {
    let client = Client::new();

    let sports: Vec<Sport> = client
        .get(format!("{BASE_URL}/?apiKey={api_key}"))
        .send()
        .context("Failed to fetch sports")?
        .json()
        .context("Failed to parse sports response")?;

    Ok(sports.into_iter().map(|s| s.key).collect())
}

pub fn fetch_events(api_key: &str, sport: &str) -> Result<Vec<Event>> {
    let client = Client::new();

    let events: Vec<Event> = client
        .get(format!("{BASE_URL}/{sport}/odds/?regions=au&apiKey={api_key}"))
        .send()
        .context("Failed to fetch odds")?
        .json()
        .context("Failed to parse odds response")?;

    Ok(events)
}
