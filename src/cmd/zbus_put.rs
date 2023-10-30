extern crate log;

use zenoh::config::{Config};
use zenoh::prelude::sync::*;

use parse_datetime;
use crate::stdlib::telemetry_key;
use crate::stdlib::payload;
use crate::cmd;



pub fn run(c: &cmd::Cli, p: &cmd::Put, zc: Config)  {
    log::trace!("zbus_put::run() reached");
    log::debug!("ZENOH bus address: {}", &c.bus);
    if ! telemetry_key::telemetry_key_validate(p.key.clone()) {
        log::error!("Telemetry key is invalid");
        return;
    }
    match parse_datetime::parse_datetime(&p.timestamp) {
        Ok(ts) => {
            match &ts.timestamp_nanos_opt() {
                Some(tsn) => {
                    log::debug!("Timestamp is: {:?}/{:?}", &ts, &tsn);
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
                            match payload::generate_payload(*tsn, key.clone(), &p.value) {
                                Some(data) => {
                                    log::debug!("Generated payload: {:?}", &data);
                                    match session.put(&key, data.clone()).encoding(KnownEncoding::AppJson).res() {
                                        Ok(_) => {}
                                        Err(err) => {
                                            log::error!("Telemetry submission for key {} failed: {:?}", &key, err);
                                        }
                                    }
                                }
                                None => {
                                    log::error!("Data generation return an empty result")
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
                None => {
                    log::error!("Timestamp acquisition come up empty");
                }
            }
        }
        Err(err) => {
            log::error!("Error parsing timestamp: {:?}", err);
        }
    }
}
