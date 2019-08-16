use crate::matrix::Matrix;


/*
NOTE: this is all very preliminary... 
*/
pub struct Layer {

    activations: Option<Box<Matrix>>,
    delta_activations: Option<Box<Matrix>>,
    
    sums: Option<Box<Matrix>>,
    delta_sums: Option<Box<Matrix>>,
    
    weights: Option<Box<Matrix>>,
    delta_weights: Option<Box<Matrix>>,
    
    bias: Option<Box<Matrix>>,
    delta_bias: Option<Box<Matrix>>,

    neuron_count: usize,

    // This is a double linked list... which is hard in Rust... I will need to study for this one... :/
    // And these layers are stored in a vector with each element holding knowledge of its neighbours...........
    // https://rust-unofficial.github.io/too-many-lists/ see if this helps.
    previous_layer: Option<Box<Layer>>,
    next_layer: Option<Box<Layer>>,
}

impl<'a> Layer {
    pub fn new(neuron_count: usize) -> Box<Layer>{
        Box::new(Layer {
            neuron_count: neuron_count,
            
            //None:
            activations: None,
            delta_activations: None,
            sums: None,
            delta_sums: None,
            weights: None,
            delta_weights: None,
            bias: None,
            delta_bias: None,
            previous_layer: None,
            next_layer: None,
        })
        
    }

    pub fn set_previous_layer(& mut self, previous_layer: Box<Layer>, trainingset_batch_size: usize) {
        self.previous_layer = Some(previous_layer);
        
    }

    pub fn feed_forward() {
        
    } 

    pub fn back_prop() {

    }

    pub fn set_input_layer_activation_matrix(input_neuron_count: usize, batch_size: usize ) -> Box<Layer> {
        Box::new(Layer {
            activations: Matrix::new(input_neuron_count, batch_size),
            
            //None:
            delta_activations: None,
            sums: None,
            delta_sums: None,
            weights: None,
            delta_weights: None,
            bias: None,
            delta_bias: None,
            neuron_count: input_neuron_count,
            previous_layer: None,
            next_layer: None,

        })
        
    }

}