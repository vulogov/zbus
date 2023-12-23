extern crate log;
use crate::zbus_lib::sampler::Sampler;

use ta::indicators::{SimpleMovingAverage, ExponentialMovingAverage};
use ta::Next;

impl Sampler {
    pub fn smooth(&mut self) -> Sampler {
        let mut res = Sampler::init();
        let mut sma = SimpleMovingAverage::new(3).unwrap();
        for i in 0..128 {
            res.try_set(sma.next(*self.d.get(i).unwrap()));
        }
        res
    }
    pub fn exp_smooth(&mut self) -> Sampler {
        let mut res = Sampler::init();
        let mut sma = ExponentialMovingAverage::new(3).unwrap();
        for i in 0..128 {
            res.try_set(sma.next(*self.d.get(i).unwrap()));
        }
        res
    }
}
