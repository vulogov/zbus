extern crate log;
use rhai::{Engine};
use rhai::plugin::*;

#[export_module]
pub mod string_module {

}

pub fn init(engine: &mut Engine) {
    log::trace!("Running STDLIB::string init");
    let module = exported_module!(string_module);

    engine.register_static_module("string", module.into());
}
