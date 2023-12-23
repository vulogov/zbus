extern crate log;
use crate::cmd;
use rhai::{Engine, Scope, Dynamic};
use rhai::packages::Package;
use rhai_sci::SciPackage;
use rhai_rand::RandomPackage;
use rhai_fs::FilesystemPackage;
use rhai_url::UrlPackage;
use rhai_ml::MLPackage;

pub mod json;
pub mod zbus_log;
pub mod timestamp;
pub mod sampler;

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
         .push("API_ENDPOINT", Dynamic::from(s.endpoint.clone()));
    initscope(&mut scope);
    initlib(&mut engine);
    let _ = engine.run_with_scope(&mut scope, script.as_str());
    drop(scope);
    drop(engine);
}

pub fn initscope(scope: &mut Scope) {
    log::debug!("Initializing ZBUS RHAI scope");
    scope.push("ANSWER", 42_i64);
}

pub fn initlib(engine: &mut Engine)  {
    log::debug!("Initializing ZBUS RHAI library");
    json::init(engine);
    zbus_log::init(engine);
    timestamp::init(engine);
    sampler::init(engine);
}
