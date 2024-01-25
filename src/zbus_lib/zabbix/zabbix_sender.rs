extern crate log;
use std::io::{Read, Write};
use rhai::{Dynamic, EvalAltResult, Map, Array};

use serde_json;
use rhai::serde::{to_dynamic};
use crate::zbus_lib::sampler;
use crate::zbus_lib::timestamp::timestamp_module::{timestamp_ns, whole_seconds, nanoseconds};
use crate::zbus_lib::zabbix::{ZabbixError};

#[derive(Debug, Clone)]
pub struct ZabbixSender {
    pub addr:   String,
}

impl ZabbixSender {
    pub fn new(addr: String) -> Self {
        Self {
            addr:   addr,
        }
    }

    pub fn send_with_current_timestamp(self: &mut ZabbixSender, host: String, key: String, value: Dynamic) -> Result<Dynamic, Box<EvalAltResult>> {
        self.send(host, key, timestamp_ns(), value)
    }

    pub fn send_sampler(self: &mut ZabbixSender, host: String, key: String, mut data: sampler::Sampler) -> Result<Array, Box<EvalAltResult>> {
        let mut res = Array::new();
        for (t,v) in data.values_raw() {
            match self.send(host.clone(), key.clone(), t, Dynamic::from(v)) {
                Ok(ret) => res.push(ret),
                Err(_) => {}
            }
        }
        Ok(res)
    }

    pub fn send(self: &mut ZabbixSender, host: String, key: String, t: f64, value: Dynamic) -> Result<Dynamic, Box<EvalAltResult>> {
        let data = serde_json::json!({
            "request": "sender data",
            "data": [{
                "host":     host,
                "key":      key,
                "value":    format!("{}", &value),
                "clock":    whole_seconds(t) as i64,
                "ns":       nanoseconds(t) as i64,
            },]
        });
        log::trace!("ZabbixSender::send() payload: {:?}", &data.to_string());
        match self.addr.parse::<std::net::SocketAddr>() {
            Ok(address) => {
                match send_metric(address, serde_json::ser::to_vec(&data).unwrap()) {
                    Ok(res) => Ok(res),
                    Err(err) => Err(format!("ZabbixSender error: {:?}", err).into()),
                }
            }
            Err(err) => {
                log::error!("Error parsing address for ZabbixSender::send(): {}", err);
                return Err(format!("Error parsing address for ZabbixSender::send(): {}", err).into());
            }
        }
    }
}


pub fn send_metric(
    addr: std::net::SocketAddr,
    data: Vec<u8>,
) -> Result<Dynamic, Box<dyn std::error::Error>> {
    let mut buffer = vec![];
    log::trace!("zabbix::zabbix_sender[{}:{}", &addr.ip(), &addr.port());
    let mut sock = std::net::TcpStream::connect(addr)?;
    log::trace!("zabbix::zabbix_sender connected");
    sock.write(
        &[
            "ZBXD\x01".as_bytes(),
            (data.len() as u32).to_le_bytes().as_ref(),
            0u32.to_le_bytes().as_ref(),
            &data,
        ]
        .concat(),
    )?;
    sock.read_to_end(&mut buffer)?;
    log::trace!("zabbix::zabbix_sender received");
    let _ = sock.shutdown(std::net::Shutdown::Both);
    log::trace!("zabbix::zabbix_sender closed");
    assert_eq!(&buffer[0..5], &[90, 66, 88, 68, 1]);
    let len = u32::from_le_bytes([buffer[5], buffer[6], buffer[7], buffer[8]]) as usize;
    let response = String::from_utf8_lossy(&buffer[13..13 + len]).to_string();
    if response.starts_with("ZBX_NOTSUPPORTED\0") {
        Err(Box::new(ZabbixError::SenderError(
            response.split('\0').nth(1).unwrap_or_default().to_owned(),
        )))
    } else {
        match to_dynamic(response) {
            Ok(res) => return Ok(res),
            Err(err) => return Err(format!("zabbix::zabbix_sender error converting result: {:?}", err).into()),
        }
    }
}

pub fn zabbix_sender(addr: String, data: Map) -> Result<Dynamic, Box<EvalAltResult>> {
    match addr.parse::<std::net::SocketAddr>() {
        Ok(address) => {
            match serde_json::ser::to_vec(&data) {
                Ok(jdata) => {
                    match send_metric(address, jdata) {
                        Ok(res) => return Ok(res),
                        Err(err) => return Err(format!("zabbix::zabbix_sender error: {:?}", err).into()),
                    }
                }
                Err(err) => {
                    log::error!("Error parsing data: {}", err);
                    return Err(format!("Error parsing data: {}", err).into());
                }
            }
        }
        Err(err) => {
            log::error!("Error parsing address for zabbix::zabbix_get: {}", err);
            return Err(format!("Error parsing address for zabbix::zabbix_get: {}", err).into());
        }
    }
}
