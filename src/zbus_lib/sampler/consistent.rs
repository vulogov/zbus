extern crate log;
use crate::zbus_lib::sampler::Sampler;
use rhai::{EvalAltResult};

impl Sampler {
    pub fn n_get(&mut self) -> i64 {
        self.n
    }

    pub fn n_set(&mut self, v: i64) -> Self {
        self.n = v;
        self
    }

    pub fn q_set(&mut self, v: f64) -> Self {
        self.q = v;
        self
    }

}
