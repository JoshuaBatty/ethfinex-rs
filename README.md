[![MIT licensed](https://img.shields.io/badge/License-MIT-blue.svg)](./LICENSE-MIT)
[![Apache-2.0 licensed](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](./LICENSE-APACHE)

# ethfinex-rs

Unofficial Rust Library for the [Ethfinex API V2](https://bitfinex.readme.io/v2/docs/getting-started)

# Risk Warning

It is a personal project, use at your own risk. I will not be responsible for your investment losses.
Cryptocurrency investment is subject to high market risk.

# Usage

Add this to your Cargo.toml

```toml
[dependencies]
ethfinex = { git = "https://github.com/JoshuaBatty/ethfinex-rs.git" }
```

## PUBLIC ENDPOINTS

Ticker, Trades, Book, Candles, see [example](https://github.com/JoshuaBatty/ethfinex-rs/blob/master/examples/src/public_endpoints.rs)

## PRIVATE ENDPOINTS

Wallets, Orders, Trades, Margin and Funding Info, see [example](https://github.com/JoshuaBatty/ethfinex/blob/master/examples/src/private_endpoints.rs)

## PUBLIC CHANNELS (WEBSCOKETS)

Ticker, Trades, Book, Raw Book, Candles, see [example](https://github.com/JoshuaBatty/ethfinex-rs/blob/master/examples/src/public_channels.rs)
