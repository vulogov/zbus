extern crate log;
use crate::cmd;
use rust_search::SearchBuilder;
use zenoh::config::{Config};
use zenoh::prelude::sync::*;
use easy_reader::EasyReader;
use std::fs::File;
use unit_conversions;

pub fn convert_zabbix_export_payload_to_zbus(key: String, platform: String, payload: serde_json::Value) -> Option<serde_json::Value> {
    match cmd::zabbix_lib::zabbix_key_to_zenoh(key.clone()) {
        Some(zkey) => {
            let timestamp = unit_conversions::time::seconds::to_nanoseconds(payload["clock"].as_f64().unwrap()) + payload["ns"].as_f64().unwrap();
            let mut host = payload["host"]["host"].to_string();
            if host.chars().nth(0) == Some('\"') {
                host = (&host[1..host.len() - 1]).to_string();
                log::debug!("Host is: {}", &host);
            }
            return Some(serde_json::json!({
                "ts": timestamp as u64,
                "platform": platform,
                "src": host,
                "skey": key,
                "key": zkey,
                "value": payload["value"],
                "name": payload["name"],
            }));
        }
        None => return None,
    }
}

pub fn run(c: &cmd::Cli, exp: &cmd::History, zc: Config)  {
    log::trace!("zbus_export_zabbix::run() reached");
    match zenoh::open(zc).res() {
        Ok(session) => {
            loop {
                let search: Vec<String> = SearchBuilder::default()
                    .location(exp.path.clone())
                    .search_input(exp.search.clone())
                    .ext(exp.extension.clone())
                    .depth(1)
                    .build()
                    .collect();
                    for name in search {
                        log::debug!("Processing input file with Zabbix JSON: {}", &name);
                        match File::open(name) {
                            Ok(file) => {
                                match EasyReader::new(file) {
                                    Ok(mut reader) => {
                                        let _ = reader.build_index();
                                        reader.bof();
                                        loop {
                                            match reader.next_line() {
                                                Ok(Some(line)) => {
                                                    match serde_json::from_str::<serde_json::Value>(&line) {
                                                        Ok(zjson) => {
                                                            match cmd::zenoh_lib::get_key_from_metadata(c.platform_name.clone(), "*".to_string(), zjson["itemid"].to_string(), &session) {
                                                                Some(key) => {
                                                                    match convert_zabbix_export_payload_to_zbus(key, c.platform_name.clone(), zjson.clone()) {
                                                                        Some(payload) => {
                                                                            let store_key = match zjson["type"].as_i64() {
                                                                                Some(2) => format!("log/metric/{}/{}/{}{}", &c.protocol_version, &c.platform_name, &payload["src"].as_str().unwrap(), &payload["key"].as_str().unwrap()),
                                                                                _ => format!("zbus/metric/{}/{}/{}{}", &c.protocol_version, &c.platform_name, &payload["src"].as_str().unwrap(), &payload["key"].as_str().unwrap()),
                                                                            };
                                                                            match session.put(store_key.clone(), payload.clone()).encoding(KnownEncoding::AppJson).res() {
                                                                                Ok(_) => log::debug!("ZBX->ZBUS: {}", &store_key),
                                                                                Err(err) => log::error!("Error ingesting {} {:?}: {:?}", &payload["key"], &payload, err),
                                                                            }
                                                                        }
                                                                        None => continue,
                                                                    }
                                                                }
                                                                None => continue,
                                                            }
                                                        }
                                                        Err(err) => {
                                                            log::error!("Error while converting JSON data from ZENOH bus: {:?}", err);
                                                        }
                                                    }
                                                }
                                                Ok(None) => break,
                                                _ => break,
                                            }
                                        }
                                    }
                                    Err(err) => {
                                        log::error!("input::textfile:: : {}", err);
                                    }
                                }
                            }
                            Err(err) => {
                                log::error!("input::textfile:: : {}", err);
                            }
                        }
                }
                if exp.in_loop {
                    log::debug!("Sleeping in export thread");
                    std::thread::sleep(std::time::Duration::from_millis((1000*exp.every).into()));
                } else {
                    log::debug!("Breaking from export thread");
                    break;
                }
            }
            let _ = session.close().res();
            log::debug!("Session to ZENOH bus is closed");
        }
        Err(err) => {
            log::error!("Error connecting to ZENOH bus: {:?}", err);
        }
    }
}
