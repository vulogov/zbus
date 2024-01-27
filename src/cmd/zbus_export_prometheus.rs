extern crate log;
use crate::cmd;
use zenoh::config::{Config};
use zenoh::prelude::sync::*;
use crate::stdlib::getfile::{get_file_from_uri};


pub fn run(c: &cmd::Cli, prometheus: &cmd::Prometheus, zc: Config)  {
    log::trace!("zbus_export_prometheus::run() reached");
    match zenoh::open(zc).res() {
        Ok(session) => {
            loop {
                match get_file_from_uri(prometheus.exporter.clone()) {
                    Some(data) => {
                        let lines: Vec<_> = data.lines().map(|s| Ok(s.to_owned())).collect();
                        match prometheus_parse::Scrape::parse(lines.into_iter()) {
                            Ok(metrics) => {
                                for s in &metrics.samples {
                                    let zkey_raw = cmd::prometheus_lib::prometheus_key_to_zenoh(&s);
                                    let zkey = format!("zbus/metric/{}/{}/{}/{}", &c.protocol_version, &c.platform_name, &prometheus.source, &zkey_raw);
                                    match s.value {
                                        prometheus_parse::Value::Counter(value) |
                                        prometheus_parse::Value::Untyped(value) |
                                        prometheus_parse::Value::Gauge(value) => {
                                            let payload = serde_json::json!({
                                                    "platform":   &c.platform_name,
                                                    "key":        &zkey,
                                                    "skey":       &zkey_raw,
                                                    "ts":         &s.timestamp.timestamp_nanos_opt().unwrap(),
                                                    "value":      &value.clone(),
                                            });
                                            match session.put(zkey.clone(), payload.clone()).encoding(KnownEncoding::AppJson).res() {
                                                Ok(_) => log::debug!("PROMETHEUS->ZBUS: {}", &zkey),
                                                Err(err) => log::error!("Error ingesting {} {:?}: {:?}", &payload["key"], &payload, err),
                                            }
                                        }
                                        _ => continue,
                                    }
                                }
                            }
                            Err(err) => {
                                log::error!("Error parsing Prometheus responce: {:?}", err);
                            }
                        }
                    }
                    None => {
                        log::error!("Prometheus exporter did not returned any data");
                    }
                }
                if prometheus.in_loop {
                    log::debug!("Sleeping in export thread");
                    std::thread::sleep(std::time::Duration::from_millis((1000*prometheus.every).into()));
                } else {
                    log::debug!("Breaking from export thread");
                    break;
                }
            }
            let _ = session.close().res();
            log::debug!("Session to ZENOH bus is closed");
        }
        Err(err) => {
            log::error!("Error connecting to ZENOH bus: {:?}", err);
        }
    }
}
