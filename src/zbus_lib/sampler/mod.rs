extern crate log;
use rhai::{Dynamic, Array, EvalAltResult};
use rhai::plugin::*;

use crate::zbus_lib::sampler::tsf::TSF;
use crate::zbus_lib::sampler::forecast_oscillator::FOSC;
use crate::zbus_lib::timestamp::timestamp_module::{timestamp_ns};
use crate::stdlib::traits::Indicator;
use lexical_core;
use std::collections::VecDeque;

mod eq;
mod metric;
mod zip;
mod construct;
mod smooth;
mod normalize;
mod generate;
mod harmonic;
mod distributions;
pub mod tsf;
pub mod forecast_oscillator;
pub mod markov;

#[derive(Debug, Clone)]
pub struct Sampler {
    d: VecDeque<f64>,
    s: VecDeque<f64>,
    tsf: TSF,
    fosc: FOSC,
    tsf_next: f64,
    fosc_next: f64,
}

impl Sampler {
    fn new() -> Self {
        Self {
            d: VecDeque::with_capacity(128),
            s: VecDeque::with_capacity(128),
            tsf: TSF::new(8),
            fosc: FOSC::new(8),
            tsf_next: 0.0 as f64,
            fosc_next: 0.0 as f64,
        }
    }
    pub fn init() -> Sampler {
        let mut res = Sampler::new();
        res.zero();
        res
    }
    pub fn init_no_ts() -> Sampler {
        let mut res = Sampler::new();
        res.zero_no_ts();
        res
    }
    fn zero(self: &mut Sampler) {
        for _ in 1..129 {
            self.try_set_no_ts(0.0 as f64);
            self.try_set_current_ts();
        }
    }
    fn zero_no_ts(self: &mut Sampler) {
        for _ in 1..129 {
            self.try_set_no_ts(0.0 as f64);
            self.try_set_ts(0.0);
        }
    }

    fn try_set_current_ts(self: &mut Sampler) {
        self.try_set_ts(timestamp_ns())
    }

    fn try_set_ts(self: &mut Sampler, ts: f64) {
        if self.s.len() == self.s.capacity() {
            let _ = self.s.pop_front();
        }
        let _ = self.s.push_back(ts);
    }

    pub fn try_set(self: &mut Sampler, v: f64) {
        self.try_set_no_ts(v);
        self.try_set_current_ts();
    }
    pub fn try_set_no_ts(self: &mut Sampler, v: f64) {
        if self.d.len() == self.d.capacity() {
            let _ = self.d.pop_front();
        }
        match self.tsf.next(v.clone()) {
            Some(next_val) => {
                self.tsf_next = next_val;
            }
            None => self.tsf_next = v.clone(),
        }
        match self.fosc.next(v.clone()) {
            Some(next_val) => {
                self.fosc_next = next_val;
            }
            None => self.fosc_next = v.clone(),
        }
        let _ = self.d.push_back(v);
    }
    pub fn set(self: &mut Sampler, v: Dynamic) -> Result<Dynamic, Box<EvalAltResult>> {
        if v.is_float() {
            self.try_set(v.clone_cast::<f64>());
            return Result::Ok(Dynamic::from(self.d.len() as i64));
        }
        if v.is_int() {
            self.try_set(v.clone_cast::<i64>() as f64);
            return Result::Ok(Dynamic::from(self.d.len() as i64));
        }
        if v.is_string() {
            match lexical_core::parse::<f64>(v.clone_cast::<String>().as_bytes()) {
                Ok(res) => {
                    self.try_set(res);
                }
                _ => {
                    return Err("Error parsing string value for Sampler".into());
                }
            }
            return Result::Ok(Dynamic::from(self.d.len() as i64));
        }
        Err("Value for the Sampler must be numeric".into())
    }
    pub fn set_and_ts_raw(self: &mut Sampler, v: Dynamic, ts: f64) -> Result<i64, Box<EvalAltResult>> {
        if v.is_float() {
            self.try_set_no_ts(v.clone_cast::<f64>());
            self.try_set_ts(ts);
            return Result::Ok(self.d.len() as i64);
        }
        if v.is_int() {
            self.try_set_no_ts(v.clone_cast::<i64>() as f64);
            self.try_set_ts(ts);
            return Result::Ok(self.d.len() as i64);
        }
        if v.is_string() {
            match lexical_core::parse::<f64>(v.clone_cast::<String>().as_bytes()) {
                Ok(res) => {
                    self.try_set_no_ts(res);
                    self.try_set_ts(ts);
                }
                _ => {
                    return Err("Error parsing string value for Sampler".into());
                }
            }
            return Result::Ok(self.d.len() as i64);
        }
        Err("Value for the Sampler must be numeric".into())
    }
    pub fn set_and_ts(self: &mut Sampler, v: Dynamic, ts: f64) -> Result<Dynamic, Box<EvalAltResult>> {
        match self.set_and_ts_raw(v, ts) {
            Ok(res) => return Result::Ok(Dynamic::from(res)),
            Err(err) => return Err(format!("{:?}", err).into()),
        }
    }
    fn tsf_next(self: &mut Sampler) -> Dynamic {
        Dynamic::from(self.tsf_next)
    }
    fn oscillator(self: &mut Sampler) -> Dynamic {
        Dynamic::from(self.fosc_next)
    }
    fn get(self: &mut Sampler) -> Dynamic {
        let mut res = Array::new();
        for v in &self.d {
            res.push(Dynamic::from(v.clone()));
        }
        Dynamic::from(res)
    }
    pub fn data(self: &mut Sampler) -> Dynamic {
        let mut res = Array::new();
        for i in 0..128 {
            match self.try_get_xy(i) {
                Ok((x,y)) => {
                    if x != 0.0 {
                        res.push(Dynamic::from(y));
                    }
                }
                Err(_) => {}
            }
        }
        Dynamic::from(res)
    }
    pub fn data_raw(self: &mut Sampler) -> Vec::<f64> {
        let mut res: Vec::<f64> = Vec::new();
        for i in 0..128 {
            match self.try_get_xy(i) {
                Ok((x,y)) => {
                    if x != 0.0 {
                        res.push(y);
                    }
                }
                Err(_) => {}
            }
        }
        res
    }
    pub fn try_get_xy(self: &mut Sampler, i: i64) -> Result<(f64, f64), Box<EvalAltResult>> {
        if (i < 0) || (i > 127) {
            return Err("Sampler.get() out of bound".into());
        }
        let x = self.s.get(i as usize).unwrap().clone();
        let y = self.d.get(i as usize).unwrap().clone();
        return Result::Ok((x,y));
    }
    fn get_xy(self: &mut Sampler, i: i64) -> Result<Dynamic, Box<EvalAltResult>> {
        match self.try_get_xy(i) {
            Ok((x,y)) => {
                let mut res = Array::new();
                res.push(Dynamic::from(x));
                res.push(Dynamic::from(y));
                return Result::Ok(Dynamic::from(res));
            }
            Err(err) => return Err(err),
        }
    }
    pub fn values(self: &mut Sampler) -> Result<Vec<Dynamic>, Box<EvalAltResult>> {
        let mut res: Array = Array::new();
        for i in 0..128 {
            match self.get_xy(i) {
                Ok(v) => {
                    res.push(v);
                }
                Err(_) => {}
            }
        }
        Ok(res)
    }
    pub fn values_raw(self: &mut Sampler) -> Vec<(f64,f64)> {
        let mut res: Vec<(f64,f64)> = Vec::new();
        for i in 0..128 {
            match self.try_get_xy(i) {
                Ok(v) => {
                    res.push(v);
                }
                Err(_) => {}
            }
        }
        res
    }
    pub fn raw(self: &mut Sampler) -> Vec<f64> {
        let mut res: Vec<f64> = Vec::new();
        for v in &self.d {
            res.push(v.clone());
        }
        res
    }
    fn try_downsample(self: &mut Sampler) -> VecDeque<f64> {
        let mut res: VecDeque<f64> = VecDeque::new();
        for i in (0..127).step_by(8) {
            let mut c: f64 = 0.0;
            for j in 0..8 {
                match self.d.get((i+j) as usize) {
                    Some(val) => c += val,
                    None => continue,
                }
            }
            let c = c / 8.0;
            res.push_back(c);
        }
        res
    }
    fn downsample(self: &mut Sampler) -> Dynamic {
        let ds_res = self.try_downsample();
        let mut res = Array::new();
        for v in &ds_res {
            res.push(Dynamic::from(v.clone()));
        }
        Dynamic::from(res)
    }
    fn car(self: &mut Sampler, n: i64) -> Dynamic {
        let mut res = Array::new();
        if n <= 0 {
            return Dynamic::from(res);
        }
        for v in 0..n as usize {
            match self.d.get(v as usize) {
                Some(val) => res.push(Dynamic::from(val.clone())),
                None => continue,
            }
        }
        Dynamic::from(res)
    }
    fn cdr(self: &mut Sampler, n: i64) -> Dynamic {
        let mut res = Array::new();
        if n >= 128 {
            return Dynamic::from(res);
        }
        for v in (n as usize)..129 {
            match self.d.get(v as usize) {
                Some(val) => res.push(Dynamic::from(val.clone())),
                None => continue,
            }
        }
        Dynamic::from(res)
    }
    fn distribute_timestamps(self: &mut Sampler, t: f64, d: i64) -> Sampler {
        let mut res = self.clone();
        let mut nt: f64 = t - ((d as f64 * 1000000000.0) * 128.0) as f64;
        for _ in 0..128 {
            res.try_set_ts(nt);
            nt = nt + (d as f64 * 1000000000.0);
        }
        res
    }
    fn distribute_from_current_timestamp(self: &mut Sampler, d: i64) -> Sampler {
        self.distribute_timestamps(timestamp_ns(), d)
    }
    pub fn min(self: &mut Sampler) -> f64 {
        self.data_raw().iter().copied().fold(f64::NAN, f64::min)
    }
    pub fn max(self: &mut Sampler) -> f64 {
        self.data_raw().iter().copied().fold(f64::NAN, f64::max)
    }
    pub fn data_len(self: &mut Sampler) -> i64 {
        let mut res: i64 = 0;
        for (t,_) in self.values_raw() {
            if t != 0.0 {
                res += 1;
            }
        }
        res.clone()
    }
}

#[export_module]
pub mod sampler_module {

}


pub fn init(engine: &mut Engine) {
    log::trace!("Running STDLIB::sampler init");

    engine.register_type::<Sampler>()
          .register_fn("Sampler", Sampler::init)
          .register_fn("EmptySampler", Sampler::init_no_ts)
          .register_fn("len", Sampler::data_len)
          .register_fn("set", Sampler::set)
          .register_fn("set", Sampler::set_and_ts)
          .register_fn("set", Sampler::set_and_ts_from_metric)
          .register_fn("raw", Sampler::raw)
          .register_fn("get", Sampler::get)
          .register_fn("data", Sampler::data)
          .register_fn("xy", Sampler::get_xy)
          .register_fn("values", Sampler::values)
          .register_fn("min", Sampler::min)
          .register_fn("max", Sampler::max)
          .register_fn("tsf_next", Sampler::tsf_next)
          .register_fn("oscillator", Sampler::oscillator)
          .register_fn("downsample", Sampler::downsample)
          .register_fn("smooth", Sampler::smooth)
          .register_fn("exp_smooth", Sampler::exp_smooth)
          .register_fn("normalize", Sampler::normalize)
          .register_fn("car", Sampler::car)
          .register_fn("cdr", Sampler::cdr)
          .register_fn("equal", Sampler::equal)
          .register_fn("equal", Sampler::try_equal)
          .register_fn("harmonic", Sampler::harmonic)
          .register_fn("markov", Sampler::markov)
          .register_fn("distribute_timestamps", Sampler::distribute_timestamps)
          .register_fn("distribute_timestamps", Sampler::distribute_from_current_timestamp)
          .register_fn("to_string", |x: &mut Sampler| format!("{:?}", x.d) );

    let mut module = exported_module!(sampler_module);
    module.set_native_fn("make_normal", distributions::create_normal_normalized_dist);
    module.set_native_fn("zip", zip::sampler_zip);
    module.set_native_fn("Sampler", construct::sampler_construct);
    module.set_native_fn("Normal", distributions::create_normal_dist);
    module.set_native_fn("Uniform_normalized", distributions::create_uniform_normalized_dist);
    module.set_native_fn("Uniform", distributions::create_uniform_dist);
    module.set_native_fn("Exponential", distributions::create_exponential_dist);
    module.set_native_fn("Binomial", distributions::create_binomial_dist);
    module.set_native_fn("Log", distributions::create_log_dist);
    module.set_native_fn("Sawtooth", generate::create_st_dist);
    module.set_native_fn("Periodic", generate::create_periodic_dist);
    module.set_native_fn("Sinusoidal", generate::create_sinus_dist);
    module.set_native_fn("Square", generate::create_square_dist);
    module.set_native_fn("Triangle", generate::create_triangle_dist);
    engine.register_static_module("sampler", module.into());


}
