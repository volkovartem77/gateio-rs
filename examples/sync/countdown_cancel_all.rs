use gateio_rs::{api::spot, http::Credentials, ureq::GateHttpClient};
use serde_json::Value;

fn main() -> Result<(), Box<gateio_rs::ureq::Error>> {
    dotenv::dotenv().ok();

    // ⚠️  WARNING: This will set a countdown timer to cancel ALL orders!
    println!("⚠️  WARNING: This will set a countdown timer to cancel ALL orders!");
    println!("Proceeding in 3 seconds... Press Ctrl+C to abort");
    std::thread::sleep(std::time::Duration::from_secs(3));

    let api_key = std::env::var("GATE_API_KEY").expect("GATE_API_KEY not set");
    let api_secret = std::env::var("GATE_API_SECRET").expect("GATE_API_SECRET not set");
    let credentials = Credentials::new(api_key, api_secret);

    let client = GateHttpClient::default().credentials(credentials.clone());

    // Set countdown timer to cancel all orders after 60 seconds
    let timeout_seconds = 60;
    let req = spot::countdown_cancel_all(timeout_seconds).credentials(credentials);
    // .currency_pair("BTC_USDT"); // Optional: apply only to specific pair

    let resp = client.send(req)?;
    let body = resp.into_body_str()?;
    let resp_obj: Value = serde_json::from_str(&body).unwrap();
    println!("Countdown timer set: {:?}", resp_obj);

    Ok(())
}
