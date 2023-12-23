extern crate log;
use std::{env};
use std::{thread, time};
use rhai::{Engine, Dynamic, Module};
use rhai::plugin::*;
use proctitle::set_title;

#[export_module]
pub mod system_module {
    pub fn sleep(s: i64) {
        thread::sleep(time::Duration::from_secs(s as u64));
    }
    pub fn sleep_millisecond(s: i64) {
        thread::sleep(time::Duration::from_millis(s as u64));
    }
    pub fn env(n: String) -> String {
        match env::var(&n) {
            Ok(val) => return val,
            Err(e) => {
                log::error!("Error fetching environment variable {}: {:?}", &n, e);
            }
        }
        return "".to_string();
    }
    pub fn running_as() -> String {
        match sudo::check() {
            sudo::RunningAs::Root => "root".to_string(),
            sudo::RunningAs::User => "user".to_string(),
            sudo::RunningAs::Suid => "suid".to_string(),
        }
    }
    pub fn title(t: String) {
        set_title(t);
    }
}

pub fn init(engine: &mut Engine) {
    log::trace!("Running STDLIB::system init");

    let module = exported_module!(system_module);
    engine.register_static_module("system", module.into());
}
