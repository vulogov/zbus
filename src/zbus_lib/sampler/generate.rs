extern crate log;
use crate::zbus_lib::sampler::Sampler;
use rhai::{NativeCallContext, EvalAltResult};
use statrs::generate::{InfiniteSawtooth, InfinitePeriodic, InfiniteSinusoidal, InfiniteSquare, InfiniteTriangle};

pub fn create_st_dist(_context: NativeCallContext, p: i64, l: f64, u: f64, d: i64) -> Result<Sampler, Box<EvalAltResult>> {
    let mut res = Sampler::init();
    let x = InfiniteSawtooth::new(p, u, l, d);
    for v in x.take(128).collect::<Vec<f64>>() {
        res.try_set(v as f64);
    }
    Result::Ok(res)
}

pub fn create_periodic_dist(_context: NativeCallContext, rate: f64, freq: f64, a: f64, p: f64, d: i64) -> Result<Sampler, Box<EvalAltResult>> {
    let mut res = Sampler::init();
    let x = InfinitePeriodic::new(rate, freq, a, p, d);
    for v in x.take(128).collect::<Vec<f64>>() {
        res.try_set(v as f64);
    }
    Result::Ok(res)
}

pub fn create_sinus_dist(_context: NativeCallContext, rate: f64, freq: f64, a: f64, m: f64, p: f64, d: i64) -> Result<Sampler, Box<EvalAltResult>> {
    let mut res = Sampler::init();
    let x = InfiniteSinusoidal::new(rate, freq, a, m, p, d);
    for v in x.take(128).collect::<Vec<f64>>() {
        res.try_set(v as f64);
    }
    Result::Ok(res)
}

pub fn create_square_dist(_context: NativeCallContext, hd: i64, ld: i64, hv: f64, lv: f64, d: i64) -> Result<Sampler, Box<EvalAltResult>> {
    let mut res = Sampler::init();
    let x = InfiniteSquare::new(hd, ld, hv, lv, d);
    for v in x.take(128).collect::<Vec<f64>>() {
        res.try_set(v as f64);
    }
    Result::Ok(res)
}

pub fn create_triangle_dist(_context: NativeCallContext, rd: i64, fd: i64, hv: f64, lv: f64, d: i64) -> Result<Sampler, Box<EvalAltResult>> {
    let mut res = Sampler::init();
    let x = InfiniteTriangle::new(rd, fd, hv, lv, d);
    for v in x.take(128).collect::<Vec<f64>>() {
        res.try_set(v as f64);
    }
    Result::Ok(res)
}
