extern crate log;
use rhai::{EvalAltResult};
use crate::zbus_lib::bus::Bus;
use std::thread;
use zenoh::prelude::sync::*;

impl Bus {
    pub fn subscribe(&mut self, key: String) -> Result<bool, Box<EvalAltResult>> {
        let zc = self.zc.clone();
        let _ = thread::spawn(move || {
                match zenoh::open(zc).res() {
                    Ok(session) => {
                        match session.declare_subscriber(&key)
                                .callback_mut(move |sample| {
                                    let slices = &sample.value.payload.contiguous();
                                    match std::str::from_utf8(slices) {
                                        Ok(data) => {
                                            match serde_json::from_str::<serde_json::Value>(&data) {
                                                Ok(_zjson) => {

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
                                std::thread::yield_now();
                            }
                            Err(err) => {
                                log::error!("Telemetry subscribe for key {} failed: {:?}", &key, err);
                                // return Err(format!("Telemetry subscribe for key {} failed: {:?}", &key, err).into());
                            }
                        }
                        let _ = session.close();
                        log::debug!("Connection to ZENOH bus is closed");
                    }
                    Err(err) => {
                        log::error!("Error connecting to ZENOH bus: {:?}", &err);
                        // return Err(format!("Error connecting to ZENOH bus: {:?}", &err).into());
                    }
                }
            }
        );
        Ok(true)
    }
}
