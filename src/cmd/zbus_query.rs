extern crate log;
use crate::cmd;
use zenoh::config::{Config};

pub fn run(c: &cmd::Cli, q: &cmd::Query, zc: Config)  {
    log::trace!("zbus_query::run() reached");
    match &q.command {
        cmd::QueryCommands::QueryRaw(raw) => {
            cmd::zbus_query_raw::run(c, q, &raw, zc.clone());
        }
        cmd::QueryCommands::QueryMetadata(metadata) => {
            cmd::zbus_query_metadata::run(c, q, &metadata, zc.clone());
        }
    }
}
