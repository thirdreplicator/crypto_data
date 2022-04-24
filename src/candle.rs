use serde::{Serialize, Deserialize};
use reqwest::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct Candle(
  pub i64,
  pub String,
  pub String,
  pub String,
  pub String,
  pub String,
  pub i64,
  pub String,
  pub i64,
  pub String,
  pub String,
  pub String,
);

pub async fn request() -> Result<Vec<Candle>, Error> {
    let resp = reqwest::get("https://api.binance.com/api/v3/klines?symbol=BTCUSDT&interval=1m&limit=10")
        .await?
        .json::<Vec<Candle>>()
        .await?;
    Ok(resp)
}
