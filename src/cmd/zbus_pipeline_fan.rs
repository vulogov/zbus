extern crate log;
use zenoh::config::{Config};

use crate::cmd;

pub fn run(_c: &cmd::Cli, _pipeline: &cmd::Pipeline, _fan: &cmd::PipelineFan, _zc: Config)  {
    log::trace!("zbus_pipeline_fan::run() reached");

}
