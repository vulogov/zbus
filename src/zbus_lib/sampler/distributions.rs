extern crate log;
use crate::zbus_lib::sampler::Sampler;
use rhai::{NativeCallContext, EvalAltResult};
use rand::distributions::{Distribution, Uniform};
use statrs::distribution::{Normal, Binomial, Exp, LogNormal};

pub fn create_normal_dist(_context: NativeCallContext, m: f64, dev: f64) -> Result<Sampler, Box<EvalAltResult>> {
    let mut res = Sampler::init();
    let mut r = rand::thread_rng();
    let n = Normal::new(m, dev).unwrap();
    for _v in 1..129 {
        res.try_set((n.sample(&mut r) as f64).abs());
    }
    Result::Ok(res)
}

pub fn create_binomial_dist(_context: NativeCallContext, p: f64, n: i64) -> Result<Sampler, Box<EvalAltResult>> {
    let mut res = Sampler::init();
    let mut r = rand::thread_rng();
    let n = Binomial::new(p, n as u64).unwrap();
    for _v in 1..129 {
        res.try_set((n.sample(&mut r) as f64).abs());
    }
    Result::Ok(res)
}

pub fn create_exponential_dist(_context: NativeCallContext, rate: f64) -> Result<Sampler, Box<EvalAltResult>> {
    let mut res = Sampler::init();
    let mut r = rand::thread_rng();
    let n = Exp::new(rate).unwrap();
    for _v in 1..129 {
        res.try_set((n.sample(&mut r) as f64).abs());
    }
    Result::Ok(res)
}

pub fn create_log_dist(_context: NativeCallContext, l: f64, s: f64) -> Result<Sampler, Box<EvalAltResult>> {
    let mut res = Sampler::init();
    let mut r = rand::thread_rng();
    let n = LogNormal::new(l,s).unwrap();
    for _v in 1..129 {
        res.try_set((n.sample(&mut r) as f64).abs());
    }
    Result::Ok(res)
}

pub fn create_uniform_dist(_context: NativeCallContext, l: f64, u: f64) -> Result<Sampler, Box<EvalAltResult>> {
    let mut r = rand::thread_rng();
    let n = Uniform::new::<f64, f64>(l,u);

    let mut res = Sampler::init();
    for _v in 1..129 {
        res.try_set(n.sample(&mut r) as f64);
    }
    Result::Ok(res)

}

pub fn create_uniform_normalized_dist(context: NativeCallContext) -> Result<Sampler, Box<EvalAltResult>> {
    create_uniform_dist(context, 0.0, 1.0)
}

pub fn create_normal_normalized_dist(context: NativeCallContext) -> Result<Sampler, Box<EvalAltResult>> {
    create_normal_dist(context, 0.5, 0.01)
}
