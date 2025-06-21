use base64::Engine as _;
use hmac::{Hmac, Mac};
use sha2::Sha512;
use std::error::Error;
use hmac::digest::Digest;
use serde_json::{Map, Value};

pub fn sign_hmac(method: &str, path: &str, query: &str, payload: &str, timestamp: &str, key: &str) -> Result<String, Box<dyn Error>> {

    // 1. SHA512(payload)
    let hashed_payload = hex::encode(Sha512::digest(payload.as_bytes()));

    // 2. Build canonical string
    let s = format!(
        "{}\n{}\n{}\n{}\n{}",
        method,
        path,
        query,
        hashed_payload,
        timestamp
    );

    // 3. HMAC-SHA512(secret, s)
    let mut mac = Hmac::<Sha512>::new_from_slice(key.to_string().as_bytes())?;
    mac.update(s.to_string().as_bytes());
    let signature_bytes = mac.finalize().into_bytes();
    Ok(hex::encode(signature_bytes))
}

pub fn payload_to_json_str(payload: Vec<(String, String)>) -> Result<String, serde_json::Error> {
    if payload.is_empty() {
        Ok(String::new())  // -> ""
    } else {
        let mut map = Map::new();
        for (k, v) in payload {
            map.insert(k, Value::String(v));
        }

        let obj = Value::Object(map);
        serde_json::to_string(&obj)  // e.g. -> "{\"currency_pair\":\"GT_USDT\",…}"
    }
}
