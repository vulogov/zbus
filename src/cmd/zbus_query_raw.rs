extern crate log;
use crate::cmd;
use zenoh::config::{Config};
use zenoh::prelude::sync::*;

pub fn run(_c: &cmd::Cli, _q: &cmd::Query, r: &cmd::QueryRaw, zc: Config)  {
    log::trace!("zbus_query_raw::run() reached");
    match zenoh::open(zc.clone()).res() {
        Ok(session) => {
            if r.all {
                log::trace!("Receiving all matched data");
                match cmd::zenoh_lib::zenoh_get_all(r.key.clone(), &session) {
                    Some(res) => {
                        for v in res {
                            println!("{}", &v.to_string().as_str());
                        }
                    }
                    None => log::info!("Query returned nothing"),
                }
            } else {
                log::trace!("Receiving first matched element");
                match cmd::zenoh_lib::zenoh_get_first(r.key.clone(), &session) {
                    Some(res) => {
                        println!("{}", &res.to_string().as_str());
                    }
                    None => log::info!("Query returned nothing"),
                }
            }
            let _ = session.close();
        }
        Err(err) => {
            log::error!("Error connecting to ZENOH bus: {:?}", err);
        }
    }
}
