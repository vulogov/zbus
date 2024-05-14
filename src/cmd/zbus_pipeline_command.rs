extern crate log;
use zenoh::config::{Config};
use crate::cmd;
use crate::zbus_lib;
use crate::zbus_lib::timestamp::timestamp_module::{timestamp_ns};

pub fn run(c: &cmd::Cli, _pipeline: &cmd::Pipeline, command: &cmd::PipelineCommand, zc: Config)  {
    log::trace!("zbus_pipeline_generator::run() reached");

    zbus_lib::bus::channel::pipes_init();

    cmd::zbus_pipeline_lib::pipeline_channel_bus("out".to_string(), command.pipeline.clone(), c.clone(), zc);

    zbus_lib::system::system_module::sleep(1);

    for v in command.args.clone() {
        log::debug!("Sending command: {}", &v);
        let data = serde_json::json!({
            "timestamp":    timestamp_ns(),
            "platform":     c.platform_name.clone(),
            "pipeline":     command.pipeline.clone(),
            "command":      &v,
        });
        zbus_lib::bus::channel::pipe_push_raw("out".to_string(), data.to_string());
    }

    log::debug!("Script is finished, now wait for flushing the OUT channel");
    while ! zbus_lib::bus::channel::pipe_is_empty_raw("out".to_string()) {
        log::debug!("OUT channel is not flushed to ZBUS. Waiting...");
        zbus_lib::system::system_module::sleep(10);
    }
    log::debug!("All channels are flushed. Exiting...");

}
