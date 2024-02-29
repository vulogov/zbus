extern crate log;
use rhai::{Engine, Dynamic, Map, Array, NativeCallContext, EvalAltResult};
use rhai::plugin::*;
use rhai::serde::{to_dynamic};
use serde_json::{to_string, from_str};


#[export_module]
pub mod json_module {

}

pub fn json_to_string_fun(_context: NativeCallContext, d: Dynamic) -> Result<String, Box<EvalAltResult>> {
    match to_string(&d) {
        Ok(res) => Ok(res),
        Err(err) => {
            log::error!("Error converting JSON to string: {}", err);
            return Err(format!("Error converting JSON to string: {}", err).into());
        }
    }
}

pub fn string_to_array_fun(_context: NativeCallContext, d: String) -> Result<Array, Box<EvalAltResult>> {
    match from_str::<Dynamic>(&d) {
        Ok(res) => {
            if res.is_array() {
                return Ok(res.clone_cast::<Array>());
            } else {
                log::error!("Value is not of Array type");
                return Err("Value is not of Array type".into());
            }
        }
        Err(err) => {
            log::error!("Error converting from JSON: {}", err);
            return Err(format!("Error converting string to array: {}", err).into());
        }
    }
}

pub fn string_to_map_fun(_context: NativeCallContext, d: String) -> Result<Map, Box<EvalAltResult>> {
    match from_str::<Dynamic>(&d) {
        Ok(res) => {
            if res.is_map() {
                return Ok(res.clone_cast::<Map>());
            } else {
                log::error!("Value is not of Map type");
                return Err("Value is not of Map type".into());
            }
        }
        Err(err) => {
            log::error!("Error converting from JSON: {}", err);
            return Err(format!("Error converting string to map: {}", err).into());
        }
    }
}

pub fn string_to_dynamic_fun(_context: NativeCallContext, d: String) -> Result<Dynamic, Box<EvalAltResult>> {
    match to_dynamic(d) {
        Ok(res) => Ok(res),
        Err(err) => {
            log::error!("Error converting string to dynamic: {}", err);
            return Err(format!("Error converting string to dynamic: {}", err).into());
        }
    }
}

pub fn init(engine: &mut Engine) {
    log::trace!("Running STDLIB::JSON init");
    let mut module = exported_module!(json_module);
    module.set_native_fn("string", json_to_string_fun);
    module.set_native_fn("to_list", string_to_array_fun);
    module.set_native_fn("to_map", string_to_map_fun);
    module.set_native_fn("dynamic", string_to_dynamic_fun);
    engine.register_static_module("json", module.into());
}
