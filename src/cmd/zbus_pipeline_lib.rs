extern crate log;
use zenoh::config::{Config};
use zenoh::prelude::sync::*;
use crate::zbus_lib;
use crate::cmd;

pub fn pipeline_channel_bus(c_name: String, bus_key: String, c: cmd::Cli, zc: Config)  {

    match zbus_lib::threads::THREADS.lock() {
        Ok(t) => {
            t.execute(move ||
            {
                log::debug!("Channel->Bus thread has been started for {}", &bus_key);
                match zenoh::open(zc.clone()).res() {
                    Ok(session) => {
                        let pipeline_name = format!("zbus/pipeline/{}/{}", &c.protocol_version, &bus_key);
                        loop {
                            if ! zbus_lib::bus::channel::pipe_is_empty_raw(c_name.clone()) {
                                log::debug!("Processing data in {} channel to {}", &c_name, &pipeline_name);
                                while ! zbus_lib::bus::channel::pipe_is_empty_raw(c_name.clone()) {
                                    match zbus_lib::bus::channel::pipe_pull_raw(c_name.clone()) {
                                        Ok(res) => {
                                            let _ = session.put(pipeline_name.clone(), res.clone()).encoding(KnownEncoding::AppJson).res();
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

}

pub fn pipeline_bus_channel(c_name: String, bus_key: String, c: cmd::Cli, zc: Config)  {

    match zbus_lib::threads::THREADS.lock() {
        Ok(t) => {
            t.execute(move ||
            {
                log::debug!("Bus->Channel thread has been started for {}", &bus_key);
                let ch_name = c_name.clone();
                match zenoh::open(zc.clone()).res() {
                    Ok(session) => {
                        let pipeline_name = format!("zbus/pipeline/{}/{}", &c.protocol_version, &bus_key);
                        match session.declare_subscriber(&pipeline_name)
                                .callback_mut(move |sample| {
                                    let slices = &sample.value.payload.contiguous();
                                    match std::str::from_utf8(slices) {
                                        Ok(data) => {
                                            zbus_lib::bus::channel::pipe_push_raw(ch_name.clone(), data.to_string());
                                        }
                                        Err(err) => {
                                            log::error!("Error while extracting data from ZENOH bus: {:?}", err);
                                        }
                                    }
                                })
                                .res() {
                            Ok(_) => {
                                loop {
                                    zbus_lib::system::system_module::sleep(5);
                                    std::thread::yield_now();
                                }
                            }
                            Err(err) => {
                                log::error!("Telemetry subscribe for key {} failed: {:?}", &pipeline_name, err);
                                return;
                            }
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

}
