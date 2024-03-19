extern crate log;
use zenoh::config::{Config};
use rhai::{Map};
use std::io::{self, Write};
use crate::cmd;
use crate::zbus_lib;

fn run_in_stdout() {
    loop {
        if ! zbus_lib::bus::channel::pipe_is_empty_raw("in".to_string()) {
            log::debug!("Processing data in IN channel");
            while ! zbus_lib::bus::channel::pipe_is_empty_raw("in".to_string()) {
                match zbus_lib::bus::channel::pipe_pull_raw("in".to_string()) {
                    Ok(res) => {
                        match serde_json::from_str::<Map>(&res) {
                            Ok(m) => {
                                let out = format!("{:?}", &m);
                                let mut stdout = io::stdout().lock();
                                let _ = stdout.write_all(out.as_bytes());
                                let _ = stdout.write_all("\n".as_bytes());
                                let _ = stdout.flush();
                                drop(out);
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

pub fn run(c: &cmd::Cli, _pipeline: &cmd::Pipeline, cmdvec: &cmd::PipelineVector, zc: Config)  {
    log::trace!("zbus_pipeline_vector::run() reached");

    cmd::zbus_pipeline_lib::pipeline_bus_channel("in".to_string(), cmdvec.pipeline_in.clone(), c.clone(), zc);
    zbus_lib::bus::channel::pipes_init();

    if cmdvec.group.stdout {
        run_in_stdout();
    } else {
        run_in_stdout();
    }

    log::debug!("Script is finished, now wait for flushing the IN channel");
    while ! zbus_lib::bus::channel::pipe_is_empty_raw("in".to_string()) {
        log::debug!("OUT channel is not flushed to ZBUS. Waiting...");
        zbus_lib::system::system_module::sleep(5);
    }
    log::debug!("All channels are flushed. Exiting...");

}
