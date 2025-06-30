use serde_json::Value;
use gateio_rs::{
    ureq::GateHttpClient,
    api::spot,
    http::Credentials,
};

fn main() -> Result<(), Box<gateio_rs::ureq::Error>> {
    dotenv::dotenv().ok();
    
    // ⚠️  WARNING: This will cancel ALL open orders!
    println!("⚠️  WARNING: This will cancel ALL open orders!");
    println!("Proceeding in 3 seconds... Press Ctrl+C to abort");
    std::thread::sleep(std::time::Duration::from_secs(3));
    
    let api_key = std::env::var("GATE_API_KEY").expect("GATE_API_KEY not set");
    let api_secret = std::env::var("GATE_API_SECRET").expect("GATE_API_SECRET not set");
    let credentials = Credentials::new(api_key, api_secret);
    
    let client = GateHttpClient::default().credentials(credentials.clone());
    
    // Cancel all open orders (optionally filter by currency_pair)
    let req = spot::cancel_all_open_orders()
        .currency_pair("DUREV_USDT"); // Optional: cancel only for specific pair
    
    let resp = client.send(req)?;
    let body = resp.into_body_str()?;
    let resp_obj: Value = serde_json::from_str(&body).unwrap();
    println!("{:?}", resp_obj);
    
    Ok(())
}