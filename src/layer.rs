use crate::matrix::Matrix;


/*
NOTE: adding the reference to a Matrix and lifetime specifiers to the Layer struct, breaks the code in neural_net.rs
This needs to be sorted out before we can continue...
https://youtu.be/7mP7TDhbHdI?t=694
*/
pub struct Layer<'a> {

    activations: &'a mut Matrix,
}