use serde_json::Value;
use gateio_rs::{
    ureq::GateHttpClient,
    api::spot,
    http::Credentials,
};

fn main() -> Result<(), Box<gateio_rs::ureq::Error>> {
    dotenv::dotenv().ok();
    
    let api_key = std::env::var("GATE_API_KEY").expect("GATE_API_KEY not set");
    let api_secret = std::env::var("GATE_API_SECRET").expect("GATE_API_SECRET not set");
    let credentials = Credentials::new(api_key, api_secret);
    
    let client = GateHttpClient::default().credentials(credentials.clone());
    
    // Get candlestick data for BTC_USDT
    let req = spot::get_candlesticks("BTC_USDT")
        .interval("1h")
        .limit(100);
    
    let resp = client.send(req)?;
    let body = resp.into_body_str()?;
    let resp_obj: Value = serde_json::from_str(&body).unwrap();
    println!("{:?}", resp_obj);
    
    Ok(())
}