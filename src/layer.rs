use crate::matrix::Matrix;


/*
NOTE: this is all very preliminary... 
*/
pub struct Layer<'a> {

    activations: &'a mut Matrix<'a>,
    delta_activations: &'a mut Matrix<'a>,
    
    sums: &'a mut Matrix<'a>,
    delta_sums: &'a mut Matrix<'a>,
    
    weights: &'a mut Matrix<'a>,
    delta_weights: &'a mut Matrix<'a>,
    
    bias: &'a mut Matrix<'a>,
    delta_bias: &'a mut Matrix<'a>,

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