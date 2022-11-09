use std::f32::consts::PI;

use bevy::prelude::*;
use rand::prelude::*;

use crate::brain::Brain;

pub struct SimulationSettings {
    pub rotation_speed_max: f32,
    pub acceleration_max: f32,
    /// Wert zwischen 0 (kein damping) und 1 (100% damping)
    pub velocity_damping: f32,
    pub base_energy_drain: f32,
    pub cell_radius: f32,
    pub food_radius: f32,
}

impl Default for SimulationSettings {
    fn default() -> Self {
        Self {
            rotation_speed_max: 1.,
            acceleration_max: 1.,
            velocity_damping: 0.5,
            base_energy_drain: 1.,
            cell_radius: 5.,
            food_radius: 3.,
        }
    }
}

#[derive(Debug, Component)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Component, Deref, DerefMut)]
pub struct Rotation(pub f32);

#[derive(Debug, Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Component, Deref, DerefMut)]
pub struct Age(pub u32);

#[derive(Debug, Component, Deref, DerefMut)]
pub struct Energy(pub f32);

#[derive(Debug, Component)]
pub struct Cell;

#[derive(Debug, Component)]
pub struct Food;

pub fn tick(
    mut commands: Commands,
    mut cell_query: Query<
        (
            Entity,
            &mut Brain,
            &mut Position,
            &mut Rotation,
            &mut Velocity,
            &mut Energy,
            &mut Age,
        ),
        (With<Cell>, Without<Food>),
    >,
    mut food_query: Query<(Entity, &mut Position, &mut Energy), (With<Food>, Without<Cell>)>,
    simulation_settings: ResMut<SimulationSettings>,
) {
    for (_, mut brain, mut position, mut rotation, mut velocity, mut energy, mut age) in
        &mut cell_query
    {
        let _iterate_on_cell_span = info_span!("iterate_on_cell").entered();

        // inputs berechnen und in input neuronen schreiben
        {
            let _calculate_brain_inputs_span = info_span!("calculate_brain_inputs").entered();
            let mut nearest_food_angle = 0.;
            let mut nearest_food_distance_squared = f32::INFINITY;
            for (_, food_position, _) in &food_query {
                let food_relative_position = Position {
                    x: food_position.x - position.x,
                    y: food_position.y - position.y,
                };
                let distance_squared = food_relative_position.x * food_relative_position.x
                    + food_relative_position.y * food_relative_position.y;
                if distance_squared < nearest_food_distance_squared {
                    nearest_food_distance_squared = distance_squared;
                    nearest_food_angle = food_relative_position.y.atan2(food_relative_position.x);
                }
            }
            brain.write_neuron(0, nearest_food_angle);
        }

        // brain rechnen lassen
        {
            let _tick_brain_span =
                info_span!("tick_brain", neurons = brain.neurons().len()).entered();
            brain.tick();
        }

        // output neuronen auslesen
        let rotation_neuron_output = brain.read_neuron(1).unwrap();
        let acceleration_neuron_output = brain.read_neuron(2).unwrap();

        // simulationsschritte ausführen
        **rotation += rotation_neuron_output * simulation_settings.rotation_speed_max;
        let new_velocity = Velocity {
            x: velocity.x
                + rotation.cos()
                    * acceleration_neuron_output
                    * simulation_settings.acceleration_max,
            y: velocity.y
                + rotation.sin()
                    * acceleration_neuron_output
                    * simulation_settings.acceleration_max,
        };
        let kinetic_energy = velocity.x * velocity.x + velocity.y * velocity.y;
        let new_kinetic_energy = new_velocity.x * new_velocity.x + new_velocity.y * new_velocity.y;
        **energy -= (new_kinetic_energy - kinetic_energy).abs();
        *velocity = new_velocity;
        position.x += velocity.x;
        position.y += velocity.y;
        velocity.x *= 1. - simulation_settings.velocity_damping;
        velocity.y *= 1. - simulation_settings.velocity_damping;
        {
            let _calculate_collisions_span = info_span!("calculate_collisions").entered();
            let distance_min_squared = (simulation_settings.cell_radius
                + simulation_settings.food_radius)
                * (simulation_settings.cell_radius + simulation_settings.food_radius);
            for (_, food_position, mut food_energy) in &mut food_query {
                let food_relative_position = Position {
                    x: food_position.x - position.x,
                    y: food_position.y - position.y,
                };
                let distance_squared = food_relative_position.x * food_relative_position.x
                    + food_relative_position.y * food_relative_position.y;
                if distance_squared < distance_min_squared {
                    **energy += **food_energy;
                    **food_energy = 0.;
                }
            }
        }
        if **energy > 200. {
            // kind spawnen
            let mut child_brain = brain.clone();
            child_brain.mutate();
            commands.spawn_bundle((
                Cell,
                Position {
                    x: position.x,
                    y: position.y,
                },
                Rotation(**rotation),
                Velocity { x: 0., y: 0. },
                Age(0),
                Energy(100.),
                child_brain,
            ));
            **energy -= 100.;
        }
        **energy -= simulation_settings.base_energy_drain;
        **age += 1;
    }

    // zellen ohne energie löschen
    for (entity, brain, .., energy, age) in &cell_query {
        if **energy <= 0. {
            commands.entity(entity).despawn();
            info!(
                "zelle ist im alter von {} ticks mit {} neuronen gestorben",
                **age,
                brain.neurons().len()
            );
        }
    }

    // essen ohne energie löschen
    for (entity, _, energy) in &food_query {
        if **energy <= 0. {
            commands.entity(entity).despawn();
        }
    }
}

pub struct SpawnCell {
    pub energy: f32,
}

pub fn spawn_cells(mut commands: Commands, mut spawn_cell_events: EventReader<SpawnCell>) {
    for spawn_cell_event in spawn_cell_events.iter() {
        let mut brain = Brain::new();
        brain.mutate();
        let angle_from_center = random::<f32>() * 2. * PI;
        let distance_from_center = random::<f32>() * 500.;
        commands.spawn_bundle((
            Cell,
            Position {
                x: angle_from_center.cos() * distance_from_center,
                y: angle_from_center.sin() * distance_from_center,
            },
            Rotation(random::<f32>() * 2. * PI),
            Velocity { x: 0., y: 0. },
            Age(0),
            Energy(spawn_cell_event.energy),
            brain,
        ));
    }
}

pub struct SpawnFood {
    pub energy: f32,
}

pub fn spawn_food(mut commands: Commands, mut spawn_food_events: EventReader<SpawnFood>) {
    for spawn_food_event in spawn_food_events.iter() {
        let angle_from_center = random::<f32>() * 2. * PI;
        let distance_from_center = random::<f32>() * 500.;
        commands.spawn_bundle((
            Food,
            Position {
                x: angle_from_center.cos() * distance_from_center,
                y: angle_from_center.sin() * distance_from_center,
            },
            Energy(spawn_food_event.energy),
        ));
    }
}

pub struct Clear;

pub fn clear(
    mut commands: Commands,
    mut clear_events: EventReader<Clear>,
    entities_to_clear_query: Query<Entity, Or<(With<Cell>, With<Food>)>>,
) {
    for _ in clear_events.iter() {
        for entity in &entities_to_clear_query {
            commands.entity(entity).despawn();
        }
    }
}
