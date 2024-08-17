use hex;
use serde_json::Value;
use sha2::{Sha256, Digest};

mod models;
pub mod handlers;

pub fn detect_event_schema(event_data: &Value) -> String {
    let event_json = serde_json::to_string(event_data).unwrap();
    let mut hasher = Sha256::new();
    hasher.update(event_json);
    hex::encode(hasher.finalize())
}
