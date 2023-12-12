extern crate log;
use crate::cmd;
use reqwest;
use zenoh::config::{Config};
use zenoh::prelude::sync::*;

fn zabbix_api_get_sli(sla: &cmd::Sla, v: serde_json::Value) -> Option<serde_json::Value> {
    match reqwest::blocking::Client::new()
                .post(format!("{}/api_jsonrpc.php", sla.endpoint))
                .bearer_auth(&sla.token)
                .json(&serde_json::json!({
                    "jsonrpc": "2.0",
                    "method": "sla.getsli",
                    "id": 1,
                    "params": {
                        "slaid": v["slaid"],
                        "serviceids": v["serviceid"],
                        "periods": 1,
                    }
                }))
                .send() {
        Ok(res) => {
            match res.json::<serde_json::Value>() {
                Ok(jres) => {
                    match jres.get("result") {
                        Some(result) => {
                            match result.get("sli") {
                                Some(sli) => return Some(sli.clone()),
                                None => return None,
                            }
                        }
                        None => return None,
                    }
                }
                Err(err) => {
                    log::error!("Error decode sla.getsli: {:?}", err);
                    return None;
                }
            };
        }
        Err(err) => {
            log::error!("Error requesting sla.getsli: {:?}", err);
            return None;
        }
    }
}

fn zabbix_api_get_service(sla: &cmd::Sla, v: serde_json::Value) -> Option<Vec<serde_json::Value>> {
    let mut srv_res: Vec<serde_json::Value> = Vec::new();
    match reqwest::blocking::Client::new()
                .post(format!("{}/api_jsonrpc.php", sla.endpoint))
                .bearer_auth(&sla.token)
                .json(&serde_json::json!({
                    "jsonrpc": "2.0",
                    "method": "service.get",
                    "id": 1,
                    "params": {
                        "output": "extend",
                        "slaids": v["slaid"],
                    }
                }))
                .send() {
        Ok(res) => {
            match res.json::<serde_json::Value>() {
                Ok(jres) => {
                    match jres.get("result") {
                        Some(result) => {
                            for val in result.as_array().unwrap() {
                                srv_res.push(serde_json::json!({
                                    "name": val["name"],
                                    "id": val["serviceid"],
                                    "slaid": v["slaid"],
                                }));
                            }
                        }
                        None => {

                        }
                    }
                }
                Err(err) => {
                    log::error!("Error decoding service.get: {:?}", err);
                }
            }
        }
        Err(err) => {
            log::error!("Error requesting service.get: {:?}", err);
            return None;
        }
    }
    Some(srv_res)
}

fn zabbix_api_sla(sla: &cmd::Sla) -> Option<Vec<serde_json::Value>> {
    let mut sla_res: Vec<serde_json::Value> = Vec::new();
    match reqwest::blocking::Client::new()
                .post(format!("{}/api_jsonrpc.php", sla.endpoint))
                .bearer_auth(&sla.token)
                .json(&serde_json::json!({
                    "jsonrpc": "2.0",
                    "method": "sla.get",
                    "id": 1,
                    "params": {
                        "output": "extend",
                        "preservekeys": true,
                    }
                }))
                .send() {
        Ok(res) => {
            let jres: serde_json::Value = match res.json() {
                Ok(jres) => jres,
                Err(err) => {
                    log::error!("Error in sla.get: {:?}", err);
                    return None;
                }
            };
            match jres.get("result") {
                Some(result) => {
                    for i in result.as_object().unwrap() {
                        let (_, value) = i;
                        match zabbix_api_get_service(sla, value.clone()) {
                            Some(services) => {
                                for v in services {
                                    match zabbix_api_get_sli(sla, v.clone()) {
                                        Some(sli_result) => {
                                            for sli1 in sli_result.as_array().unwrap() {
                                                for sli2 in sli1.as_array().unwrap() {
                                                    sla_res.push(serde_json::json!({
                                                        "name": value["name"],
                                                        "service": v["name"],
                                                        "downtime": sli2["downtime"],
                                                        "uptime": sli2["uptime"],
                                                        "sli": sli2["sli"],
                                                    }));
                                                }
                                            }
                                        }
                                        None => { log::error!("SLI computation returns empty result"); }
                                    }
                                }
                            }
                            None => {

                            }
                        }
                    }
                    return Some(sla_res);
                }
                None => {
                    println!("{:?}", &jres);
                }
            }
        }
        Err(err) => {
            log::error!("Error in sla.get: {:?}", err);
        }
    }
    None
}

pub fn run(c: &cmd::Cli, sla: &cmd::Sla, zc: Config)  {
    log::trace!("zbus_export_sla_zabbix::run() reached");
    match zenoh::open(zc).res() {
        Ok(session) => {
            loop {
                match zabbix_api_sla(sla) {
                    Some(res) => {
                        for s in res {
                            let key = format!("zbus/sli/{}/{}/{}/{}", &c.protocol_version, &c.platform_name, &s["name"].as_str().unwrap(), &s["service"].as_str().unwrap());
                            match session.put(&key, s.clone()).encoding(KnownEncoding::AppJson).res() {
                                Ok(_) => { log::debug!("Submitting SLI: {}", &key); }
                                Err(err) => {
                                    log::error!("SLI submission for key {} failed: {:?}", &key, err);
                                }
                            }
                        }
                    }
                    None => {
                        log::error!("Zabbix SLA return no data");
                        break;
                    }
                }
                if sla.in_loop {
                    log::debug!("Sleeping in export thread");
                    std::thread::sleep(std::time::Duration::from_millis((1000*sla.every).into()));
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
