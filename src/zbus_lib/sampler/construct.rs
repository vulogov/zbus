extern crate log;
use crate::zbus_lib::sampler::Sampler;
use rhai::{Array, NativeCallContext, EvalAltResult};

pub fn sampler_construct(_context: NativeCallContext, t: Array) -> Result<Sampler, Box<EvalAltResult>> {
    let mut res = Sampler::init();
    for v in t {
        if v.is_float() {
            res.try_set(v.clone_cast::<f64>());
        }
    }
    return Result::Ok(res);
}
