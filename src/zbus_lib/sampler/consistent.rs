extern crate log;
use crate::zbus_lib::sampler::Sampler;
use statrs::statistics::Statistics;


impl Sampler {
    pub fn n_get(&mut self) -> i64 {
        self.n
    }

    pub fn n_set(&mut self, v: i64) {
        self.n = v;
    }

    pub fn q_set(&mut self, v: f64) {
        self.q = v;
    }

    pub fn q_get(&mut self) -> f64 {
        self.q
    }

    pub fn set_consistent(&mut self, v: f64) -> bool {
        self.try_set(v);
        if (self.data_len() as i64 + 1) >= self.n {
            let vals = self.data_raw();
            let s_dev = vals.std_dev();
            if s_dev.is_nan() {
                return false;
            }
            if s_dev <= self.q_get() {
                return true;
            }
        }
        false
    }

    pub fn consistency(&mut self) -> f64 {
        let vals = self.data_raw();
        let s_dev = vals.std_dev();
        return s_dev;
    }

}
