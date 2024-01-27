extern crate log;
use crate::cmd;
use crate::stdlib::getfile;
use zenoh::config::{Config};
use crate::zbus_lib;


pub fn run(c: &cmd::Cli, s: &cmd::Script, _zc: Config)  {
    log::trace!("zbus_script::run() reached");
    if s.stdin {
        zbus_lib::run_zbus_script(getfile::get_file_from_stdin(), c, s)
    } else if ! s.file.trim().is_empty() {
        match getfile::get_file_from_file(s.file.trim().to_string()) {
            Some(script) => zbus_lib::run_zbus_script(script, c, s),
            None => log::error!("Script is empty"),
        }
    } else if ! s.uri.trim().is_empty() {
        match getfile::get_file_from_uri(s.uri.trim().to_string()) {
            Some(script) => zbus_lib::run_zbus_script(script, c, s),
            None => log::error!("Script is empty"),
        }
    } else if ! s.eval.trim().is_empty() {
        zbus_lib::run_zbus_script(s.eval.trim().to_string(), c, s)
    } else {
        zbus_lib::run_zbus_script(getfile::get_file_from_stdin(), c, s)
    }
}
