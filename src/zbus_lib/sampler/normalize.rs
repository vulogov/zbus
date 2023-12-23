extern crate log;
use crate::zbus_lib::sampler::Sampler;

impl Sampler {
    pub fn normalize(&mut self) -> Sampler {
        let mut res = Sampler::init();
        let mut y: Vec<f64> = Vec::new();
        for i in 0..128 {
            y.push(*self.d.get(i).unwrap());
        }
        let min_y = y.iter().cloned().fold(0./0., f64::min);
        let max_y = y.iter().cloned().fold(0./0., f64::max);
        if (max_y - min_y) == 0.0 {
            return res;
        }
        for v in y {
            res.try_set(((v-min_y)/(max_y-min_y)) as f64);
        }
        res
    }
}
