extern crate log;
use crate::zbus_lib::sampler::Sampler;
use rhai::{Dynamic, Array};
use decorum::{R64};
use markov_chain::Chain;

impl Sampler {
    pub fn markov(&mut self) -> Dynamic {
        let source = self.data_raw();
        let mut dst: Vec<R64> = Vec::new();
        for v in source {
            dst.push(v.into());
        }
        let mut palanteer = Chain::<R64>::new(16);
        palanteer.train(dst);
        let res = palanteer.generate_limit(16);
        let mut out = Array::new();
        for i in res {
            out.push(Dynamic::from(f64::from(i)));
        }
        Dynamic::from(out)
    }
}
