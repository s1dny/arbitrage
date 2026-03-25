# Arbitrage Opportunity Calculator

A CLI tool that scans live sports betting odds across multiple bookmakers to find guaranteed-profit arbitrage opportunities using [The Odds API](https://the-odds-api.com/)

## How It Works

An arbitrage opportunity exists when different bookmakers disagree on the odds enough that you can bet on **all** outcomes and still profit regardless of the result. This tool:

1. Fetches all active sports from The Odds API
2. Pulls live odds from bookmakers for each sport
3. Finds the best available price for each outcome across all bookmakers
4. Calculates whether the combined implied probabilities fall below 100% (indicating an arbitrage)
5. Displays the ROI, profit, and optimal stake distribution

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (1.56+)
- A free API key from [The Odds API](https://the-odds-api.com/)

## Setup

```sh
git clone https://github.com/k4yp/arbitrage.git
cd arbitrage
echo "API_KEY=your_key_here" > .env
cargo run
```

## Configuration

| Constant | Location | Default | Description |
|---|---|---|---|
| `BET_AMOUNT` | `src/main.rs` | `100.0` | Base bet amount used to calculate profit |
| `SKIP_BOOKMAKERS` | `src/main.rs` | `["betfair_ex_au"]` | Bookmakers excluded from scanning (e.g. exchanges) |

## Example Output

```
██████████████████████████████ 42/42

basketball_nba
  Lakers vs Celtics
  ROI 2.36%  |  Profit $2.36 on $100 bet
    sportsbet  Lakers: 1.71  (stake $59.88)
    unibet     Celtics: 2.55  (stake $40.12)
```

When no opportunities are found:

```
██████████████████████████████ 42/42
No arbitrage opportunities found.
```
