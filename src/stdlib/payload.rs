extern crate log;
use rhai::{Engine, Dynamic};
use serde::{Serialize};
use serde_json;

#[derive(Serialize, Debug)]
struct TelemetryPayloadString {
    ts: i64,
    key: String,
    value: String
}

#[derive(Serialize, Debug)]
struct TelemetryPayloadInt {
    ts: i64,
    key: String,
    value: i64
}

#[derive(Serialize, Debug)]
struct TelemetryPayloadFloat {
    ts: i64,
    key: String,
    value: f64
}

pub fn generate_payload(timestamp: i64, key: String, v: &String) -> Option<String> {

    let engine = Engine::new();
    match engine.eval_expression::<Dynamic>(&v) {
        Ok(val) => {
            match val.type_name() {
                "string" => {
                    return Some(serde_json::to_string(&TelemetryPayloadString {ts: timestamp, key: key, value: val.to_string()}).unwrap())
                }
                "i64" => {
                    return Some(serde_json::to_string(&TelemetryPayloadInt {ts: timestamp, key: key, value: val.as_int().unwrap()}).unwrap())
                }
                "f64" => {
                    return Some(serde_json::to_string(&TelemetryPayloadFloat {ts: timestamp, key: key, value: val.as_float().unwrap()}).unwrap())
                }
                _ => log::error!("Scripting return unrecognizeable data type: {}", &val.type_name())
            }
        }
        Err(err) => {
            log::error!("Expression evaluation error: {:?}", err);
            return None;
        }
    }
    None
}
