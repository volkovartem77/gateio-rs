use gateio_rs::{
    hyper::GateHttpClient,
    api::spot::get_ticker::GetTicker,
    http::request::Credentials,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let credentials = Credentials::from_hmac("your-api-key", "your-secret");

    let client = GateHttpClient::default().credentials(credentials);

    let request = GetTicker::new().symbol("BTC_USDT").timezone("utc8");

    let response = client.send(request).await?;

    println!("{:#}", response.into_body_str().await?);

    Ok(())
}