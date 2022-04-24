use crypto_data::candle;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://api.binance.com/api/v3/klines?symbol=BTCUSDT&interval=1m&limit=10")
        .await?
        .json::<Vec<candle::Candle>>()
        .await?;
    println!("{:#?}", resp);
    Ok(())
}

