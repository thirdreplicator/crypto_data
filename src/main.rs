//use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Kline(
  i64,
  String,
  String,
  String,
  String,
  String,
  i64,
  String,
  i64,
  String,
  String,
  String,
);

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://api.binance.com/api/v3/klines?symbol=BTCUSDT&interval=1m&limit=10")
        .await?
        .json::<Vec<Kline>>()
        .await?;
    println!("{:#?}", resp);
    Ok(())
}

