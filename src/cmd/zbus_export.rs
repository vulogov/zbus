extern crate log;
use crate::cmd;
use zenoh::config::{Config};

pub fn run(c: &cmd::Cli, exp: &cmd::Export, zc: Config)  {
    log::trace!("zbus_export::run() reached");
    match &exp.command {
        cmd::ExportCommands::History(history) => {
            match history.source {
                cmd::TelemetrySources::Zabbix => {
                    cmd::zbus_export_zabbix::run(c, history, zc.clone());
                }
            }
        }
        cmd::ExportCommands::Sla(_sla) => {
        }
    }
}
