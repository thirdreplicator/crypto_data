use serde::{Serialize, Deserialize};

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
