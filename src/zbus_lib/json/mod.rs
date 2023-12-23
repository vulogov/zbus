extern crate log;
use rhai::{Engine, Dynamic, Map, Array};
use rhai::plugin::*;
use rhai::serde::{to_dynamic};
use serde_json::{to_string, from_str};


#[export_module]
pub mod json_module {
    pub fn dynamic(d: String) -> Dynamic {
        match to_dynamic(d) {
            Ok(res) => res,
            Err(err) => {
                log::error!("Error converting from JSON: {}", err);
                return Dynamic::default();
            }
        }
    }
    pub fn to_map(d: String) -> Map {
        match from_str(&d) {
            Ok(res) => res,
            Err(err) => {
                log::error!("Error converting from JSON: {}", err);
                return Map::new();
            }
        }
    }
    pub fn to_list(d: String) -> Array {
        match from_str(&d) {
            Ok(res) => res,
            Err(err) => {
                log::error!("Error converting from JSON: {}", err);
                return Array::new();
            }
        }
    }
    pub fn string(d: Dynamic) -> String {
        match to_string(&d) {
            Ok(res) => res,
            Err(err) => {
                log::error!("Error converting to JSON: {}", err);
                return "".to_string();
            }
        }
    }
}

pub fn init(engine: &mut Engine) {
    log::trace!("Running STDLIB::JSON init");
    let module = exported_module!(json_module);
    engine.register_static_module("json", module.into());
}
