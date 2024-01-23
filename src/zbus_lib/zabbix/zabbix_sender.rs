extern crate log;
use std::io::{Read, Write};
use rhai::{Dynamic, EvalAltResult};

use std::time::Duration;
use std::thread;

use crate::zbus_lib::sampler;

// #[derive(Debug, Clone)]
// pub struct ZabbixAgent {
//     pub addr: String,
// }
//
// impl ZabbixAgent {
//     pub fn new(addr: String) -> Self {
//         Self {
//             addr: addr,
//         }
//     }
//     pub fn get(self: &mut ZabbixAgent, key: String) -> Result<Dynamic, Box<EvalAltResult>> {
//         zabbix_get(self.addr.clone(), key)
//     }
//     pub fn get_to_sampler(self: &mut ZabbixAgent, key: String, mut data: sampler::Sampler) -> Result<sampler::Sampler, Box<EvalAltResult>> {
//         match self.get(key) {
//             Ok(value) => {
//                 match data.set(value) {
//                     Ok(_) => return Ok(data),
//                     Err(err) => return Err(err),
//                 }
//             }
//             Err(err) => return Err(err),
//         }
//     }
//     pub fn get_sampler(self: &mut ZabbixAgent, key: String, n: i64, t: i64) -> Result<sampler::Sampler, Box<EvalAltResult>> {
//         let mut c: i64 = 0;
//         let mut data = sampler::Sampler::init();
//         while c < n {
//             match self.get_to_sampler(key.clone(), data) {
//                 Ok(ndata) => data = ndata,
//                 Err(err) => return Err(err),
//             }
//             c += 1;
//             thread::sleep(Duration::from_secs(t as u64));
//         }
//         Ok(data)
//     }
// }
//
// #[derive(Debug)]
// pub enum ZabbixError {
//     NotSupported(String),
// }
//
// impl std::fmt::Display for ZabbixError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             ZabbixError::NotSupported(e) => f.write_fmt(format_args!("ZabbixNotSupported ({})", e)),
//         }
//     }
// }
// impl std::error::Error for ZabbixError {
//     fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
//         None
//     }
// }
//
// /// Run a passive check on a specified host and return the result
// pub fn send_metric(
//     addr: std::net::SocketAddr,
//     name: &str,
// ) -> Result<String, Box<dyn std::error::Error>> {
//     let mut buffer = vec![];
//     log::trace!("input::zabbix[{}:{}", &addr.ip(), &addr.port());
//     let mut sock = std::net::TcpStream::connect(addr)?;
//     log::trace!("input::zabbix connected");
//     sock.write(
//         &[
//             "ZBXD\x01".as_bytes(),
//             (name.len() as u32).to_le_bytes().as_ref(),
//             0u32.to_le_bytes().as_ref(),
//             name.as_bytes(),
//         ]
//         .concat(),
//     )?;
//     sock.read_to_end(&mut buffer)?;
//     log::trace!("input::zabbix received");
//     let _ = sock.shutdown(std::net::Shutdown::Both);
//     log::trace!("input::zabbix closed");
//     assert_eq!(&buffer[0..5], &[90, 66, 88, 68, 1]);
//     let len = u32::from_le_bytes([buffer[5], buffer[6], buffer[7], buffer[8]]) as usize;
//     let response = String::from_utf8_lossy(&buffer[13..13 + len]).to_string();
//     if response.starts_with("ZBX_NOTSUPPORTED\0") {
//         Err(Box::new(ZabbixError::NotSupported(
//             response.split('\0').nth(1).unwrap_or_default().to_owned(),
//         )))
//     } else {
//         Ok(response)
//     }
// }
//
// pub fn zabbix_get(addr: String, key: String) -> Result<Dynamic, Box<EvalAltResult>> {
//     match addr.parse::<std::net::SocketAddr>() {
//         Ok(address) => {
//             match get_metric(address, &key) {
//                 Ok(res) => {
//                     log::trace!("Get from Zabbix {}: {}]", &addr, &res);
//                     return Ok(Dynamic::from(res));
//                 }
//                 Err(err) => {
//                     log::error!("Error getting data from Zabbix agent: {}", err);
//                     return Err(format!("Error getting data from Zabbix agent: {}", err).into());
//                 }
//             }
//         }
//         Err(err) => {
//             log::error!("Error parsing address for input::zabbix: {}", err);
//             return Err(format!("Error parsing address for input::zabbix: {}", err).into());
//         }
//     }
//
// }
