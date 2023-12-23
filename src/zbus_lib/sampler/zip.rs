extern crate log;
use crate::zbus_lib::sampler::Sampler;
use rhai::{Dynamic, Array, FnPtr, NativeCallContext, EvalAltResult};

pub fn sampler_zip(context: NativeCallContext, t: &mut Sampler, f: FnPtr) -> Result<Vec<rhai::Dynamic>, Box<EvalAltResult>> {
    let mut res = Array::new();
    for i in 0..128 {
        match t.try_get_xy(i) {
            Ok((x, y)) => {
                let r: Result<Dynamic, Box<EvalAltResult>> = f.call_within_context(&context, (x,y,));
                match r {
                    Ok(val) => res.push(val),
                    Err(_) => continue,
                }
            }
            Err(err) => return Err(err),
        }
    }
    return Result::Ok(res);
}
