use syloid_ai::neuralnetaslist::{NN, Link, Layer, InputLayer, HiddenLayer, OutputLayer};

fn main() {
    let mut nn = NN::new();
    let i = InputLayer::new(1000, Link::Empty);
    let h = HiddenLayer::new(10, Link::Empty, Link::Empty);
    let o = OutputLayer::new(5, Link::Empty);
    
    nn.push(Layer::InputLayer(Box::new(i)));
    nn.push(Layer::HiddenLayer(Box::new(h)));
    nn.push(Layer::OutputLayer(Box::new(o)));

}