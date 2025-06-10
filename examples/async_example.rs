// examples/async_example.rs

use gateio_rs::Client;
use tokio; // убедитесь, что в Cargo.toml стоит `tokio` в секции [dependencies] для фичи rt-tokio

#[tokio::main]
async fn main() -> gateio_rs::Result<()> {
    // Ключи можно пустые, мы мокаем результат
    let client = Client::new("", "");

    // вызываем асинхронно
    let price = client.ticker("BTC_USDT").await?;
    println!("→ async price = {price}");

    Ok(())
}