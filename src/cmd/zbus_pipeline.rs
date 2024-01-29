extern crate log;
use zenoh::config::{Config};
use zenoh::prelude::sync::*;

use crate::cmd;

pub fn run(c: &cmd::Cli, p: &cmd::Pipeline, zc: Config)  {
    log::trace!("zbus_pipeline::run() reached");

}
