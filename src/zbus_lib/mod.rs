extern crate log;
use crate::cmd;
use rhai::{Engine, Scope, Dynamic};
use rhai::packages::Package;
use rhai_sci::SciPackage;
use rhai_rand::RandomPackage;
use rhai_fs::FilesystemPackage;
use rhai_url::UrlPackage;
use rhai_ml::MLPackage;

pub mod bus;
pub mod conversions;
pub mod json;
pub mod zbus_log;
pub mod timestamp;
pub mod threads;
pub mod neuralnet;
pub mod interval;
pub mod grok;
pub mod sampler;
pub mod dynamicsampler;
pub mod metric;
pub mod input;
pub mod filters;
pub mod system;
pub mod string;
pub mod zabbix;

pub fn run_zbus_script(script: String, c: &cmd::Cli, s: &cmd::Script) {
    log::trace!("Execiting ZBUS script len()={}", &script.len());
    let mut engine = Engine::new();
    engine.register_global_module(SciPackage::new().as_shared_module());
    engine.register_global_module(RandomPackage::new().as_shared_module());
    engine.register_global_module(FilesystemPackage::new().as_shared_module());
    engine.register_global_module(UrlPackage::new().as_shared_module());
    engine.register_global_module(MLPackage::new().as_shared_module());
    let mut scope = Scope::new();
    let mut argv: Vec<Dynamic> = Vec::new();
    for v in s.args.clone() {
        argv.push(Dynamic::from(v));
    }
    scope.push("ARGV", Dynamic::from(argv))
         .push("ZBUS_PROTOCOL_VERSION", Dynamic::from(c.protocol_version.clone()))
         .push("PLATFORM_NAME", Dynamic::from(c.platform_name.clone()))
         .push("ZBUS_ADDRESS", Dynamic::from(c.bus.clone()))
         .push("API_ENDPOINT", Dynamic::from(s.endpoint.clone()))
         .push("PIPELINE_TOOL", Dynamic::from("NONE".to_string()));
    initscope(&mut scope);
    initlib(&mut engine, c);
    match engine.run_with_scope(&mut scope, script.as_str()) {
        Ok(res) => {
            log::debug!("Script returned: {:?}", res);
        }
        Err(err) => {
            log::error!("Script returned error: {:?}", err);
        }
    }
    //threads::terminale_all();
    //log::debug!("Threads has been terminated, killing ZB-script engine");
    drop(scope);
    drop(engine);
    log::debug!("ZB-script engine is no more");
}

pub fn run_zbus_script_no_cli(script: String, label: String) {
    log::trace!("Execiting ZBUS script len()={}, label={}", &script.len(), &label);
    let mut engine = Engine::new();
    engine.register_global_module(SciPackage::new().as_shared_module());
    engine.register_global_module(RandomPackage::new().as_shared_module());
    engine.register_global_module(FilesystemPackage::new().as_shared_module());
    engine.register_global_module(UrlPackage::new().as_shared_module());
    engine.register_global_module(MLPackage::new().as_shared_module());
    let mut scope = Scope::new();
    let argv: Vec<Dynamic> = Vec::new();
    scope.push("ARGV", Dynamic::from(argv))
         .push("ZBUS_PROTOCOL_VERSION", Dynamic::from("v1"))
         .push("PLATFORM_NAME", Dynamic::from(""))
         .push("ZBUS_ADDRESS", Dynamic::from(""))
         .push("API_ENDPOINT", Dynamic::from(""))
         .push("PIPELINE_TOOL", Dynamic::from("NONE".to_string()));
    initscope(&mut scope);
    initlib_no_cli(&mut engine);
    match engine.run_with_scope(&mut scope, script.as_str()) {
        Ok(res) => {
            log::debug!("Script returned: {:?}", res);
        }
        Err(err) => {
            log::error!("Script returned error: {:?}", err);
        }
    }
    drop(scope);
    drop(engine);
    log::debug!("ZB-script engine is no more, label={}", &label);
}

pub fn initscope(scope: &mut Scope) {
    log::debug!("Initializing ZBUS RHAI scope");
    scope.push("ANSWER", 42_i64);
}

pub fn initlib(engine: &mut Engine, c: &cmd::Cli)  {
    log::debug!("Initializing ZBUS RHAI library");
    initlib_no_cli_required(engine);
    threads::init(engine, c);
}

pub fn initlib_no_cli(engine: &mut Engine)  {
    log::debug!("Initializing ZBUS RHAI library for thread management");
    initlib_no_cli_required(engine);
    threads::init_no_cli(engine);
}

fn initlib_no_cli_required(engine: &mut Engine)  {
    log::debug!("Initializing ZBUS RHAI library generic modules");
    bus::init(engine);
    conversions::init(engine);
    json::init(engine);
    grok::init(engine);
    zbus_log::init(engine);
    timestamp::init(engine);
    interval::init(engine);
    sampler::init(engine);
    dynamicsampler::init(engine);
    metric::init(engine);
    system::init(engine);
    input::init(engine);
    neuralnet::init(engine);
    filters::init(engine);
    string::init(engine);
    zabbix::init(engine);
}
