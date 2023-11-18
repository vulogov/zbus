extern crate log;
use reqwest;
use serde_json;
use crate::cmd;
use crate::cmd::zabbix_lib;
use crate::cmd::{Login, Metadata};
use zenoh::config::{Config};

fn zabbix_api_login(api: &cmd::Api, login: &Login) -> Option<String> {
    match reqwest::blocking::Client::new()
                .post(format!("{}/api_jsonrpc.php", api.endpoint))
                .json(&serde_json::json!({
                    "jsonrpc": "2.0",
                    "method": "user.login",
                    "id": 1,
                    "params": {
                        "username": &login.login,
                        "password": &login.password,
                    }
                }))
                .send() {
        Ok(res) => {
            let jres: serde_json::Value = match res.json() {
                Ok(jres) => jres,
                Err(err) => {
                    log::error!("Error in user.login: {:?}", err);
                    return None;
                }
            };
            match &jres.get("result") {
                Some(result) => {
                    return Some(result.as_str()?.to_string());
                }
                None => {

                }
            }
        }
        Err(err) => {
            log::error!("Error in user.login: {:?}", err);
        }
    }
    None
}

fn zabbix_api_metadata(api: &cmd::Api, meta: &Metadata) -> Option<Vec<serde_json::Value>> {
    match reqwest::blocking::Client::new()
                .post(format!("{}/api_jsonrpc.php", api.endpoint))
                .bearer_auth(&meta.token)
                .json(&serde_json::json!({
                    "jsonrpc": "2.0",
                    "method": "item.get",
                    "id": 1,
                    "params": {
                        "templated": false,
                    }
                }))
                .send() {
        Ok(res) => {
            let jres: serde_json::Value = match res.json() {
                Ok(jres) => jres,
                Err(err) => {
                    log::error!("Error in item.get: {:?}", err);
                    return None;
                }
            };
            match jres.get("result") {
                Some(result) => {
                    match result.as_array() {
                        Some(ares) => {
                            return Some(ares.to_vec());
                        }
                        None => {

                        }
                    }
                }
                None => {
                    println!("{:?}", &jres);
                }
            }
        }
        Err(err) => {
            log::error!("Error in item.get: {:?}", err);
        }
    }
    None
}

pub fn run(_c: &cmd::Cli, api: &cmd::Api, _zc: Config)  {
    log::trace!("zabbix_api::run() reached");
    match &api.command {
        cmd::ApiCommands::Login(login) => {
            log::debug!("zabbix::api::login reached");
            match zabbix_api_login(api, &login) {
                Some(res) => {
                    println!("{}", res.as_str());
                }
                None => {
                    log::error!("zabbix::api::login did not return anything");
                    return
                }
            }
        }
        cmd::ApiCommands::Metadata(metadata) => {
            log::debug!("zabbix::api::metadata reached");
            match zabbix_api_metadata(api, &metadata) {
                Some(res) => {
                    for v in res {
                        let hostid = &v["hostid"].as_str().expect("string expected").to_string();
                        let itemid = &v["itemid"].as_str().expect("string expected").to_string();
                        match zabbix_lib::zabbix_key_to_zenoh((&hostid).to_string(), (&itemid).to_string(), (&v["key_"].as_str().expect("string expected")).to_string()) {
                            Some(key) => {
                                println!("{} {} {}",
                                    &hostid,
                                    &itemid,
                                    key
                                );
                            }
                            None => continue,
                        }
                    }
                }
                None => {
                    log::error!("zabbix::api::metadata did not return anything");
                    return
                }
            }
        }
    }
}
