extern crate log;
use std;
use std::io::{stdin, Read};
use crate::cmd;
use crate::stdlib::telemetry_key;

use zenoh::config::{Config};
use zenoh::prelude::sync::*;
use serde_json;

pub fn run(c: &cmd::Cli, s: &cmd::Subscribe, zc: Config)  {
    log::trace!("zbus_put::run() reached");
    log::debug!("ZENOH bus address: {}", &c.bus);

    if ! telemetry_key::telemetry_key_validate(s.key.clone()) {
        log::error!("Telemetry key is invalid");
        return;
    }

    match zenoh::open(zc).res() {
        Ok(session) => {
            log::debug!("Connection to ZENOH bus succesful");
            let key = match s.telemetry_type {
                cmd::TelemetryType::Metric => format!("zbus/metric/{}/{}", &c.protocol_version,  &s.key),
                cmd::TelemetryType::Event => format!("zbus/event/{}/{}", &c.protocol_version, &s.key),
                cmd::TelemetryType::Trace => format!("zbus/trace/{}/{}", &c.protocol_version, &s.key),
                cmd::TelemetryType::Log => format!("zbus/log/{}/{}", &c.protocol_version, &s.key)
            };
            log::debug!("Telemetry key is: {}", &key);
            match session.declare_subscriber(&key)
                    .callback_mut(move |sample| {
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
                    })
                    .res() {
                Ok(_) => {
                    for byte in stdin().bytes() {
                        match byte {
                            Ok(b'q') => break,
                            _ => std::thread::yield_now(),
                        }
                    }
                }
                Err(err) => {
                    log::error!("Telemetry subscribe for key {} failed: {:?}", &key, err);
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
