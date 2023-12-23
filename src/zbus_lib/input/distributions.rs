extern crate log;

use rhai::{Dynamic, FnPtr, NativeCallContext, EvalAltResult};
use rand::distributions::{Distribution, Uniform};
use statrs::distribution::{Normal, Binomial, Exp, LogNormal};
use statrs::generate::{InfiniteSawtooth, InfinitePeriodic, InfiniteSinusoidal, InfiniteSquare, InfiniteTriangle};
use crate::zbus_lib::system::system_module::{sleep_millisecond};

pub fn norm_distribution_gen(context: NativeCallContext, m: f64, dev: f64, f: FnPtr, delay: i64) -> Result<(), Box<EvalAltResult>>{

    let mut r = rand::thread_rng();
    let n = Normal::new(m, dev).unwrap();
    loop {
        let val = (n.sample(&mut r) as f64).abs();
        let r: Result<(), Box<EvalAltResult>> = f.call_within_context(&context, (Dynamic::from_float(val),));
        match r {
            Ok(_) => sleep_millisecond(delay),
            Err(err) => {
                log::debug!("distribution generator cb returned: {}", err);
                break;
            }
        }
    }
    Result::Ok(())
}

pub fn exp_distribution_gen(context: NativeCallContext, rate: f64, f: FnPtr, delay: i64) -> Result<(), Box<EvalAltResult>>{

    let mut r = rand::thread_rng();
    let n = Exp::new(rate).unwrap();
    loop {
        let val = n.sample(&mut r) as f64;
        let r: Result<(), Box<EvalAltResult>> = f.call_within_context(&context, (Dynamic::from_float(val),));
        match r {
            Ok(_) => sleep_millisecond(delay),
            Err(err) => {
                log::debug!("distribution generator cb returned: {}", err);
                break;
            }
        }
    }
    Result::Ok(())
}

pub fn binomial_distribution_gen(context: NativeCallContext, p: f64, n: i64, f: FnPtr, delay: i64) -> Result<(), Box<EvalAltResult>>{

    let mut r = rand::thread_rng();
    let n = Binomial::new(p, n as u64).unwrap();
    loop {
        let val = n.sample(&mut r) as f64;
        let r: Result<(), Box<EvalAltResult>> = f.call_within_context(&context, (Dynamic::from_float(val),));
        match r {
            Ok(_) => sleep_millisecond(delay),
            Err(err) => {
                log::debug!("distribution generator cb returned: {}", err);
                break;
            }
        }
    }
    Result::Ok(())
}

pub fn lognormal_distribution_gen(context: NativeCallContext, l: f64, s: f64, f: FnPtr, delay: i64) -> Result<(), Box<EvalAltResult>>{

    let mut r = rand::thread_rng();
    let n = LogNormal::new(l,s).unwrap();
    loop {
        let val = n.sample(&mut r) as f64;
        let r: Result<(), Box<EvalAltResult>> = f.call_within_context(&context, (Dynamic::from_float(val),));
        match r {
            Ok(_) => sleep_millisecond(delay),
            Err(err) => {
                log::debug!("distribution generator cb returned: {}", err);
                break;
            }
        }
    }
    Result::Ok(())
}

pub fn uniform_distribution_gen(context: NativeCallContext, l: f64, u: f64, f: FnPtr, delay: i64) -> Result<(), Box<EvalAltResult>>{

    let mut r = rand::thread_rng();
    let n = Uniform::new::<f64, f64>(l,u);
    loop {
        let val = n.sample(&mut r) as f64;
        let r: Result<(), Box<EvalAltResult>> = f.call_within_context(&context, (Dynamic::from_float(val),));
        match r {
            Ok(_) => sleep_millisecond(delay),
            Err(err) => {
                log::debug!("distribution generator cb returned: {}", err);
                break;
            }
        }
    }
    Result::Ok(())
}

pub fn sawtooth_gen(context: NativeCallContext, p: i64, l: f64, h: f64, d: i64, f: FnPtr, delay: i64) -> Result<(), Box<EvalAltResult>>{

    let mut n = InfiniteSawtooth::new(p, h, l, d);
    loop {
        match n.next() {
            Some(val) => {
                let r: Result<(), Box<EvalAltResult>> = f.call_within_context(&context, (Dynamic::from_float(val),));
                match r {
                    Ok(_) => sleep_millisecond(delay),
                    Err(err) => {
                        log::debug!("distribution generator cb returned: {}", err);
                        break;
                    }
                }
            }
            _ => break,
        }
    }
    Result::Ok(())
}

pub fn periodic_gen(context: NativeCallContext, rate: f64, freq: f64, a: f64, p: f64, d: i64, f: FnPtr, delay: i64) -> Result<(), Box<EvalAltResult>>{

    let mut n = InfinitePeriodic::new(rate, freq, a, p, d);
    loop {
        match n.next() {
            Some(val) => {
                let r: Result<(), Box<EvalAltResult>> = f.call_within_context(&context, (Dynamic::from_float(val),));
                match r {
                    Ok(_) => sleep_millisecond(delay),
                    Err(err) => {
                        log::debug!("distribution generator cb returned: {}", err);
                        break;
                    }
                }
            }
            _ => break,
        }
    }
    Result::Ok(())
}

pub fn sinusoidal_gen(context: NativeCallContext, rate: f64, freq: f64, a: f64, m: f64, p: f64, d: i64, f: FnPtr, delay: i64) -> Result<(), Box<EvalAltResult>>{

    let mut n = InfiniteSinusoidal::new(rate, freq, a, m, p, d);
    loop {
        match n.next() {
            Some(val) => {
                let r: Result<(), Box<EvalAltResult>> = f.call_within_context(&context, (Dynamic::from_float(val),));
                match r {
                    Ok(_) => sleep_millisecond(delay),
                    Err(err) => {
                        log::debug!("distribution generator cb returned: {}", err);
                        break;
                    }
                }
            }
            _ => break,
        }
    }
    Result::Ok(())
}

pub fn square_gen(context: NativeCallContext, hd: i64, ld: i64, hv: f64, lv: f64, d: i64, f: FnPtr, delay: i64) -> Result<(), Box<EvalAltResult>>{

    let mut n = InfiniteSquare::new(hd, ld, hv, lv, d);
    loop {
        match n.next() {
            Some(val) => {
                let r: Result<(), Box<EvalAltResult>> = f.call_within_context(&context, (Dynamic::from_float(val),));
                match r {
                    Ok(_) => sleep_millisecond(delay),
                    Err(err) => {
                        log::debug!("distribution generator cb returned: {}", err);
                        break;
                    }
                }
            }
            _ => break,
        }
    }
    Result::Ok(())
}

pub fn triangle_gen(context: NativeCallContext, rd: i64, fd: i64, lv: f64, hv: f64, d: i64, f: FnPtr, delay: i64) -> Result<(), Box<EvalAltResult>>{

    let mut n = InfiniteTriangle::new(rd, fd, hv, lv, d);
    loop {
        match n.next() {
            Some(val) => {
                let r: Result<(), Box<EvalAltResult>> = f.call_within_context(&context, (Dynamic::from_float(val),));
                match r {
                    Ok(_) => sleep_millisecond(delay),
                    Err(err) => {
                        log::debug!("distribution generator cb returned: {}", err);
                        break;
                    }
                }
            }
            _ => break,
        }
    }
    Result::Ok(())
}
