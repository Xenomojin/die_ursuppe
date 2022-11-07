use std::f32::consts::PI;

use bevy::prelude::*;
use rand::prelude::*;

use crate::brain::Brain;

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
) {
    for (_, mut brain, mut position, mut rotation, mut velocity, mut energy, mut age) in
        &mut cell_query
    {
        // inputs berechnen und in input neuronen schreiben
        let mut angle_to_next_food = 0.;
        let mut distance_to_next_food = f32::INFINITY;
        for (_, food_position, _) in &food_query {
            let food_relative_position = Position {
                x: food_position.x - position.x,
                y: food_position.y - position.y,
            };
            let distance = (food_relative_position.x * food_relative_position.x
                + food_relative_position.y * food_relative_position.y)
                .sqrt();
            if distance < distance_to_next_food {
                distance_to_next_food = distance;
                angle_to_next_food = food_relative_position.y.atan2(food_relative_position.x);
            }
        }
        brain.write_neuron(0, angle_to_next_food);

        // brain rechnen lassen
        brain.tick();

        // output neuronen auslesen
        **rotation += brain.read_neuron(1).unwrap();
        let acceleration = brain.read_neuron(2).unwrap();

        // simulationsschritte ausführen
        let new_velocity = Velocity {
            x: velocity.x + rotation.cos() * acceleration,
            y: velocity.y + rotation.sin() * acceleration,
        };
        let kinetic_energy = (velocity.x * velocity.x + velocity.y * velocity.y).sqrt();
        let new_kinetic_energy =
            (new_velocity.x * new_velocity.x + new_velocity.y * new_velocity.y).sqrt();
        **energy -= (new_kinetic_energy - kinetic_energy).abs();
        *velocity = new_velocity;
        position.x += velocity.x;
        position.y += velocity.y;
        velocity.x *= 0.5;
        velocity.y *= 0.5;
        for (_, food_position, mut food_energy) in &mut food_query {
            let food_relative_position = Position {
                x: food_position.x - position.x,
                y: food_position.y - position.y,
            };
            let distance = (food_relative_position.x * food_relative_position.x
                + food_relative_position.y * food_relative_position.y)
                .sqrt();
            if distance < 3. + 5. {
                **energy += **food_energy;
                **food_energy = 0.;
            }
        }
        if **energy > 200. {
            // erschaffe brain
            let mut child_brain = brain.clone();
            child_brain.mutate();

            // copiere position
            let child_position = Position {
                x: position.x,
                y: position.y,
            };

            // copiere rotation
            let child_rotation = Rotation(**rotation);

            // spawnen
            commands.spawn_bundle((
                Cell,
                child_position,
                child_rotation,
                Velocity { x: 0., y: 0. },
                Age(0),
                Energy(100.),
                child_brain,
            ));
            **energy -= 50.;
        }
        **energy -= 0.4;
        **age += 1;
    }

    // zellen ohne energie löschen
    for (entity, .., energy, age) in &cell_query {
        if **energy <= 0. {
            commands.entity(entity).despawn();
            info!("zelle ist im alter von {} ticks gestorben", **age);
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
        let distance_from_center = random::<f32>() * 1000.;
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
        let distance_from_center = random::<f32>() * 1000.;
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
