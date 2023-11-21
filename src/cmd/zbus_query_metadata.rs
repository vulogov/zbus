extern crate log;
use crate::cmd;
use zenoh::config::{Config};
use zenoh::prelude::sync::*;

pub fn run(_c: &cmd::Cli, _q: &cmd::Query, r: &cmd::QueryMetadata, zc: Config)  {
    log::trace!("zbus_query_metadata::run() reached");
    match zenoh::open(zc.clone()).res() {
        Ok(session) => {
            if r.convert {
                match cmd::zenoh_lib::get_key_from_metadata(r.hostid.clone(), r.itemid.clone(), &session) {
                    Some(key) => {
                        match cmd::zabbix_lib::zabbix_key_to_zenoh(key) {
                            Some(key) => println!("{}", key),
                            None => log::info!("Key not found"),
                        }
                    }
                    None => log::info!("Key not found"),
                }
            } else {
                match cmd::zenoh_lib::get_key_from_metadata(r.hostid.clone(), r.itemid.clone(), &session) {
                    Some(key) => println!("{}", &key.as_str()),
                    None => log::info!("Key not found"),
                }
            }
            let _ = session.close();
        }
        Err(err) => {
            log::error!("Error connecting to ZENOH bus: {:?}", err);
        }
    }
}
