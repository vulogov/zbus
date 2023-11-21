extern crate log;
use crate::cmd;
use zenoh::config::{Config};
use zenoh::prelude::sync::*;

pub fn run(_c: &cmd::Cli, _q: &cmd::Query, r: &cmd::QueryMetadata, zc: Config)  {
    log::trace!("zbus_query_metadata::run() reached");
    match zenoh::open(zc.clone()).res() {
        Ok(session) => {
            
            let _ = session.close();
        }
        Err(err) => {
            log::error!("Error connecting to ZENOH bus: {:?}", err);
        }
    }
}
