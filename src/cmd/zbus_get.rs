extern crate log;
use zenoh::config::{Config};
use zenoh::prelude::sync::*;
use serde_json;

use crate::cmd;
use crate::stdlib::telemetry_key;

pub fn run(c: &cmd::Cli, p: &cmd::Get, zc: Config)  {
    log::trace!("zbus_get::run() reached");
    log::debug!("ZENOH bus address: {}", &c.bus);

    if ! telemetry_key::telemetry_key_validate(p.key.clone()) {
        log::error!("Telemetry key is invalid");
        return;
    }

    match zenoh::open(zc).res() {
        Ok(session) => {
            log::debug!("Connection to ZENOH bus succesful");
            let key = match p.telemetry_type {
                cmd::TelemetryType::Metric => format!("zbus/metric/{}/{}/{}", &c.protocol_version, &p.source, &p.key),
                cmd::TelemetryType::Event => format!("zbus/event/{}/{}/{}", &c.protocol_version, &p.source, &p.key),
                cmd::TelemetryType::Trace => format!("zbus/trace/{}/{}/{}", &c.protocol_version, &p.source, &p.key),
                cmd::TelemetryType::Log => format!("zbus/log/{}/{}/{}", &c.protocol_version, &p.source, &p.key)
            };
            log::debug!("Telemetry key is: {}", &key);
            match session.get(&key).res() {
                Ok(replies) => {
                    while let Ok(reply) = replies.recv() {
                        match reply.sample {
                            Ok(sample) => {
                                let slices = &sample.value.payload.contiguous();
                                match std::str::from_utf8(slices) {
                                    Ok(data) => {
                                        match serde_json::from_str::<serde_json::Value>(&data) {
                                            Ok(zjson) => {
                                                println!("{}", &zjson.to_string());
                                            }
                                            Err(err) => {
                                                log::error!("Error while converting JSON data from ZENOH bus: {:?}", err);
                                            }
                                        }
                                    }
                                    Err(err) => {
                                        log::error!("Error while extracting data from ZENOH bus: {:?}", err);
                                    }
                                }
                            }
                            Err(err) => {
                                log::error!("Error while getting data from ZENOH bus: {:?}", err);
                            }
                        }
                    }
                }
                Err(err) => {
                    log::error!("Telemetry retival for key {} failed: {:?}", &key, err);
                    return;
                }
            }
            let _ = session.close();
            log::debug!("Connection to ZENOH bus is closed");
        }
        Err(err) => {
            log::error!("Error connecting to ZENOH bus: {:?}", err);
        }
    }
}
