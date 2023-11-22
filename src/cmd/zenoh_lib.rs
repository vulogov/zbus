extern crate log;
use serde_json;
use zenoh::prelude::sync::*;

pub fn get_key_from_metadata(platform: String, hostid: String, itemid: String, session: &Session) -> Option<String> {
    match zenoh_get_first(format!("zbus/metadata/v1/{}/{}/{}", platform, hostid, itemid).to_string(), session) {
        Some(mdata) => {
            match mdata.get("key_") {
                Some(key) => return Some(key.as_str()?.to_string()),
                None => return None,
            }
        }
        None => {
            return None;
        }
    }
}

pub fn zenoh_put(key: String, payload: String, session: &Session) {
    match session.put(&key, payload.clone()).encoding(KnownEncoding::AppJson).res() {
        Ok(_) => {}
        Err(err) => {
            log::error!("Metadata submission for key {} failed: {:?}", &key, err);
        }
    }
}

pub fn zenoh_get_first(key: String, session: &Session) -> Option<serde_json::Value> {
    match session.get(&key).res() {
        Ok(replies) => {
            while let Ok(reply) = replies.recv() {
                match reply.sample {
                    Ok(sample) => {
                        let slices = &sample.value.payload.contiguous();
                        match std::str::from_utf8(slices) {
                            Ok(data) => {
                                match serde_json::from_str::<serde_json::Value>(&data) {
                                    Ok(zjson) => {
                                        return Some(zjson);
                                    }
                                    Err(err) => {
                                        log::error!("Error while converting JSON data from ZENOH bus: {:?}", err);
                                    }
                                }
                            }
                            Err(err) => {
                                log::error!("Error while extracting data from ZENOH bus: {:?}", err);
                            }
                        }
                    }
                    Err(err) => {
                        log::error!("Error while getting data from ZENOH bus: {:?}", err);
                    }
                }
            }
        }
        Err(err) => {
            log::error!("Telemetry retival for key {} failed: {:?}", &key, err);
        }
    }
    None
}

pub fn zenoh_get_all(key: String, session: &Session) -> Option<Vec<serde_json::Value>> {
    let mut res: Vec<serde_json::Value> = Vec::new();
    match session.get(&key).res() {
        Ok(replies) => {
            while let Ok(reply) = replies.recv() {
                match reply.sample {
                    Ok(sample) => {
                        let slices = &sample.value.payload.contiguous();
                        match std::str::from_utf8(slices) {
                            Ok(data) => {
                                match serde_json::from_str::<serde_json::Value>(&data) {
                                    Ok(zjson) => {
                                        res.push(zjson);
                                    }
                                    Err(err) => {
                                        log::error!("Error while converting JSON data from ZENOH bus: {:?}", err);
                                        return None;
                                    }
                                }
                            }
                            Err(err) => {
                                log::error!("Error while extracting data from ZENOH bus: {:?}", err);
                                return None;
                            }
                        }
                    }
                    Err(err) => {
                        log::error!("Error while getting data from ZENOH bus: {:?}", err);
                        return None;
                    }
                }
            }
        }
        Err(err) => {
            log::error!("Telemetry retival for key {} failed: {:?}", &key, err);
            return None;
        }
    }
    return Some(res);
}
