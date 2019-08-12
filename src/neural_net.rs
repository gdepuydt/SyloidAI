pub fn hello(name: &str) {
    println!("Hello there again, {}!", name)
}
use crate::layer::Layer;
use crate::trainingset::Trainingset;

pub struct NeuralNet<'a> {
     
    layers: Vec<&'a mut Layer<'a>>,
    training_set: &'a Trainingset,
}

impl NeuralNet<'_> {
    pub fn new() -> Self {
        unimplemented!();
    }

    pub fn add_input_layer(input_neuron_count: usize, trainingset_batch_size: usize) {
        unimplemented!();
    }

    pub fn add_hidden_layer(neuron_count: usize) {
        unimplemented!();
    }

    pub fn add_output_layer(neuron_count: usize) {
        unimplemented!();
    }

    pub fn forward_prop() {
        unimplemented!();
    }

    pub fn back_prop() {
        unimplemented!();
    }
}