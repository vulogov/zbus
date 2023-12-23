extern crate log;
use  statrs::prec::almost_eq;
use crate::zbus_lib::sampler::Sampler;


impl Sampler {
    pub fn try_equal(&mut self, other: Sampler, acc: f64) -> bool {
        for i in 0..128 {
            match self.d.get(i) {
                Some(a) => {
                    match other.d.get(i) {
                        Some(b) => {
                            if acc == 0.0 {
                                if a != b {
                                    return false;
                                }
                            } else {
                                let res = almost_eq(*a, *b, acc);
                                if ! res {
                                    return false;
                                }
                            }
                        }
                        _ => return false,
                    }
                }
                _ => return false,
            }
        }
        true
    }
    pub fn equal(&mut self, other: Sampler) -> bool {
        self.try_equal(other, 0.0)
    }
}
