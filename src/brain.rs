use bevy::prelude::*;
use rand::prelude::*;

const BIAS_FUNCTION: fn(f32) -> f32 = |x| (x * 2. - 1.).powi(25) + x * 0.01;
const WEIGHT_FUNCTION: fn(f32) -> f32 = |x| (x * 2. - 1.).powi(25) + x * 0.01;
const NEURON_FUNCTION: fn(f32) -> i8 = |x| (2.22 * x - 1.11).powi(15) as i8;
const CONNECTION_FUNCTION: fn(f32) -> i8 = |x| (2.38 * x - 1.19).powi(7) as i8;
const ACTIVATION_FUNCTION: fn(f32) -> f32 = |x| x.tanh();
/// Summe der [Neuron]s die als input verwendet werden und [Neuron]s die als output verwendet werden.
const IMMUNE_NEURON_COUNT: u8 = 8;

#[derive(Default, Debug, Clone, Component, Reflect)]
#[reflect(Component)]
pub struct Brain {
    neurons: Vec<Neuron>,
}

impl Brain {
    /// # Funktion `new()`
    /// Erzeugt ein neues [Brain].
    /// Erstellt beim Erzeugen `IMMUNE_NEURON_COUNT` anzahl an [Neuron]s.
    pub fn new() -> Self {
        let mut neurons = Vec::new();
        for _ in 0..IMMUNE_NEURON_COUNT {
            neurons.push(Neuron {
                inputs: Vec::new(),
                bias: 0.,
                output: 0.,
            });
        }
        Self { neurons }
    }

    /// # Funktion Mutate
    /// Erstellt/ Löscht eine Zufällig Anzahl an [Neuron]s, die Abhängig von der Anzahl [Neuron]s im Vergleich zur Norm (35)\
    /// Beim Erstellen eines [Neuron] wird diesem eine eingehende Verbindung ([NeuronInput]) und eine ausgehende Verbindung zugewiesen.
    /// # Panics
    /// Panic kann auftreten, falls das [Brain] keine [Neuron]s enthält.
    pub fn mutate(&mut self) {
        let neuron_change = NEURON_FUNCTION(random::<f32>());
        if neuron_change as i8 >= 0 {
            // Fügt Neuronen hinzu
            for _ in 0..neuron_change {
                let mut new_neuron = Neuron {
                    inputs: Vec::new(),
                    bias: 0.,
                    output: 0.,
                };
                new_neuron.inputs.push(NeuronInput {
                    neuron_id: (random::<f32>() * self.neurons.len() as f32) as usize,
                    weight: random::<f32>() * 2. - 1.,
                });
                let neurons_len = self.neurons.len();
                self.neurons[(random::<f32>() * neurons_len as f32) as usize]
                    .inputs
                    .push(NeuronInput {
                        neuron_id: neurons_len,
                        weight: (random::<f32>() * 2. - 1.),
                    });
                self.neurons.push(new_neuron);
            }
        } else {
            // Entfernt Neuronen
            for _ in 0..(-neuron_change) {
                let to_delete_neuron_id = (random::<f32>() * self.neurons.len() as f32) as usize;
                // Überspringe entfernen, wenn immune neuron betroffen ist
                if to_delete_neuron_id < IMMUNE_NEURON_COUNT as usize {
                    continue;
                }
                for neuron in &mut self.neurons {
                    let mut input_index = 0;
                    while input_index < neuron.inputs.len() {
                        if neuron.inputs[input_index].neuron_id == to_delete_neuron_id {
                            neuron.inputs.remove(input_index);
                        } else {
                            input_index += 1;
                        }
                    }
                    for input in &mut neuron.inputs {
                        if input.neuron_id > to_delete_neuron_id {
                            input.neuron_id -= 1;
                        }
                    }
                }
                self.neurons.remove(to_delete_neuron_id);
            }
        }
        let connection_change = CONNECTION_FUNCTION(random::<f32>());
        if connection_change >= 0 {
            // Fügt Connections hinzu
            for _ in 0..connection_change {
                let new_connection_from_neuron_id =
                    (random::<f32>() * self.neurons.len() as f32) as usize;
                let new_connection_to_neuron_id =
                    (random::<f32>() * self.neurons.len() as f32) as usize;
                self.neurons[new_connection_to_neuron_id]
                    .inputs
                    .push(NeuronInput {
                        neuron_id: new_connection_from_neuron_id,
                        weight: (random::<f32>() * 2. - 1.),
                    });
            }
        } else {
            // Entfernt Connections
            for _ in 0..(-connection_change) {
                let connection_count = self
                    .neurons
                    .iter()
                    .fold(0, |count, neuron| count + neuron.inputs.len());
                let to_delete_connection = (random::<f32>() * connection_count as f32) as usize;
                let mut counted_connections = 0;
                for neuron in &mut self.neurons {
                    let new_counted_connections = counted_connections + neuron.inputs.len();
                    if new_counted_connections > to_delete_connection {
                        let neuron_input_index = to_delete_connection - counted_connections;
                        neuron.inputs.remove(neuron_input_index);
                        break;
                    }
                    counted_connections = new_counted_connections;
                }
            }
        }
        // Verändert weight und bias von Neuronen zufällig
        for neuron in &mut self.neurons {
            neuron.bias += BIAS_FUNCTION(random::<f32>());
            for neuron_input in &mut neuron.inputs {
                neuron_input.weight += WEIGHT_FUNCTION(random::<f32>());
            }
        }
    }

    /// # Funktion WriteNeuron
    /// Gibt den [Output] für ein bestimmtes [Neuron] zurück\
    /// (Falls es existiert, sonst [None])
    pub fn read_neuron(&self, neuron_id: usize) -> Option<f32> {
        if let Some(neuron) = self.neurons.get(neuron_id) {
            Some(neuron.output)
        } else {
            None
        }
    }

    /// # Funktion WriteNeuron
    /// Setzt den [Output] für ein bestimmtes [Neuron].\
    /// (Falls es existiert)
    pub fn write_neuron(&mut self, neuron_id: usize, value: f32) {
        if let Some(neuron) = self.neurons.get_mut(neuron_id) {
            neuron.output = value;
        }
    }

    ///  # Funktion Tick:
    ///  Geht über alle [Neuron]s
    ///
    ///  Für jedes [Neuron]: \
    ///  Erstellt die Summe von den vorherigen `Output`s der Neuronen, auf die die [NeuronInputs] des [Neuron] zeigen,
    ///  und multipliziert die einzelnen [Output]s mit mit dem `weight` des entsprchenden `input`s.\
    ///  Addiert den `bias` zu dieser Summe.\
    ///  Wendet die [ACTIVATION_FUNCTION] auf die Summe an.\
    ///  Wendet alle berechneten [Output]s auf die [Neuron]s an.
    pub fn tick(&mut self) {
        let mut outputs_temp = Vec::new();
        for neuron in &self.neurons {
            let mut new_output = neuron.bias;
            for neuron_input in &neuron.inputs {
                new_output += self.neurons[neuron_input.neuron_id].output * neuron_input.weight;
            }
            new_output = ACTIVATION_FUNCTION(new_output);
            outputs_temp.push(new_output);
        }
        for (neuron_id, neuron) in self.neurons.iter_mut().enumerate() {
            neuron.output = outputs_temp[neuron_id];
        }
    }

    /// Getter für Feld `neurons`.
    pub fn neurons(&self) -> &Vec<Neuron> {
        &self.neurons
    }
}

#[derive(Debug, Clone, Reflect, FromReflect)]
pub struct Neuron {
    pub inputs: Vec<NeuronInput>,
    pub bias: f32,
    pub output: f32,
}

#[derive(Debug, Clone, Reflect, FromReflect)]
pub struct NeuronInput {
    pub neuron_id: usize,
    pub weight: f32,
}
