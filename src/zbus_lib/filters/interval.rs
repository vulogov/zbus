extern crate log;
use rhai::{Engine, Module, NativeCallContext, EvalAltResult};

use crate::zbus_lib::sampler;
use iset::set::IntervalSet;

pub fn interval_fit_function(_context: NativeCallContext, mut data: sampler::Sampler, test_value: f64, width: f64) -> Result<bool, Box<EvalAltResult>>{
    let mut rs: IntervalSet::<f64> = IntervalSet::new();
    for v in data.data_raw() {
        rs.insert(v-width..v+width);
    }
    Ok(rs.has_overlap(test_value-width..test_value+width))
}

pub fn init(_engine: &mut Engine, fm: &mut Module) {
    log::trace!("Running STDLIB::filters::interval init");
    let mut interval_module = Module::new();
    interval_module.set_native_fn("interval_fit", interval_fit_function);
    fm.set_sub_module("interval", interval_module);
}
