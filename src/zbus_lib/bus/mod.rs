extern crate log;
use std::str::FromStr;
use rhai::{Engine, Map, EvalAltResult};
use rhai::plugin::*;
use zenoh::config::{Config, ConnectConfig, EndPoint, WhatAmI};
use zenoh::prelude::sync::*;

use crate::zbus_lib::sampler;

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

    pub fn feed(self: &mut Bus, key: String, mut s: sampler::Sampler) -> Result<sampler::Sampler, Box<EvalAltResult>> {
        match self.get(key.clone()) {
            Ok(value) => {
                match value.get("value") {
                    Some(v) => {
                        match value.get("ts") {
                            Some(ts) => {
                                if ts.is_float() {
                                    match s.set_and_ts(v.clone(), ts.as_float().unwrap() as f64) {
                                        Ok(_) => return Ok(s),
                                        Err(err) => {
                                            log::error!("Bus()::feed() issue for {}: {:?}", &key, err);
                                            return Err(format!("Bus()::feed() issue for {}: {:?}", &key, err).into());
                                        }
                                    }
                                } else {
                                    log::error!("Bus()::feed() timestamp issue for {}", &key);
                                    return Err(format!("Bus()::feed() timestamp issue for {}", &key).into());
                                }
                            }
                            None => {
                                log::error!("Bus()::feed() timestamp issue for {}", &key);
                                return Err(format!("Bus()::feed() timestamp issue for {}", &key).into());
                            }
                        }
                    }
                    None => {
                        log::error!("Bus()::feed() value issue for {}", &key);
                        return Err(format!("Bus()::feed() value issue for {}", &key).into());
                    }
                }
            }
            Err(err) => {
                log::error!("Bus()::feed() issue for {}: {:?}", &key, err);
                return Err(format!("Bus()::feed() issue for {}: {:?}", &key, err).into());
            }
        }
    }

    pub fn collect_n_values(self: &mut Bus, key: String, n: i64) -> Result<sampler::Sampler, Box<EvalAltResult>> {
        let mut ret_data = sampler::Sampler::init();
        match zenoh::open(self.zc.clone()).res() {
            Ok(session) => {
                let mut c: i64 = 0_i64;
                match session.declare_subscriber(&key).res() {
                    Ok(subscriber) => {
                        while c < n {
                            match subscriber.recv() {
                                Ok(sample) => {
                                    let slices = &sample.value.payload.contiguous();
                                    match std::str::from_utf8(slices) {
                                        Ok(data) => {
                                            match serde_json::from_str::<serde_json::Value>(&data) {
                                                Ok(zjson) => {
                                                    match zjson.get("value") {
                                                        Some(v) => {
                                                            match zjson.get("ts") {
                                                                Some(ts) => {
                                                                    if ts.is_i64() {
                                                                        match v.as_f64() {
                                                                            Some(v_n) => {
                                                                                match ret_data.set_and_ts(Dynamic::from(v_n), ts.as_i64().unwrap() as f64) {
                                                                                    Ok(_) => {
                                                                                        c += 1;
                                                                                    },
                                                                                    Err(err) => {
                                                                                        log::error!("Bus()::feed() issue for {}: {:?}", &key, err);
                                                                                        return Err(format!("Bus()::feed() issue for {}: {:?}", &key, err).into());
                                                                                    }
                                                                                }
                                                                            }
                                                                            None => {
                                                                                log::error!("Bus()::feed() issue for {}: Value must be numeric", &key);
                                                                                return Err(format!("Bus()::feed() issue for {}: Value must be numeric", &key).into());
                                                                            }
                                                                        }
                                                                    } else {
                                                                        log::error!("Bus()::feed() timestamp issue for {}", &key);
                                                                        return Err(format!("Bus()::feed() timestamp issue for {}", &key).into());
                                                                    }
                                                                }
                                                                None => {
                                                                    log::error!("Bus()::feed() timestamp issue for {}", &key);
                                                                    return Err(format!("Bus()::feed() timestamp issue for {}", &key).into());
                                                                }
                                                            }
                                                        }
                                                        None => {
                                                            log::error!("Bus()::collect() value issue for {}", &key);
                                                        }
                                                    }
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
                                    log::error!("Bus()::recv() problem: {:?}", err);
                                }
                            }
                        }
                    }
                    Err(err) => {
                        log::error!("Error subscribing to Bus(): {:?}", err);
                        return Err(format!("Error subscribing to Bus(): {:?}", err).into());
                    }
                }
                let _ = session.close();
                log::debug!("Connection to ZENOH bus is closed");
            }
            Err(err) => {
                log::error!("Error opening Bus() session: {:?}", err);
                return Err(format!("Error opening Bus() session: {:?}", err).into());
            }
        }
        Ok(ret_data)
    }

    pub fn collect(self: &mut Bus, key: String) -> Result<sampler::Sampler, Box<EvalAltResult>> {
        self.collect_n_values(key, 128)
    }

    pub fn send(self: &mut Bus, key: String, mut data: sampler::Sampler) -> Result<i64, Box<EvalAltResult>> {
        let parsed_key: Vec<&str> = key.as_str().split("/").collect();
        let skey = &parsed_key[&parsed_key.len() - 1];
        let src = &parsed_key[&parsed_key.len() - 2];
        let platform = &parsed_key[&parsed_key.len() - 3];
        let mut c: i64 = 0_i64;
        for v in data.values().unwrap() {
            let ts  = &v.clone().cast::<Vec<Dynamic>>()[0];
            let val = &v.clone().cast::<Vec<Dynamic>>()[1];
            if ts.as_float().unwrap() != 0.0 {
                let zjson = serde_json::json!({
                    "skey":         skey,
                    "src":          src,
                    "platform":     platform,
                    "key":          key,
                    "ts":           ts.as_float().unwrap(),
                    "value":        val.as_float().unwrap(),
                });
                let zdata: Map = rhai::serde::to_dynamic(zjson).unwrap().cast::<Map>();
                self.put(key.clone(), zdata);
                c += 1;
            }
        }
        Ok(c)
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
          .register_fn("feed", Bus::feed)
          .register_fn("send", Bus::send)
          .register_fn("collect", Bus::collect)
          .register_fn("collect_n_values", Bus::collect_n_values)
          .register_fn("to_string", |x: &mut Bus| format!("{:?}", x) );
    let module = exported_module!(bus_module);
    engine.register_static_module("bus", module.into());
}
