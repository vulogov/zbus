extern crate log;

use rhai::{Engine, Scope, Dynamic};
use rhai::packages::Package;
use rhai_sci::SciPackage;
use rhai_rand::RandomPackage;
use rhai_fs::FilesystemPackage;
use rhai_url::UrlPackage;
use rhai_ml::MLPackage;

use zenoh::config::{Config};

use crate::zbus_lib;
use crate::cmd;
use crate::cmd::{zbus_pipeline_feeder, zbus_pipeline_generator, zbus_pipeline_processor, zbus_pipeline_sink, zbus_pipeline_aggregator, zbus_pipeline_fan};

pub fn run_zbus_script_for_pipeline(script: String, tool: String, c: &cmd::Cli, argv: Vec<Dynamic>) {
    log::trace!("Execiting ZBUS scriptfor pipeline len()={}", &script.len());
    let mut engine = Engine::new();
    engine.register_global_module(SciPackage::new().as_shared_module());
    engine.register_global_module(RandomPackage::new().as_shared_module());
    engine.register_global_module(FilesystemPackage::new().as_shared_module());
    engine.register_global_module(UrlPackage::new().as_shared_module());
    engine.register_global_module(MLPackage::new().as_shared_module());
    let mut scope = Scope::new();

    scope.push("ARGV", Dynamic::from(argv))
         .push("ZBUS_PROTOCOL_VERSION", Dynamic::from(c.protocol_version.clone()))
         .push("PLATFORM_NAME", Dynamic::from(c.platform_name.clone()))
         .push("ZBUS_ADDRESS", Dynamic::from(c.bus.clone()))
         .push("API_ENDPOINT", Dynamic::from("".to_string()))
         .push("PIPELINE_TOOL", Dynamic::from(tool.clone()));
    zbus_lib::initscope(&mut scope);
    zbus_lib::initlib(&mut engine, c);
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
    log::debug!("ZB-script engine serving pipeline is no more");
}

pub fn run(c: &cmd::Cli, pipeline: &cmd::Pipeline, zc: Config)  {
    log::trace!("zbus_pipeline::run() reached");
    match &pipeline.command {
        cmd::PipelineCommands::Feeder(feeder) => {
            zbus_pipeline_feeder::run(c, pipeline, feeder, zc)
        }
        cmd::PipelineCommands::Generator(generator) => {
            zbus_pipeline_generator::run(c, pipeline, generator, zc)
        }
        cmd::PipelineCommands::Processor(processor) => {
            zbus_pipeline_processor::run(c, pipeline, processor, zc)
        }
        cmd::PipelineCommands::Sink(sink) => {
            zbus_pipeline_sink::run(c, pipeline, sink, zc)
        }
        cmd::PipelineCommands::Aggregator(aggregator) => {
            zbus_pipeline_aggregator::run(c, pipeline, aggregator, zc)
        }
        cmd::PipelineCommands::Fan(fan) => {
            zbus_pipeline_fan::run(c, pipeline, fan, zc)
        }
    }
}
