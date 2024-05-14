extern crate log;

use crate::zbus_lib::neuralnet;
use crate::zbus_lib::sampler::Sampler;
use rhai::{NativeCallContext, EvalAltResult};

fn train_flat_curve(_nn: &mut neuralnet::NRNeuralNetwork) {

}

pub fn new_sampler_nn(_context: NativeCallContext) -> Result<neuralnet::NRNeuralNetwork, Box<EvalAltResult>> {
    let mut nn = neuralnet::NRNeuralNetwork::init(128, 5, 128*5);
    train_flat_curve(&mut nn);
    Ok(nn)
}

impl neuralnet::NRNeuralNetwork {
    pub fn analyze(&mut self, _data: Sampler) {

    }
}
