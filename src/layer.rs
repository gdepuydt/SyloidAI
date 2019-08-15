use crate::matrix::Matrix;


/*
NOTE: this is all very preliminary... 
*/
pub struct Layer<'a> {

    activations: &'a mut Matrix,
    delta_activations: &'a mut Matrix,
    
    sums: &'a mut Matrix,
    delta_sums: &'a mut Matrix,
    
    weights: &'a mut Matrix,
    delta_weights: &'a mut Matrix,
    
    bias: &'a mut Matrix,
    delta_bias: &'a mut Matrix,

    neuron_count: usize,

    previous_layer: &'a mut Layer<'a>,
    next_layer: &'a mut Layer<'a>,
}

impl Layer<'_> {
    pub fn new(neuron_count: usize) -> Self{
        unimplemented!();
    }

    pub fn set_previous_layer(previous_layer: &mut Layer)  {

    }

    pub fn feed_forward() {
        
    } 

    pub fn back_prop() {

    }

}