#![deny(unstable_features, unused_must_use, unused_mut, unused_imports, unused_import_braces)]

#[macro_use]
extern crate error_chain;

extern crate hex;
extern crate reqwest;
extern crate ring;
extern crate serde;
#[macro_use]
extern crate serde_json;
extern crate tungstenite;
extern crate url;

#[macro_use]
extern crate serde_derive;

mod account;
pub mod book;
mod candles;
mod client;
mod orders;
pub mod ticker;
pub mod tickers;
pub mod trades;

pub mod api;
pub mod currency;
pub mod errors;
pub mod events;
pub mod pairs;
pub mod precision;
pub mod websockets;
