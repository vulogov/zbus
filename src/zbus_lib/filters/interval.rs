extern crate log;
use rhai::{Engine, Module, NativeCallContext, EvalAltResult};

use crate::zbus_lib::sampler;
use iset::set::IntervalSet;
use iset::IntervalMap;

#[derive(Debug, Clone)]
pub struct SampleInterval {
    m:  IntervalMap<f64, String>,
}

impl SampleInterval {
    fn new() -> Self {
        Self {
            m: IntervalMap::new(),
        }
    }
    fn interval(self: &mut SampleInterval, lower: f64, upper: f64, label: String) {
        if self.m.contains(lower..upper) {
            return;
        }
        let _ = self.m.insert(lower..upper, label);
    }
    fn check(self: &mut SampleInterval, value: f64) -> Result<String, Box<EvalAltResult>> {
        for (_, v) in self.m.overlap(value) {
            return Ok(v.to_string());
        }
        return Err(format!("Interval key error: {}", &value).into());
    }
    fn len(self: &mut SampleInterval) -> i64 {
        self.m.len() as i64
    }
}

pub fn interval_fit_function(_context: NativeCallContext, mut data: sampler::Sampler, test_value: f64, width: f64) -> Result<bool, Box<EvalAltResult>>{
    let mut rs: IntervalSet::<f64> = IntervalSet::new();
    for v in data.data_raw() {
        rs.insert(v-width..v+width);
    }
    Ok(rs.has_overlap(test_value-width..test_value+width))
}

pub fn init(engine: &mut Engine, fm: &mut Module) {
    log::trace!("Running STDLIB::filters::interval init");
    engine.register_type::<SampleInterval>()
          .register_fn("SampleInterval", SampleInterval::new)
          .register_fn("interval", SampleInterval::interval)
          .register_fn("check", SampleInterval::check)
          .register_fn("len", SampleInterval::len)
          .register_fn("to_string", |x: &mut SampleInterval| format!("SampleInterval len={:?}", x.m.len()) );

    let mut interval_module = Module::new();
    interval_module.set_native_fn("interval_fit", interval_fit_function);
    fm.set_sub_module("interval", interval_module);
}
