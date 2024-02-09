extern crate log;
use zenoh::config::{Config};

use crate::cmd;

pub fn run(_c: &cmd::Cli, _p: &cmd::Pipeline, _zc: Config)  {
    log::trace!("zbus_pipeline::run() reached");

}
