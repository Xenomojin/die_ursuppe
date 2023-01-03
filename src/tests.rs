#[test]
fn brain_new_immune_neuron_count_test() {
    use crate::brain::{Brain, IMMUNE_NEURON_COUNT};

    // Neues Brain erstellen
    let brain = Brain::new();

    // Garantieren, dass ein neues Brain die richtige Anzahl an Neuronen hat
    assert_eq!(brain.neurons().len() as u8, IMMUNE_NEURON_COUNT);
}

#[test]
fn brain_mutate_immune_neuron_count_test() {
    use crate::brain::{Brain, IMMUNE_NEURON_COUNT};

    // Neues Brain erstellen
    let mut brain = Brain::new();

    // Brain 10000 mal mutieren lassen
    for _ in 0..10000 {
        brain.mutate();

        // Garantieren, dass das Brain nie weniger als die Mindestanzahl an Neuronen hat
        assert!(brain.neurons().len() >= IMMUNE_NEURON_COUNT as usize);
    }
}

#[test]
fn brain_read_neuron_write_neuron_test() {
    use crate::brain::{Brain, IMMUNE_NEURON_COUNT};

    // Neues Brain erstellen
    let mut brain = Brain::new();

    if IMMUNE_NEURON_COUNT > 0 {
        // Garantieren, dass es das Neuron 0 gibt und es mit dem Standartwert 0 initialisiert wurde
        assert_eq!(brain.read_neuron(0), Some(0.));
    }

    // Garantieren, dass read_neuron None zurück gibt, falls das gewünschte Neuron nicht existiert
    assert_eq!(brain.read_neuron(IMMUNE_NEURON_COUNT as usize), None);

    if IMMUNE_NEURON_COUNT > 0 {
        brain.write_neuron(0, 5.);
        // Garantieren, dass das gerade beschriebene Neuron 0 jetzt den neuen Wert enthält
        assert_eq!(brain.read_neuron(0), Some(5.));
    }
}
