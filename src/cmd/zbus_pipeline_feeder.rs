extern crate log;
use zenoh::config::{Config};
use zenoh::prelude::sync::*;
use rhai::{Dynamic, Map};
use crate::cmd;
use crate::stdlib::getfile;
use crate::zbus_lib;

pub fn run(c: &cmd::Cli, pipeline: &cmd::Pipeline, feeder: &cmd::PipelineFeeder, zc: Config)  {
    log::trace!("zbus_pipeline_feeder::run() reached");

    let mut argv: Vec<Dynamic> = Vec::new();
    for v in feeder.args.clone() {
        argv.push(Dynamic::from(v));
    }

    zbus_lib::bus::channel::pipes_init();

    match zbus_lib::threads::THREADS.lock() {
        Ok(t) => {
            let c = c.clone();
            t.execute(move ||
            {
                log::debug!("Feeder thread has been started");
                match zenoh::open(zc.clone()).res() {
                    Ok(session) => {
                        log::debug!("Bus({}) session established", &c.bus);
                        loop {
                            if ! zbus_lib::bus::channel::pipe_is_empty_raw("out".to_string()) {
                                log::debug!("Processing data in OUT channel");
                                while ! zbus_lib::bus::channel::pipe_is_empty_raw("out".to_string()) {
                                    match zbus_lib::bus::channel::pipe_pull_raw("out".to_string()) {
                                        Ok(res) => {
                                            match serde_json::from_str::<Map>(&res) {
                                                Ok(m) => {
                                                    match m.get("key") {
                                                        Some(key) => {
                                                            let _ = session.put(key.to_string(), res.clone()).encoding(KnownEncoding::AppJson).res();
                                                        }
                                                        None => {}
                                                    }
                                                }
                                                Err(_) => {}
                                            }

                                        }
                                        Err(err) => log::error!("Error getting data from ZBUS: {:?}", err),
                                    }
                                }
                            }
                            zbus_lib::system::system_module::sleep(1);
                        }
                    }
                    Err(err) => {
                        log::error!("Error opening Bus() session: {:?}", err);
                    }
                }
            });
            drop(t);
        }
        Err(err) => {
            log::error!("Error accessing Thread Manager: {:?}", err);
            return;
        }
    }

    if pipeline.group.stdin {
        cmd::zbus_pipeline::run_zbus_script_for_pipeline(getfile::get_file_from_stdin(), "FEEDER".to_string(), c, argv)
    } else {
        match &pipeline.group.file {
            Some(script_name) => {
                match getfile::get_file_from_file(script_name.trim().to_string()) {
                    Some(script) => cmd::zbus_pipeline::run_zbus_script_for_pipeline(script, "FEEDER".to_string(), c, argv),
                    None => log::error!("Script is empty"),
                }
            }
            None => {
                match &pipeline.group.url {
                    Some(script_name) => {
                        match getfile::get_file_from_uri(script_name.trim().to_string()) {
                            Some(script) => cmd::zbus_pipeline::run_zbus_script_for_pipeline(script, "FEEDER".to_string(), c, argv),
                            None => log::error!("Script is empty"),
                        }
                    }
                    None => {
                        cmd::zbus_pipeline::run_zbus_script_for_pipeline(getfile::get_file_from_stdin(), "FEEDER".to_string(), c, argv);
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
