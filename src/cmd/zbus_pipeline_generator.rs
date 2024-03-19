extern crate log;
use zenoh::config::{Config};
use rhai::{Dynamic};
use crate::cmd;
use crate::stdlib::getfile;
use crate::zbus_lib;


pub fn run(c: &cmd::Cli, pipeline: &cmd::Pipeline, generator: &cmd::PipelineGenerator, zc: Config)  {
    log::trace!("zbus_pipeline_generator::run() reached");

    let mut argv: Vec<Dynamic> = Vec::new();
    for v in generator.args.clone() {
        argv.push(Dynamic::from(v));
    }

    cmd::zbus_pipeline_lib::pipeline_channel_bus("out".to_string(), generator.pipeline.clone(), c.clone(), zc);

    zbus_lib::bus::channel::pipes_init();

    if pipeline.group.stdin {
        cmd::zbus_pipeline::run_zbus_script_for_pipeline(getfile::get_file_from_stdin(), "GENERATOR".to_string(), c, argv)
    } else {
        match &pipeline.group.file {
            Some(script_name) => {
                match getfile::get_file_from_file(script_name.trim().to_string()) {
                    Some(script) => cmd::zbus_pipeline::run_zbus_script_for_pipeline(script, "GENERATOR".to_string(), c, argv),
                    None => log::error!("Script is empty"),
                }
            }
            None => {
                match &pipeline.group.url {
                    Some(script_name) => {
                        match getfile::get_file_from_uri(script_name.trim().to_string()) {
                            Some(script) => cmd::zbus_pipeline::run_zbus_script_for_pipeline(script, "GENERATOR".to_string(), c, argv),
                            None => log::error!("Script is empty"),
                        }
                    }
                    None => {
                        cmd::zbus_pipeline::run_zbus_script_for_pipeline(getfile::get_file_from_stdin(), "GENERATOR".to_string(), c, argv);
                    }
                }
            }
        }
    }
    log::debug!("Script is finished, now wait for flushing the OUT channel");
    while ! zbus_lib::bus::channel::pipe_is_empty_raw("out".to_string()) {
        log::debug!("OUT channel is not flushed to ZBUS. Waiting...");
        zbus_lib::system::system_module::sleep(5);
    }
    log::debug!("All channels are flushed. Exiting...");

}
