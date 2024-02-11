extern crate log;
use rhai::{Engine};
use rhai::plugin::*;

use std::ops::{Add, Sub, Mul, Div};
use inari::{Interval as I, interval};
use crate::zbus_lib::sampler;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Interval {
    pub i: I,
}

impl Interval {
    fn new(x: f64, y: f64) -> Self {
        Self {
            i: interval!(x, y).unwrap(),
        }
    }
    fn from_interval(i: I) -> Self {
        Self {
            i: i,
        }
    }
    fn from_sampler(mut s: sampler::Sampler) -> Self {
        Self {
            i: interval!(s.min(), s.max()).unwrap(),
        }
    }
    fn width(self: &mut Interval) -> f64 {
        self.i.wid()
    }
    fn midpoint(self: &mut Interval) -> f64 {
        self.i.mid()
    }
    fn upper(self: &mut Interval) -> f64 {
        self.i.sup()
    }
    fn lower(self: &mut Interval) -> f64 {
        self.i.inf()
    }
    fn contains_in(self: &mut Interval, n: f64) -> bool {
        self.i.contains(n)
    }
    fn less(self: &mut Interval, other: Interval) -> bool {
        self.i.less(other.i)
    }
    fn more(self: &mut Interval, other: Interval) -> bool {
        ! self.i.less(other.i)
    }
    fn eq(self: &mut Interval, other: Interval) -> bool {
        self.i.eq(&other.i)
    }
    fn interior(self: &mut Interval, other: Interval) -> bool {
        self.i.interior(other.i)
    }
    fn disjoint(self: &mut Interval, other: Interval) -> bool {
        self.i.disjoint(other.i)
    }
    fn interval_intersection(self: &mut Interval, other: Interval) -> Interval {
        Interval::from_interval(self.i.intersection(other.i))
    }
    fn interval_add(self: &mut Interval, other: Interval) -> Interval {
        Interval::from_interval(self.i.add(other.i))
    }
    fn interval_sub(self: &mut Interval, other: Interval) -> Interval {
        Interval::from_interval(self.i.sub(other.i))
    }
    fn interval_mul(self: &mut Interval, other: Interval) -> Interval {
        Interval::from_interval(self.i.mul(other.i))
    }
    fn interval_div(self: &mut Interval, other: Interval) -> Interval {
        Interval::from_interval(self.i.div(other.i))
    }
}



#[export_module]
pub mod interval_module {
}

pub fn init(engine: &mut Engine) {
    log::trace!("Running STDLIB::interval init");
    engine.register_type::<Interval>()
          .register_fn("Interval", Interval::new)
          .register_fn("Interval", Interval::from_sampler)
          .register_fn("width", Interval::width)
          .register_fn("upper", Interval::upper)
          .register_fn("midpoint", Interval::midpoint)
          .register_fn("contains", Interval::contains_in)
          .register_fn("lower", Interval::lower)
          .register_fn("less", Interval::less)
          .register_fn("more", Interval::more)
          .register_fn("eq", Interval::eq)
          .register_fn("interior", Interval::interior)
          .register_fn("disjoint", Interval::disjoint)
          .register_fn("intersection", Interval::interval_intersection)
          .register_fn("add", Interval::interval_add)
          .register_fn("sub", Interval::interval_sub)
          .register_fn("mul", Interval::interval_mul)
          .register_fn("div", Interval::interval_div)
          .register_fn("to_string", |x: &mut Interval| format!("Interval({}:{})", x.lower(), x.upper()) );

    let module = exported_module!(interval_module);
    engine.register_static_module("interval", module.into());
}
