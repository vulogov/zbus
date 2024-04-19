extern crate log;
use crate::zbus_lib::sampler::Sampler;
use statrs::statistics::Statistics;


impl Sampler {

    pub fn set_inconsistent(&mut self, v: f64) -> bool {

        if (self.data_len() as i64 + 1) >= self.n {
            let mut vals = self.data_raw();
            vals.push(v);
            let s_dev = vals.std_dev();
            if s_dev.is_nan() {
                return false;
            }
            if s_dev > self.q_get() {
                self.try_set(v);
                return true;
            }
        } else {
            self.try_set(v);
        }
        false
    }

}
