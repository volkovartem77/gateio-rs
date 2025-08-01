use gateio_rs::{api::spot, http::Credentials, ureq::GateHttpClient};
use serde_json::Value;

fn main() -> Result<(), Box<gateio_rs::ureq::Error>> {
    dotenv::dotenv().ok();

    let api_key = std::env::var("GATE_API_KEY").expect("GATE_API_KEY not set");
    let api_secret = std::env::var("GATE_API_SECRET").expect("GATE_API_SECRET not set");
    let credentials = Credentials::new(api_key, api_secret);

    let client = GateHttpClient::default().credentials(credentials.clone());

    // Test without currency_pair parameter (get all fees)
    let req = spot::get_fee();

    println!("Fetching trading fee rates...");
    let resp = client.send(req)?;
    let body = resp.into_body_str()?;
    let resp_obj: Value = serde_json::from_str(&body).unwrap();
    println!("All trading fee rates: {:?}", resp_obj);

    // Test with specific currency_pair parameter
    let req = spot::get_fee().currency_pair("BTC_USDT");

    println!("\nFetching trading fee rates for BTC_USDT...");
    let resp = client.send(req)?;
    let body = resp.into_body_str()?;
    let resp_obj: Value = serde_json::from_str(&body).unwrap();
    println!("BTC_USDT trading fee rates: {:?}", resp_obj);

    Ok(())
}
