use crypto_data::candle;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    match candle::request().await {
        Ok(resp) => {
            candle::print_candles(resp);
        },
        _ => println!("Error")
    }
    Ok(())
}

