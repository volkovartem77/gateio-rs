#[cfg(feature = "enable-hyper")]
use gateio_rs::{api::spot::GetTicker, http::Credentials, hyper::GateHttpClient};

#[cfg(feature = "enable-hyper")]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let credentials = Credentials::new("your-api-key".to_string(), "your-secret".to_string());

    let client = GateHttpClient::default().credentials(credentials);

    let request = GetTicker::new().currency_pair("BTC_USDT").timezone("utc8");

    let response = client.send(request).await?;

    println!("{:#}", response.into_body_str().await?);

    Ok(())
}

#[cfg(not(feature = "enable-hyper"))]
fn main() {
    println!(
        "This example requires the enable-hyper feature. Run with: cargo run --example async_example --features enable-hyper --no-default-features"
    );
}
