use gateio_rs::{api::spot, http::Credentials, ureq::GateHttpClient};
use serde_json::Value;

fn main() -> Result<(), Box<gateio_rs::ureq::Error>> {
    dotenv::dotenv().ok();

    let api_key = std::env::var("GATE_API_KEY").expect("GATE_API_KEY not set");
    let api_secret = std::env::var("GATE_API_SECRET").expect("GATE_API_SECRET not set");
    let credentials = Credentials::new(api_key, api_secret);

    let client = GateHttpClient::default().credentials(credentials.clone());

    // ⚠️  WARNING: This is for cross margin liquidation scenarios
    // Use with extreme caution
    println!("⚠️  WARNING: This creates cross liquidation orders!");
    println!("These are typically used in margin liquidation scenarios.");
    println!("Proceeding in 3 seconds... Press Ctrl+C to abort");
    std::thread::sleep(std::time::Duration::from_secs(3));

    // Create cross liquidation orders
    let orders = vec![
        spot::CrossLiquidateOrder::new("BTC_USDT", "0.01", "45000")
            .text("Cross liquidation order")
            .action_mode("ACK"),
    ];

    let req = spot::create_cross_liquidate_orders(orders);

    let resp = client.send(req)?;
    let body = resp.into_body_str()?;
    let resp_obj: Value = serde_json::from_str(&body).unwrap();
    println!("{:?}", resp_obj);

    Ok(())
}
