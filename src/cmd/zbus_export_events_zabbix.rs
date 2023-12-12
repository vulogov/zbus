extern crate log;
use crate::cmd;
use zenoh::config::{Config};
use zenoh::prelude::sync::*;


pub fn run(_c: &cmd::Cli, events: &cmd::Events, zc: Config)  {
    log::trace!("zbus_export_events_zabbix::run() reached");
    match zenoh::open(zc).res() {
        Ok(session) => {
            loop {

                if events.in_loop {
                    log::debug!("Sleeping in export thread");
                    std::thread::sleep(std::time::Duration::from_millis((1000*events.every).into()));
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
