use gateio_rs::{api::spot, http::Credentials, ureq::GateHttpClient};
use serde_json::Value;

fn main() -> Result<(), Box<gateio_rs::ureq::Error>> {
    dotenv::dotenv().ok();

    let api_key = std::env::var("GATE_API_KEY").expect("GATE_API_KEY not set");
    let api_secret = std::env::var("GATE_API_SECRET").expect("GATE_API_SECRET not set");
    let credentials = Credentials::new(api_key, api_secret);

    let client = GateHttpClient::default().credentials(credentials.clone());

    // Get ticker for all currency pairs
    let req = spot::get_ticker();

    let resp = client.send(req)?;
    let body = resp.into_body_str()?;
    let resp_obj: Value = serde_json::from_str(&body).unwrap();
    println!("All tickers: {:?}", resp_obj);

    // Get ticker for specific currency pair
    let req = spot::get_ticker().currency_pair("BTC_USDT");

    let resp = client.send(req)?;
    let body = resp.into_body_str()?;
    let resp_obj: Value = serde_json::from_str(&body).unwrap();
    println!("BTC_USDT ticker: {:?}", resp_obj);

    Ok(())
}
