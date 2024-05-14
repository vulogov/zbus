extern crate log;
use zenoh::config::{Config};
use serde_json::{Map, Value};
use crate::cmd;
use crate::zbus_lib;
use crate::zbus_lib::timestamp::timestamp_module::{timestamp_ns};

pub fn run(c: &cmd::Cli, _pipeline: &cmd::Pipeline, shell: &cmd::PipelineShell, zc: Config)  {
    log::trace!("zbus_pipeline_shell::run() reached");

    zbus_lib::bus::channel::pipes_init();

    cmd::zbus_pipeline_lib::pipeline_bus_channel("in".to_string(), shell.pipeline_in.clone(), c.clone(), zc.clone());
    cmd::zbus_pipeline_lib::pipeline_channel_bus("out".to_string(), shell.pipeline.clone(), c.clone(), zc.clone());

    log::debug!("Wait for 5 seconds for ZBUS");
    zbus_lib::system::system_module::sleep(5);
    log::debug!("Hope ZBUS is ready");

    'outer: loop {
        if ! zbus_lib::bus::channel::pipe_is_empty_raw("in".to_string()) {
            log::debug!("Processing data in IN channel");
            while ! zbus_lib::bus::channel::pipe_is_empty_raw("in".to_string()) {
                match zbus_lib::bus::channel::pipe_pull_raw("in".to_string()) {
                    Ok(res) => {
                        match serde_json::from_str::<Map<String, Value>>(&res) {
                            Ok(jres) => {
                                if jres.contains_key("command") {
                                    let jcmd = jres.get("command").unwrap().to_string();
                                    log::debug!("Requested command: {}", &jcmd);
                                    let (code, out, err) = shells::sh!("bash -c {}", jcmd);
                                    let data = serde_json::json!({
                                        "timestamp":    timestamp_ns(),
                                        "command":      &jcmd,
                                        "out":          &out,
                                        "err":          &err,
                                        "retcode":      code,
                                    });
                                    zbus_lib::bus::channel::pipe_push_raw("out".to_string(), data.to_string());
                                } else {
                                    log::error!("You do not requested any command");
                                }
                            }
                            Err(err) => {
                                log::error!("Error parsing JSON: {:?}", err);
                            }
                        }
                    }
                    Err(err) => {
                        log::error!("Error getting data from ZBUS: {:?}", err);
                        break 'outer;
                    }
                }
            }
        }
        zbus_lib::system::system_module::sleep(1);
    }

    log::debug!("Processing is finished, now wait for flushing the IN channel");
    while ! zbus_lib::bus::channel::pipe_is_empty_raw("in".to_string()) {
        log::debug!("IN channel is not flushed to ZBUS. Waiting...");
        zbus_lib::system::system_module::sleep(5);
    }
    while ! zbus_lib::bus::channel::pipe_is_empty_raw("out".to_string()) {
        log::debug!("OUT channel is not flushed to ZBUS. Waiting...");
        zbus_lib::system::system_module::sleep(5);
    }
    log::debug!("All channels are flushed. Exiting...");

}
