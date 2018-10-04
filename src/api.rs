use book::*;
use ticker::*;
use tickers::*;
use trades::*;
use candles::*;
use orders::*;
use account::*;

#[derive(Clone)]
pub struct Ethfinex {
    pub book: Book,
    pub ticker: Ticker,
    pub tickers: Tickers,
    pub trades: Trades,
    pub candles: Candles,
    pub orders: Orders,
    pub account: Account
}

impl Ethfinex {
    pub fn new(api_key: Option<String>, secret_key: Option<String>) -> Self {
        Ethfinex { 
            book: Book::new(),
            ticker: Ticker::new(),
            tickers: Tickers::new(),
            trades: Trades::new(),
            candles: Candles::new(),
            orders: Orders::new(api_key.clone(), secret_key.clone()),
            account: Account::new(api_key.clone(), secret_key.clone())
        }
    }
}