extern crate log;
use crate::cmd;
use rust_search::SearchBuilder;
use zenoh::config::{Config};
use zenoh::prelude::sync::*;
use easy_reader::EasyReader;
use std::fs::File;

pub fn run(_c: &cmd::Cli, exp: &cmd::Export, zc: Config)  {
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
                                                            println!("{}", &zjson.to_string().as_str());
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
