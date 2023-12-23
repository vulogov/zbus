extern crate log;
use crate::cmd;
use crate::stdlib::getfile;
use zenoh::config::{Config};
use rhai::{Engine, Scope, Dynamic};
use rhai::packages::Package;
use rhai_sci::SciPackage;
use rhai_rand::RandomPackage;
use rhai_fs::FilesystemPackage;
use rhai_url::UrlPackage;
use rhai_ml::MLPackage;
use crate::zbus_lib;


fn run_zbus_script(script: String, s: &cmd::Script) {
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
    scope.push("ARGV", Dynamic::from(argv));
    zbus_lib::initscope(&mut scope);
    zbus_lib::initlib(&mut engine);
    let _ = engine.run_with_scope(&mut scope, script.as_str());
    drop(scope);
    drop(engine);
}

pub fn run(_c: &cmd::Cli, s: &cmd::Script, _zc: Config)  {
    log::trace!("zbus_script::run() reached");
    if s.stdin {
        run_zbus_script(getfile::get_file_from_stdin(), s)
    } else if ! s.file.trim().is_empty() {
        run_zbus_script(getfile::get_file_from_file(s.file.trim().to_string()), s)
    } else if ! s.uri.trim().is_empty() {
        run_zbus_script(getfile::get_file_from_uri(s.uri.trim().to_string()), s)
    } else if ! s.eval.trim().is_empty() {
        run_zbus_script(s.eval.trim().to_string(), s)
    } else {
        run_zbus_script(getfile::get_file_from_stdin(), s)
    }
}
