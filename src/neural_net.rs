use crate::layer::Layer;
use crate::trainingset::Trainingset;

pub struct NeuralNet {
     
    layers: Vec<Box<Layer>>,
    training_set: Box<Trainingset>,
    trainingset_batch_size: usize,
}

impl<'a> NeuralNet {
    /*pub fn new(input_neuron_count: usize, trainingset_batch_size: usize, output_neuron_count: usize) -> Self {
        unimplemented!();
    }*/

    pub fn add_input_layer(&mut self, input_neuron_count: usize, trainingset_batch_size: usize) {
        if self.layers.len() != 0 {
            panic!("Neural Net already has an input layer.");
        }
        self.trainingset_batch_size = trainingset_batch_size;
        let input_layer = Layer::set_input_layer_activation_matrix(input_neuron_count, trainingset_batch_size);
        self.layers.push(input_layer);
    }

    pub fn add_hidden_layer(&mut self, neuron_count: usize) {
        if self.layers.len() < 1 {
            panic!("Neural Net requires an input layer before adding a hidden layer.");
        }
        let mut hidden_layer = Layer::new(neuron_count);
        hidden_layer.set_previous_layer(self.layers.last().unwrap(), self.trainingset_batch_size);
        self.layers.push(hidden_layer);
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