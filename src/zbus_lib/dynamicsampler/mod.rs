extern crate log;
use rhai::{Engine, EvalAltResult, Dynamic, Array};
use rhai::plugin::*;
use std::collections::VecDeque;

use crate::zbus_lib::{timestamp, metric, string};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DynamicSampler {
    pub t:          timestamp::TimeInterval,
    pub k:          String,
    pub s:          i64,
    d:              VecDeque<metric::Metric>,
}

impl DynamicSampler {
    fn new() -> Self {
        Self {
            t: timestamp::TimeInterval::new(),
            k: "".to_string(),
            s: 128 as i64,
            d: VecDeque::new(),
        }
    }
    fn data(self: &mut DynamicSampler) -> Result<Array, Box<EvalAltResult>> {
        let mut res = Array::new();
        for m in self.d.iter() {
            match m.clone().get_value() {
                Ok(data) => {
                    res.push(data.clone());
                }
                Err(err) => return Err(format!("{:?}", err).into()),
            }
        }
        Ok(res)
    }
    fn values(self: &mut DynamicSampler) -> Result<Array, Box<EvalAltResult>> {
        let mut res = Array::new();
        for m in self.d.iter() {
            match m.clone().get_value() {
                Ok(data) => {
                    let mut row = Array::new();
                    row.push(Dynamic::from(m.timestamp));
                    row.push(data.clone());
                    res.push(row.into());
                }
                Err(err) => return Err(format!("{:?}", err).into()),
            }
        }
        Ok(res)
    }
    fn len(self: &mut DynamicSampler) -> i64 {
        self.d.len() as i64
    }
    fn set(self: &mut DynamicSampler, m: metric::Metric) -> bool {
        let key = format!("{}", m.timestamp);
        if self.k.len() == 0 {
            self.k = key.clone();
        }
        if self.k != key {
            return false;
        }
        if self.is_timestamp(m.clone()) {
            return false;
        }
        let stamp = timestamp::timestamp_module::whole_seconds(m.timestamp) as i64;
        if stamp > self.t.e {
            self.t.e = stamp;
        } else if stamp < self.t.s {
            self.t.s = stamp;
        }
        self.d.push_back(m.clone());
        if self.d.len() as i64 > self.s {
            let _ = self.d.pop_front();
        }
        true
    }
    fn is_timestamp(self: &mut DynamicSampler, m: metric::Metric) -> bool {
        for n in self.d.iter() {
            if n.timestamp == m.timestamp {
                return true;
            }
        }
        false
    }
    fn set_unique(self: &mut DynamicSampler, q: i64, m: metric::Metric) -> Result<bool, Box<EvalAltResult>> {
        let value = match m.clone().get_value() {
            Ok(data) => data,
            Err(_) => {
                return Err(format!("Can not get value for DynamicSampler().set_unique()").into());
            }
        };
        if value.is_string() {
            let s_value = value.into_string();
            let mut curr_q = 0;
            for n in self.d.iter() {
                let value2 = match n.clone().get_value() {
                    Ok(data) => data,
                    Err(_) => {
                        return Err(format!("Can not get value for DynamicSampler().set_unique()").into());
                    }
                };
                let c = string::fuzzy::str_match_levenshtein_raw(s_value.clone().unwrap(), value2.into_string().unwrap());
                if c > curr_q {
                    curr_q = c;
                }
            }
            if curr_q < q {
                return Ok(self.set(m.clone()));
            }
        } else {
            return Err(format!("Datatype stored in Metric() is not suitable for DynamicSampler().set_unique()").into());
        }
        Ok(true)
    }
}



#[export_module]
pub mod dynamicsampler_module {
}

pub fn init(engine: &mut Engine) {
    log::trace!("Running STDLIB::interval init");
    engine.register_type::<DynamicSampler>()
          .register_fn("DynamicSampler", DynamicSampler::new)
          .register_fn("data", DynamicSampler::data)
          .register_fn("values", DynamicSampler::values)
          .register_fn("len", DynamicSampler::len)
          .register_fn("set", DynamicSampler::set)
          .register_fn("set_unique", DynamicSampler::set_unique)
          .register_fn("is_timestamp", DynamicSampler::is_timestamp)
          .register_fn("to_string", |x: &mut DynamicSampler| format!("DynamicSampler().len() = {}", x.len()));

    let module = exported_module!(dynamicsampler_module);
    engine.register_static_module("dynamicsampler", module.into());
}
