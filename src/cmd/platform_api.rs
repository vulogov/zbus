extern crate log;
use crate::cmd;
use zenoh::config::{Config};

pub fn run(c: &cmd::Cli, api: &cmd::Api, zc: Config)  {
    log::trace!("platform_api::run() reached");
    match api.source {
        cmd::TelemetrySources::Zabbix => {
            cmd::zabbix_api::run(c, api, zc.clone());
        }
    }
}
