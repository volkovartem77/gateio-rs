use gateio_rs::{api::spot, http::Credentials, ureq::GateHttpClient};
use serde_json::Value;

fn main() -> Result<(), Box<gateio_rs::ureq::Error>> {
    dotenv::dotenv().ok();

    let api_key = std::env::var("GATE_API_KEY").expect("GATE_API_KEY not set");
    let api_secret = std::env::var("GATE_API_SECRET").expect("GATE_API_SECRET not set");
    let credentials = Credentials::new(api_key, api_secret);

    let client = GateHttpClient::default().credentials(credentials.clone());

    // Get insurance history with specific currency and business type
    // Using timestamp for last 30 days (example values)
    let now = 1704067200i64; // Example: 2024-01-01 00:00:00 UTC 
    let month_ago = now - (30 * 24 * 60 * 60); // 30 days ago

    let req = spot::get_insurance_history("margin", "USDT", month_ago, now).limit(10);

    let resp = client.send(req)?;
    let body = resp.into_body_str()?;
    let resp_obj: Value = serde_json::from_str(&body).unwrap();
    println!("{:?}", resp_obj);

    Ok(())
}
