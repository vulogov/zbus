extern crate log;
use std::str::FromStr;
use rhai::{Engine, Map, EvalAltResult};
use rhai::plugin::*;
use zenoh::config::{Config, ConnectConfig, EndPoint, WhatAmI};
use zenoh::prelude::sync::*;

#[derive(Debug, Clone)]
pub struct Bus {
    state:      bool,
    bus_addr:   String,
    zc:         Config,
}

impl Bus {
    fn new(address: String) -> Self {
        Self {
            state:      true,
            bus_addr:   address,
            zc:         Config::default(),
        }
    }
    pub fn init(address: String) -> Bus {
        let mut res = Bus::new(address.clone());
        res.state = false;
        match res.zc.scouting.multicast.set_enabled(Some(false)) {
            Ok(_) => {}
            Err(err) => {
                log::error!("Error configuring Bus(): {:?}", err);
                return res;
            }
        }
        match EndPoint::from_str(&address) {
            Ok(zconn) => {
                log::debug!("ZENOH bus set to: {:?}", &zconn);
                let _ = res.zc.set_connect(ConnectConfig::new(vec![zconn]).unwrap());
            }
            Err(err) => {
                log::error!("Failure in parsing connect address: {:?}", err);
                return res;
            }
        }
        match res.zc.set_mode(Some(WhatAmI::Client)) {
            Ok(_) => {},
            Err(err) => {
                log::error!("Error configuring Bus(): {:?}", err);
                return res;
            }
        }
        if res.zc.validate() {
            log::debug!("ZENOH config is OK");
        } else {
            log::error!("ZENOH config not OK");
            return res;
        }
        res.state = true;
        res
    }
    pub fn state(self: &mut Bus) -> bool {
        self.state
    }
    pub fn address(self: &mut Bus) -> String {
        self.bus_addr.clone()
    }
    pub fn put(self: &mut Bus, key: String, value: Map) -> bool {
        match zenoh::open(self.zc.clone()).res() {
            Ok(session) => {
                log::debug!("Bus({}) session established", self.address());
                match serde_json::to_string(&value) {
                    Ok(data) => {
                        log::debug!("Bus()::put() len() = {}", &data.len());
                        match session.put(&key, data.clone()).encoding(KnownEncoding::AppJson).res() {
                            Ok(_) => {
                                log::debug!("Bus()::put() submission for key {} OK", &key);
                                let _ = session.close().res();
                                return true;
                            }
                            Err(err) => {
                                log::error!("Bus()::put() submission for key {} failed: {:?}", &key, err);
                            }
                        }
                    }
                    Err(err) => {
                        log::error!("Error generating payload for Bus()::put(): {:?}", err);
                    }
                }
            }
            Err(err) => {
                log::error!("Error opening Bus() session: {:?}", err);
            }
        }
        return false;
    }
    pub fn get(self: &mut Bus, key: String) -> Result<Map, Box<EvalAltResult>> {
        match zenoh::open(self.zc.clone()).res() {
            Ok(session) => {
                log::debug!("Bus({}) session established", self.address());
                match session.get(&key).res() {
                    Ok(replies) => {
                        while let Ok(reply) = replies.recv() {
                            match reply.sample {
                                Ok(sample) => {
                                    let slices = &sample.value.payload.contiguous();
                                    match std::str::from_utf8(slices) {
                                        Ok(data) => {
                                            match serde_json::from_str::<Map>(&data) {
                                                Ok(mut zjson) => {
                                                    let _ = zjson.insert("__valid".into(), true.into());
                                                    return Ok(zjson);
                                                }
                                                Err(err) => {
                                                    log::error!("Error while converting JSON data from ZENOH bus: {:?}", err);
                                                    return Err(format!("Error while converting JSON data from ZENOH bus: {:?}", err).into());
                                                }
                                            }
                                        }
                                        Err(err) => {
                                            log::error!("Error while extracting data from ZENOH bus: {:?}", err);
                                            return Err(format!("Error while extracting data from ZENOH bus: {:?}", err).into());
                                        }
                                    }
                                }
                                Err(err) => {
                                    log::error!("Error while getting data from ZENOH bus: {:?}", err);
                                    return Err(format!("Error while getting data from ZENOH bus: {:?}", err).into());
                                }
                            }
                        }
                    }
                    Err(err) => {
                        log::error!("Bus()::get() for {} failed: {:?}", &key, err);
                        return Err(format!("Bus()::get() for {} failed: {:?}", &key, err).into());
                    }
                }
            }
            Err(err) => {
                log::error!("Error opening Bus() session: {:?}", err);
                return Err(format!("Error opening Bus() session: {:?}", err).into());
            }
        }
        return Err(format!("Bus()::get() key not found: {}", &key).into());
    }
}


#[export_module]
pub mod bus_module {
}

pub fn init(engine: &mut Engine) {
    log::trace!("Running STDLIB::bus init");
    engine.register_type::<Bus>()
          .register_fn("Bus", Bus::init)
          .register_fn("state", Bus::state)
          .register_fn("address", Bus::address)
          .register_fn("put", Bus::put)
          .register_fn("get", Bus::get)
          .register_fn("to_string", |x: &mut Bus| format!("{:?}", x) );
    let module = exported_module!(bus_module);
    engine.register_static_module("bus", module.into());
}
