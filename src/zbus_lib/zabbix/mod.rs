extern crate log;
use rhai::{Engine, Dynamic, Map, EvalAltResult};
use rhai::plugin::*;

pub mod zabbix_get;
pub mod zabbix_sender;

#[derive(Debug)]
pub enum ZabbixError {
    NotSupported(String),
    SenderError(String),
}

impl std::fmt::Display for ZabbixError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ZabbixError::NotSupported(e) => f.write_fmt(format_args!("ZabbixNotSupported ({})", e)),
            ZabbixError::SenderError(e) => f.write_fmt(format_args!("ZabbixSenderError ({})", e)),
        }
    }
}
impl std::error::Error for ZabbixError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

#[export_module]
pub mod zabbix_module {

}

pub fn zabbix_get_function(_context: NativeCallContext, addr: String, key: String) -> Result<Dynamic, Box<EvalAltResult>>{
    zabbix_get::zabbix_get(addr, key)
}

pub fn zabbix_sender_function(addr: String, data: Map) -> Result<Dynamic, Box<EvalAltResult>> {
    zabbix_sender::zabbix_sender(addr, data)
}

pub fn init(engine: &mut Engine) {
    log::trace!("Running STDLIB::zabbix init");

    engine.register_type::<zabbix_get::ZabbixAgent>()
        .register_fn("ZabbixAgent", zabbix_get::ZabbixAgent::new)
        .register_fn("get", zabbix_get::ZabbixAgent::get_sampler)
        .register_fn("get", zabbix_get::ZabbixAgent::get_to_sampler)
        .register_fn("to_string", |x: &mut zabbix_get::ZabbixAgent| format!("ZabbixAgent({:?})", x.addr) );

    engine.register_type::<zabbix_sender::ZabbixSender>()
        .register_fn("ZabbixSender", zabbix_sender::ZabbixSender::new)
        .register_fn("send", zabbix_sender::ZabbixSender::send)
        .register_fn("send", zabbix_sender::ZabbixSender::send_with_current_timestamp)
        .register_fn("send", zabbix_sender::ZabbixSender::send_sampler)
        .register_fn("to_string", |x: &mut zabbix_sender::ZabbixSender| format!("ZabbixSender({:?})", x.addr) );

    let mut module = exported_module!(zabbix_module);
    module.set_native_fn("zabbix_get", zabbix_get_function);
    module.set_native_fn("zabbix_sender", zabbix_sender_function);
    engine.register_static_module("zabbix", module.into());
}
