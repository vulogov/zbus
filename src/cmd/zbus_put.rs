extern crate log;
use parse_datetime;
use crate::stdlib::telemetry_key;
use crate::cmd;

pub fn run(c: &cmd::Cli, p: &cmd::Put)  {
    log::trace!("zbus_put::run() reached");
    log::debug!("ZENOH bus address: {}", &c.bus);
    if ! telemetry_key::telemetry_key_validate(p.key.clone()) {
        log::error!("Telemetry key is invalid");
        return;
    }
    match parse_datetime::parse_datetime(&p.timestamp) {
        Ok(ts) => {
            match &ts.timestamp_nanos_opt() {
                Some(tsn) => {
                    log::debug!("Timestamp is: {:?}/{:?}", &ts, &tsn);
                }
                None => {
                    log::error!("Timestamp acquisition come up empty");
                }
            }
        }
        Err(err) => {
            log::error!("Error parsing timestamp: {:?}", err);
        }
    }
}
