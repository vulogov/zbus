extern crate log;
use crate::cmd;

pub fn run(c: &cmd::Cli)  {
    log::trace!("zbus_get::run() reached");
    log::debug!("ZENOH bus address: {}", &c.bus);

}
