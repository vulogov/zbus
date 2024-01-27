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
        cmd::ExportCommands::Sla(sla) => {
            match sla.source {
                cmd::TelemetrySources::Zabbix => {
                    cmd::zbus_export_sla_zabbix::run(c, sla, zc.clone());
                }
            }
        }
        cmd::ExportCommands::Events(events) => {
            match events.source {
                cmd::TelemetrySources::Zabbix => {
                    cmd::zbus_export_events_zabbix::run(c, events, zc.clone());
                }
            }
        }
        cmd::ExportCommands::Prometheus(prometheus) => {
            cmd::zbus_export_prometheus::run(c, prometheus, zc.clone());
        }
    }
}
