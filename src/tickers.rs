use client::*;
use errors::*;
use serde_json::from_str;

#[derive(Serialize, Deserialize, Debug)]
pub struct TradingPair {
    pub symbol: String,
    pub bid: f64,
    pub bid_size: f64,
    pub ask: f64,
    pub ask_size: f64,
    pub daily_change: f64,
    pub daily_change_perc: f64,
    pub last_price: f64,
    pub volume: f64,
    pub high: f64,
    pub low: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FundingCurrency {
    pub symbol: String,
    pub frr: f64,
    pub bid: f64,
    pub bid_period: i64,
    pub bid_size: f64,
    pub ask: f64,
    pub ask_period: i64,
    pub ask_size: f64,
    pub daily_change: f64,
    pub daily_change_perc: f64,
    pub last_price: f64,
    pub volume: f64,
    pub high: f64,
    pub low: f64,
}

#[derive(Clone)]
pub struct Tickers {
    client: Client,
}

impl Tickers {
    pub fn new() -> Self {
        Tickers {
            client: Client::new(None, None),
        }
    }

    pub fn funding_currency<S>(&self, symbol: S) -> Result<(Vec<FundingCurrency>)>
    where
        S: Into<String>,
    {
        let endpoint: String = format!("tickers?symbols=f{}", symbol.into());
        let data = self.client.get(endpoint, String::new())?;

        let tickers: Vec<FundingCurrency> = from_str(data.as_str())?;

        Ok(tickers)
    }

    pub fn trading_pair<S>(&self, symbol: S) -> Result<(Vec<TradingPair>)>
    where
        S: Into<String>,
    {
        let symbol_str = symbol.into();
        let mut endpoint = String::new();
        if symbol_str == "ALL" {
            endpoint = format!("tickers?symbols={}", symbol_str);
        } else {
            endpoint = format!("tickers?symbols=t{}", symbol_str);
        }
        let data = self.client.get(endpoint, String::new())?;
        
        let line = data.as_str();
        let start_bytes = 0;
        let mut result = &line[start_bytes..];
        // if there's a < let's check its byte index
        if let Some(end) = result.find("f") {
            // find the first f and back trace to the end of the last trade pair
            let end_offset = 3;
            result = &line[start_bytes.. start_bytes+(end-end_offset)];
        }
        // Close off the vector so we can parse correctly
        let trade_data: String = result.to_owned() + "]";

        let tickers: Vec<TradingPair> = from_str(trade_data.as_str())?;

        Ok(tickers)
    }
}
