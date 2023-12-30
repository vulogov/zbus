extern crate log;
use std::vec::Vec;
use maver::*;
use rhai::{Engine, Array};
use rhai::plugin::*;

fn array2vec(s: Array) -> Vec<f64> {
    let mut res: Vec<f64> = Vec::new();
    for v in s {
        match v.as_float() {
            Ok(val) => res.push(val as f64),
            Err(err) => {
                log::error!("Error in perceptron training: {}", err);
                break;
            }
        }
    }
    res
}

fn vec2array(s: Vec<f64>) -> Array {
    let mut res = Array::new();
    for n in s {
        res.push(Dynamic::from(n.clone()));
    }
    res
}

#[derive(Debug, Clone)]
pub struct NRNeuralNetwork {
    nn:     NeuralNetwork,
    train:  Vec<(Vec<f64>, Vec<f64>)>,
    epoch:  usize,
    shuffle: bool,
    rate:    f64,
}

impl NRNeuralNetwork {
    fn new() -> Self {
        Self {
            nn:     NeuralNetwork::new(vec![2,2,1],Activation::Tanh,Activation::Tanh),
            train:  Vec::new(),
            epoch:  1000 as usize,
            shuffle: false,
            rate:    0.01 as f64,
        }
    }
    pub fn init(i: i64, o: i64, h: i64) -> NRNeuralNetwork {
        let mut res = NRNeuralNetwork::new();
        res.nn = NeuralNetwork::new(vec![i as usize,o as usize,h as usize],Activation::Tanh,Activation::Tanh);
        res
    }
    fn add(&mut self, d: Array, c: Array) {
        let data = array2vec(d);
        let cls  = array2vec(c);
        self.train.push((data, cls));
    }
    fn train(&mut self) {
        self.nn.train(self.epoch, self.shuffle, self.train.clone(), Some(self.rate));
    }
    fn forward(&mut self, d: Array) -> Array {
        let data: Vec<f64> = array2vec(d);
        let res: Vec<f64> = self.nn.forward_prop(&data);
        vec2array(res)
    }
}

#[export_module]
pub mod nn_module {


}


pub fn init(engine: &mut Engine) {
    log::trace!("Running STDLIB::neuralnet init");
    let module = exported_module!(nn_module);
    engine.register_static_module("neuralnet", module.into());
    engine.register_type::<NRNeuralNetwork>()
          .register_fn("NeuralNet", NRNeuralNetwork::init)
          .register_fn("add", NRNeuralNetwork::add)
          .register_fn("train", NRNeuralNetwork::train)
          .register_fn("forward", NRNeuralNetwork::forward)
          .register_fn("to_string", |x: &mut NRNeuralNetwork| format!("{:?}", x.nn) );
}
