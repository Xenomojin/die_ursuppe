use rand::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Brain {
    neurons: Vec<Neuron>,
}

impl Brain {
    /*
    pub fn mutate(&mut self) {
        for neuron in &mut self.neurons {
            let rand = random::<f32>() * 2. - 1;
            neuron.bias += rand.powi(19) + rand * 0.05;
            for neuron_input in &mut neuron.inputs {
                let rand = random::<f32>() * 2. - 1;
                neuron_input.weight += rand.powi(19) + rand * 0.05;
            }
        }
        let mut drift = self.neurons.len() / 35. - 1.;
        let mut rand = (random::<f32>() * 2. - 1. - drift) * 2.;
        if rand as i8 >= 0 {
            for i in 0..rand {
                self.neurons.push(1);
            }

        } else {

        }

    }*/

    pub fn read_neuron(&self, neuron_id: usize) -> Option<f32> {
        if let Some(neuron) = self.neurons.get(neuron_id) {
            Some(neuron.output)
        } else {
            None
        }
    }

    pub fn write_neuron(&mut self, neuron_id: usize, value: f32) {
        if let Some(neuron) = self.neurons.get_mut(neuron_id) {
            neuron.output = value;
        }
    }

    pub fn tick(&mut self) {
        let mut outputs_temp = Vec::new();
        for neuron in &self.neurons {
            let mut new_output = neuron.bias;
            for neuron_input in &neuron.inputs {
                new_output += self.neurons[neuron_input.neuron_id].output * neuron_input.weight;
            }
            new_output = new_output.tanh();
            outputs_temp.push(new_output);
        }
        for (neuron_id, neuron) in self.neurons.iter_mut().enumerate() {
            neuron.output = outputs_temp[neuron_id];
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
struct Neuron {
    inputs: Vec<NeuronInput>,
    bias: f32,
    output: f32,
}

/*
impl Neuron {
    fn new() -> Self {

    }
}*/

#[derive(Clone, Serialize, Deserialize)]
struct NeuronInput {
    neuron_id: usize,
    weight: f32,
}
