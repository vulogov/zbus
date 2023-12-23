extern crate log;
use crate::zbus_lib::sampler::Sampler;
use statrs::function::harmonic::gen_harmonic;

impl Sampler {
    pub fn harmonic(&mut self, n: i64) -> Sampler {
        let mut res = Sampler::init();
        for i in 0..128 {
            res.try_set(gen_harmonic(n as u64, *self.d.get(i).unwrap()));
        }
        res
    }
}
