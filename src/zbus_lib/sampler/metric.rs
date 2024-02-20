extern crate log;
use rhai::{EvalAltResult};
use rhai::plugin::*;
use crate::zbus_lib::sampler::Sampler;
use crate::zbus_lib::metric::Metric;


impl Sampler {
    pub fn set_and_ts_from_metric(&mut self, mut data: Metric) -> Result<Dynamic, Box<EvalAltResult>> {
        match data.get_value() {
            Ok(value) => {
                return self.set_and_ts(value, data.timestamp);
            }
            Err(err) => {
                return Err(format!("Error setting Sampler() from Metric(): {:?}", err).into());
            }
        }
    }
}
