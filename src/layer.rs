use crate::matrix::Matrix;


/*
NOTE: this is all very preliminary... 
*/
pub struct Layer {

    activations: Box<Matrix>,
    delta_activations: Box<Matrix>,
    
    sums: Box<Matrix>,
    delta_sums: Box<Matrix>,
    
    weights: Box<Matrix>,
    delta_weights: Box<Matrix>,
    
    bias: Box<Matrix>,
    delta_bias: Box<Matrix>,

    neuron_count: usize,

    previous_layer: Box<Layer>,
    next_layer: Box<Layer>,
}

impl Layer {
    pub fn new(neuron_count: usize) -> Self{
        // This can't be right...
        Layer {
            neuron_count: neuron_count,
            activations: Box::new(Matrix::new()),
            delta_activations: Box::new(Matrix::new()),
            sums: Box::new(Matrix::new()),
            delta_sums: Box::new(Matrix::new()),
            weights: Box::new(Matrix::new()),
            delta_weights: Box::new(Matrix::new()),
            bias: Box::new(Matrix::new()),
            delta_bias: Box::new(Matrix::new()),
            previous_layer: Box::new(Layer::new(neuron_count)),
            next_layer: Box::new(Layer::new(neuron_count)),
        }
    }

    pub fn set_previous_layer(previous_layer: &mut Layer)  {

    }

    pub fn feed_forward() {
        
    } 

    pub fn back_prop() {

    }

}