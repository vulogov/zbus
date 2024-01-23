extern crate log;
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use rhai::{Engine};
use rhai::plugin::*;

#[export_module]
pub mod timestamp_module {
    pub fn timestamp_ms() -> f64 {
    	SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as f64
    }
    pub fn timestamp_ns() -> f64 {
    	SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() as f64
    }
    pub fn whole_seconds(t: f64) -> f64 {
        Duration::from_nanos(t as u64).as_secs_f64()
    }
    pub fn nanoseconds(t: f64) -> f64 {
        Duration::from_nanos(t as u64).subsec_nanos() as f64
    }
}

pub fn init(engine: &mut Engine) {
    log::trace!("Running STDLIB::timestamp init");
    let module = exported_module!(timestamp_module);

    engine.register_static_module("timestamp", module.into());
}
