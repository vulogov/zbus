extern crate log;
use rhai::{Engine, Dynamic};
use rhai::plugin::*;

pub mod zabbix_get;

#[export_module]
pub mod zabbix_module {

}

pub fn zabbix_get_function(_context: NativeCallContext, addr: String, key: String) -> Result<Dynamic, Box<EvalAltResult>>{
    zabbix_get::zabbix_get(addr, key)
}

pub fn init(engine: &mut Engine) {
    log::trace!("Running STDLIB::zabbix init");

    engine.register_type::<zabbix_get::ZabbixAgent>()
          .register_fn("ZabbixAgent", zabbix_get::ZabbixAgent::new)
          .register_fn("get", zabbix_get::ZabbixAgent::get_sampler)
          .register_fn("get", zabbix_get::ZabbixAgent::get_to_sampler)
          .register_fn("to_string", |x: &mut zabbix_get::ZabbixAgent| format!("ZabbixAgent({:?})", x.addr) );

    let mut module = exported_module!(zabbix_module);
    module.set_native_fn("zabbix_get", zabbix_get_function);
    engine.register_static_module("zabbix", module.into());
}
