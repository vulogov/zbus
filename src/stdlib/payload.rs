extern crate log;
use rhai::{Engine, Dynamic};
use rhai::packages::Package;
use rhai_sci::SciPackage;
use rhai_rand::RandomPackage;
use serde::{Serialize};
use serde_json;

#[derive(Serialize, Debug)]
struct TelemetryPayloadString {
    ts: i64,
    platform: String,
    src: String,
    key: String,
    skey: String,
    value: String
}

#[derive(Serialize, Debug)]
struct TelemetryPayloadInt {
    ts: i64,
    platform: String,
    src: String,
    key: String,
    skey: String,
    value: i64
}

#[derive(Serialize, Debug)]
struct TelemetryPayloadFloat {
    ts: i64,
    platform: String,
    src: String,
    key: String,
    skey: String,
    value: f64
}

pub fn generate_payload(timestamp: i64, platform: String, src: String, key: String, skey: String, v: &String) -> Option<String> {

    let mut engine = Engine::new();
    engine.register_global_module(SciPackage::new().as_shared_module());
    engine.register_global_module(RandomPackage::new().as_shared_module());
    match engine.eval_expression::<Dynamic>(&v) {
        Ok(val) => {
            match val.type_name() {
                "string" => {
                    return Some(serde_json::to_string(&TelemetryPayloadString {ts: timestamp, platform: platform, src: src, key: key, skey: skey, value: val.to_string()}).unwrap())
                }
                "i64" => {
                    return Some(serde_json::to_string(&TelemetryPayloadInt {ts: timestamp, platform: platform, src: src, key: key, skey: skey, value: val.as_int().unwrap()}).unwrap())
                }
                "f64" => {
                    return Some(serde_json::to_string(&TelemetryPayloadFloat {ts: timestamp, platform: platform, src: src, key: key, skey: skey, value: val.as_float().unwrap()}).unwrap())
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

pub fn generate_raw_payload(timestamp: i64, platform: String, src: String, key: String, skey: String, v: &String) -> Option<String> {
    return Some(serde_json::to_string(&TelemetryPayloadString {ts: timestamp, platform: platform, src: src, key: key, skey: skey, value: v.clone()}).unwrap())
}
