# Arbitrage Opportunity Calculator

Uses [The Odds API](https://the-odds-api.com/) to find arbitrage opportunities in sports betting.

## Usage

Clone the repo  
```
git clone https://github.com/k4yp/arbitrage.git
```
Add your API key to the `.env` file  
``` 
echo "API_KEY=your_key_here" > .env
```
Run the project  
```
cargo run
```

## Example results

Will print each sport and any available arbitrage opportunities

### Example:

```
baseball_mlb
ROI: 2.36% PROFIT: $236.59 [("\"sportsbet\"", ("Houston Astros", 1.71)), ("\"unibet\"", ("Boston Red Sox", 2.55))]

```
