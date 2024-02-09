extern crate log;
use lazy_static::lazy_static;
use std::sync::Mutex;
use rhai::{Dynamic, Map, NativeCallContext, EvalAltResult};
use serde_json::{to_string, from_str};
use std::collections::btree_map::BTreeMap;
use crossbeam::channel::{Sender, Receiver, unbounded};

lazy_static! {
    static ref PIPES: Mutex<BTreeMap<String,(Sender<String>, Receiver<String>)>> = {
        let e: Mutex<BTreeMap<String,(Sender<String>, Receiver<String>)>> = Mutex::new(BTreeMap::new());
        e
    };
}

pub fn pipes_init() {
    log::debug!("Initializing default pipes");
    let mut q = PIPES.lock().unwrap();
    q.insert("in".to_string(), unbounded::<String>());
    q.insert("out".to_string(), unbounded::<String>());
    drop(q);
}

pub fn create_pipe(n: String) {
    log::debug!("Create pipe: {}", &n);
    let mut q = PIPES.lock().unwrap();
    q.insert(n.to_string(), unbounded::<String>());
    drop(q);
}

pub fn pipe_is_empty(_context: NativeCallContext, k: String) -> Result<bool, Box<EvalAltResult>> {
    let mut q = PIPES.lock().unwrap();
    if ! q.contains_key(&k) {
        drop(q);
        return Err(format!("bus no pipe: {}", &k).into());
    }
    let (_, r) = q.get_mut(&k).unwrap();
    if r.is_empty() {
        drop(q);
        return Result::Ok(true);
    }
    drop(q);
    Result::Ok(false)
}

pub fn pipe_push_raw(k: String, d: String) {
    let mut q = PIPES.lock().unwrap();
    if ! q.contains_key(&k) {
        log::trace!("new bus::internal::pipe : {}", &k);
        let (s,r) = unbounded::<String>();
        match s.send(d) {
            Ok(_) => {
                q.insert(k, (s,r));
            }
            Err(_) => {
                drop(q);
            }
        };
    } else {
        let (s,_) = q.get_mut(&k).unwrap();
        match s.send(d) {
            Ok(_) => {},
            Err(_) => {
                drop(q);
            }
        }
    }
}
pub fn pipe_push(_context: NativeCallContext, k: String, d: Dynamic) -> Result<bool, Box<EvalAltResult>> {
    try_pipe_push(k,d)
}

pub fn pipe_push_map(_context: NativeCallContext, k: String, d: Map) -> Result<bool, Box<EvalAltResult>> {
    try_pipe_push(k,Dynamic::from(d))
}

pub fn try_pipe_push(k: String, d: Dynamic) -> Result<bool, Box<EvalAltResult>> {
    match to_string(&d) {
        Ok(res) => {
            let mut q = PIPES.lock().unwrap();
            if ! q.contains_key(&k) {
                log::trace!("new bus::internal::pipe : {}", &k);
                let (s,r) = unbounded::<String>();
                match s.send(res) {
                    Ok(_) => q.insert(k, (s,r)),
                    Err(err) => {
                        drop(q);
                        return Err(format!("bus::internal::pipe error: {}", err).into());
                    }
                };
            } else {
                let (s,_) = q.get_mut(&k).unwrap();
                match s.send(res) {
                    Ok(_) => {},
                    Err(err) => {
                        drop(q);
                        return Err(format!("bus::internal::pipe error: {}", err).into());
                    }
                }
            }
            drop(q);
        }
        Err(err) => {
            let msg = format!("Error converting to JSON: {}", err);
            log::error!("{}", &msg);
            return Err(msg.into())
        }
    }
    Result::Ok(true)
}

pub fn pipe_pull(_context: NativeCallContext, k: String) -> Result<Dynamic, Box<EvalAltResult>> {
    let mut q = PIPES.lock().unwrap();
    if ! q.contains_key(&k) {
        drop(q);
        return Err(format!("bus::internal::pipe no pipe: {}", &k).into());
    }
    let (_, r) = q.get_mut(&k).unwrap();
    if r.is_empty() {
        return Err(format!("bus::internal::pipe is empty: {}", &k).into());
    }
    match r.recv() {
        Ok(res) => {
            match from_str::<Dynamic>(&res) {
                Ok(val) => {
                    drop(q);
                    return Result::Ok(val);
                }
                Err(err) => {
                    let msg = format!("Error converting from JSON: {}", err);
                    log::error!("{}", &msg);
                    return Err(msg.into());
                }
            }
        }
        Err(err) => Err(format!("bus::internal::pipe {} can not recv: {}", &k, &err).into()),
    }
}
