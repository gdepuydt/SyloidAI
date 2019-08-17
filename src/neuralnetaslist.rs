pub struct NN {
    head: Link,
}

pub enum Link {
    Empty,
    More(Box<Layer>),
}

pub enum Layer {
    Empty,
    InputLayer(Box<InputLayer>),
    HiddenLayer(Box<HiddenLayer>),
    OutputLayer(Box<OutputLayer>),
}

pub struct InputLayer {
neuron_count: usize,
next: Link,
}

impl InputLayer {
    pub fn new(neuron_count: usize, next: Link) -> Self {
        InputLayer {
            neuron_count,
            next,
        }
    } 
}

pub struct HiddenLayer {
neuron_count: usize,
previous: Link,
next: Link, 
}

impl HiddenLayer {
    pub fn new(neuron_count: usize, previous: Link, next: Link) -> Self {
        HiddenLayer {
            neuron_count,
            previous,
            next,
        }
    }
}

pub struct OutputLayer {
neuron_count: usize,
previous: Link,
}

impl OutputLayer {
    pub fn new(neuron_count: usize, previous: Link ) -> OutputLayer {
        OutputLayer {
            neuron_count,
            previous,
        }
    }
}


impl NN {
    pub fn new() -> Self {
        NN {
            head: Link::Empty,
        }
    }

    pub fn push(&mut self, layer: Layer) {
        match layer {
            InputLayer =>  println!("add input layer"),
            HiddenLayer => println!("add hidden layer"),
            OutputLayer => println!("add output layer"),
            Empty => println!("no layer to add."),
        }
    }
}
