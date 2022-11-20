use std::f32::consts::PI;

use bevy::{ecs::schedule::ShouldRun, prelude::*, time::Stopwatch};
use rand::prelude::*;

use crate::{
    brain::Brain,
    ui::{CellCountStatisticUi, ChildCountStatisticUi, ControlCenterUi, NeuronCountStatisticUi},
};

/// Größe der Chunks
pub const CHUNK_SIZE: f32 = 50.;
/// Map Größe in Chunks
pub const MAP_SIZE: u32 = 40;

/// Einstellungen für den Verlauf der Simulation
#[derive(Resource)]
pub struct SimulationSettings {
    /// Raduis einer Zelle
    pub cell_radius: f32,
    /// Raduis eines Foods
    pub food_radius: f32,
    pub base_energy_drain: f32,
    pub neuron_energy_drain: f32,
    pub connection_energy_drain: f32,
    /// Die estrebte Dauer in Sekunden zwischen Ticks
    pub tick_delta_seconds: f32,
    /// Ob die Simulation pausiert ist
    pub paused: bool,
}

// Setzt die Standartwerte für SimulationSettings
impl Default for SimulationSettings {
    fn default() -> Self {
        Self {
            cell_radius: 5.,
            food_radius: 3.,
            base_energy_drain: 0.4,
            neuron_energy_drain: 0.02,
            connection_energy_drain: 0.004,
            tick_delta_seconds: 0.02,
            paused: true,
        }
    }
}

#[derive(Default, Debug, Component, Deref, DerefMut)]
pub struct Foodlist(Vec<Entity>);

#[derive(Debug, Component)]
pub struct ChunkSettings {
    /// Die Wahrscheinlichkeit, dass in diesem Chunk Essen spawnt.
    /// Werte über 1 werden akzeptiert (1.5 bedeutet, dass mindestens
    /// ein Essen gespawned wird und ein 50%-ige Chance besteht,
    /// dass ein weiteres gespawned wird)
    pub spawn_chance: f32,
    /// Die Energie mit der das Essen gespawned wird.
    pub spawned_food_energy: f32,
    pub rotation_speed_max: f32,
    pub acceleration_max: f32,
    /// Wert zwischen 0 (kein damping) und 1 (100% damping)
    pub velocity_damping: f32,
}

impl Default for ChunkSettings {
    fn default() -> Self {
        Self {
            spawn_chance: 0.,
            spawned_food_energy: 200.,
            rotation_speed_max: 1.,
            acceleration_max: 2.,
            velocity_damping: 0.5,
        }
    }
}

#[derive(Default, Resource, Deref, DerefMut)]
pub struct ChunkList(Vec<Vec<Entity>>);

#[derive(Default, Clone, Copy, Debug, Component)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Default, Debug, Component, Deref, DerefMut)]
pub struct Rotation(pub f32);

#[derive(Default, Debug, Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

#[derive(Default, Debug, Component, Deref, DerefMut)]
pub struct Energy(pub f32);

#[derive(Default, Debug, Component)]
pub struct CellStats {
    pub age: u32,
    pub children_count: u32,
}

#[derive(Default, Debug, Component)]
pub struct Cell;

#[derive(Default, Debug, Component)]
pub struct Food;

#[derive(Default, Debug, Component)]
pub struct Chunk;

#[derive(Default, Bundle)]
pub struct CellBundle {
    pub cell: Cell,
    pub brain: Brain,
    pub position: Position,
    pub rotation: Rotation,
    pub velocity: Velocity,
    pub energy: Energy,
    pub stats: CellStats,
}

#[derive(Default, Bundle)]
pub struct FoodBundle {
    pub food: Food,
    pub position: Position,
    pub energy: Energy,
}

#[derive(Default, Bundle)]
pub struct ChunkBundle {
    pub chunk: Chunk,
    pub position: Position,
    pub foodlist: Foodlist,
    pub chunk_settings: ChunkSettings,
}

pub fn setup(mut commands: Commands, mut chunk_list: ResMut<ChunkList>) {
    for idx in 0..MAP_SIZE {
        chunk_list.push(Vec::new());
        for idy in 0..MAP_SIZE {
            let chunk_entity = commands
                .spawn(ChunkBundle {
                    position: Position {
                        x: (idx as f32 * CHUNK_SIZE + CHUNK_SIZE / 2.),
                        y: (idy as f32 * CHUNK_SIZE + CHUNK_SIZE / 2.),
                    },
                    ..default()
                })
                .id();
            chunk_list[idx as usize].push(chunk_entity);
        }
    }
}

pub fn tick_cells(
    mut commands: Commands,
    mut cell_query: Query<
        (
            &mut Brain,
            &mut Position,
            &mut Rotation,
            &mut Velocity,
            &mut Energy,
            &mut CellStats,
        ),
        (With<Cell>, Without<Food>, Without<Chunk>),
    >,
    mut food_query: Query<
        (Entity, &mut Position, &mut Energy),
        (With<Food>, Without<Cell>, Without<Chunk>),
    >,
    mut cell_count_statistic_ui: ResMut<CellCountStatisticUi>,
    mut child_count_statistic_ui: ResMut<ChildCountStatisticUi>,
    mut neuron_count_statistic_ui: ResMut<NeuronCountStatisticUi>,
    chunk_query: Query<(&Foodlist, &ChunkSettings), (With<Chunk>, Without<Cell>, Without<Food>)>,
    chunk_list: Res<ChunkList>,
    simulation_settings: Res<SimulationSettings>,
    tick: Res<Tick>,
) {
    let mut cell_count = 0;
    let mut children_count_sum = 0;
    let mut neuron_count_sum = 0;
    for (mut brain, mut position, mut rotation, mut velocity, mut energy, mut stats) in
        &mut cell_query
    {
        let _iterate_on_cell_span = info_span!("iterate_on_cell").entered();

        cell_count += 1;
        children_count_sum += stats.children_count;
        neuron_count_sum += brain.neurons().len();

        // chunk berechnen
        let chunk_idx = (position.x / CHUNK_SIZE) as i32;
        let chunk_idy = (position.y / CHUNK_SIZE) as i32;
        let chunk_settings = chunk_query
            .get_component::<ChunkSettings>(chunk_list[chunk_idx as usize][chunk_idy as usize])
            .unwrap();

        // inputs berechnen und in input neuronen schreiben
        {
            let _calculate_brain_inputs_span = info_span!("calculate_brain_inputs").entered();

            let mut chunk_entities = Vec::with_capacity(9);
            for jdx in -1..1 {
                if chunk_idx + jdx >= 0 && chunk_idx + jdx < MAP_SIZE as i32 {
                    for jdy in -1..1 {
                        if chunk_idy + jdy >= 0 && chunk_idy + jdy < MAP_SIZE as i32 {
                            chunk_entities.push(
                                chunk_list[(chunk_idx + jdx) as usize][(chunk_idy + jdy) as usize],
                            );
                        }
                    }
                }
            }
            let mut nearest_food_distance_squared = f32::INFINITY;
            let mut nearest_food_position = Position::default();
            for (foodlist, _) in chunk_query.iter_many(chunk_entities) {
                for (_, food_position, _) in food_query.iter_many(&**foodlist) {
                    let food_relative_position = Position {
                        x: food_position.x - position.x,
                        y: food_position.y - position.y,
                    };
                    let distance_squared = food_relative_position.x * food_relative_position.x
                        + food_relative_position.y * food_relative_position.y;
                    if distance_squared < nearest_food_distance_squared {
                        nearest_food_distance_squared = distance_squared;
                        nearest_food_position = *food_position;
                    }
                }
            }
            let nearest_food_angle = nearest_food_position.y.atan2(nearest_food_position.x);
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
        // rotieren und geschwindigkeit passend verändern

        **rotation += rotation_neuron_output * chunk_settings.rotation_speed_max/*imilian*/;
        let new_velocity = Velocity {
            x: velocity.x
                + rotation.cos() * acceleration_neuron_output * chunk_settings.acceleration_max,
            y: velocity.y
                + rotation.sin() * acceleration_neuron_output * chunk_settings.acceleration_max,
        };
        // kinetische energie berechnen und von energie abziehen
        let kinetic_energy = velocity.x * velocity.x + velocity.y * velocity.y;
        let new_kinetic_energy = new_velocity.x * new_velocity.x + new_velocity.y * new_velocity.y;
        **energy -= (new_kinetic_energy - kinetic_energy).abs();
        // geschwindikeit und position berechen
        *velocity = new_velocity;
        position.x += velocity.x;
        position.y += velocity.y;
        if position.x < 0.
            || position.y < 0.
            || position.x >= (MAP_SIZE as f32 * CHUNK_SIZE) as f32
            || position.y >= (MAP_SIZE as f32 * CHUNK_SIZE) as f32
        {
            **energy = 0.;
            continue;
        }
        velocity.x *= 1. - chunk_settings.velocity_damping;
        velocity.y *= 1. - chunk_settings.velocity_damping;
        // essen einsammeln
        {
            // kollisionen berechnen
            let _calculate_collisions_span = info_span!("calculate_collisions").entered();
            // benötigte distanz berechen (squared um sqrt(x) zu vermeiden)
            let distance_min_squared = (simulation_settings.cell_radius
                + simulation_settings.food_radius)
                * (simulation_settings.cell_radius + simulation_settings.food_radius);
            // tatsächliche kollisionen berechnen
            let mut chunk_entities = Vec::with_capacity(9);
            for jdx in -1..1 {
                if chunk_idx + jdx >= 0 && chunk_idx + jdx < MAP_SIZE as i32 {
                    for jdy in -1..1 {
                        if chunk_idy + jdy >= 0 && chunk_idy + jdy < MAP_SIZE as i32 {
                            chunk_entities.push(
                                chunk_list[(chunk_idx + jdx) as usize][(chunk_idy + jdy) as usize],
                            );
                        }
                    }
                }
            }
            for (foodlist, _) in chunk_query.iter_many(chunk_entities) {
                let mut food_query_iter = food_query.iter_many_mut(&**foodlist);
                while let Some((_, food_position, mut food_energy)) = food_query_iter.fetch_next() {
                    let food_relative_position = Position {
                        x: food_position.x - position.x,
                        y: food_position.y - position.y,
                    };
                    let distance_squared = food_relative_position.x * food_relative_position.x
                        + food_relative_position.y * food_relative_position.y;
                    if distance_squared < distance_min_squared {
                        // essen leersaugen
                        **energy += **food_energy;
                        **food_energy = 0.;
                    }
                }
            }
        }
        // kind spawnen
        if **energy > 200. {
            stats.children_count += 1;
            let mut child_brain = brain.clone();
            child_brain.mutate();
            commands.spawn(CellBundle {
                position: Position {
                    x: position.x,
                    y: position.y,
                },
                rotation: Rotation(**rotation),
                energy: Energy(**energy / 2.),
                brain: child_brain,
                ..default()
            });
            **energy /= 2.;
        }
        let mut connection_count = 0;
        for neuron in brain.neurons() {
            connection_count += neuron.inputs.len();
        }
        **energy -= simulation_settings.base_energy_drain
            + brain.neurons().len() as f32 * simulation_settings.neuron_energy_drain
            + connection_count as f32 * simulation_settings.connection_energy_drain;
        stats.age += 1;
    }
    cell_count_statistic_ui
        .points
        .push([**tick as f64, cell_count as f64]);
    if cell_count > 0 {
        child_count_statistic_ui
            .average_points
            .push([**tick as f64, children_count_sum as f64 / cell_count as f64]);
        neuron_count_statistic_ui
            .average_points
            .push([**tick as f64, neuron_count_sum as f64 / cell_count as f64]);
    }
}

pub fn spawn_food(
    mut commands: Commands,
    mut chunk_query: Query<(&mut Foodlist, &ChunkSettings, &Position), With<Chunk>>,
) {
    for (mut foodlist, chunk_settings, chunk_position) in &mut chunk_query {
        let mut to_place = chunk_settings.spawn_chance;
        while to_place > 1. {
            let food_entity = commands
                .spawn(FoodBundle {
                    position: Position {
                        x: chunk_position.x + (random::<f32>() - 0.5) * CHUNK_SIZE,
                        y: chunk_position.y + (random::<f32>() - 0.5) * CHUNK_SIZE,
                    },
                    energy: Energy(chunk_settings.spawned_food_energy),
                    ..default()
                })
                .id();
            foodlist.push(food_entity);
            to_place -= 1.;
        }
        if random::<f32>() < to_place {
            let food_entity = commands
                .spawn(FoodBundle {
                    position: Position {
                        x: chunk_position.x + (random::<f32>() - 0.5) * CHUNK_SIZE,
                        y: chunk_position.y + (random::<f32>() - 0.5) * CHUNK_SIZE,
                    },
                    energy: Energy(chunk_settings.spawned_food_energy),
                    ..default()
                })
                .id();
            foodlist.push(food_entity);
        }
    }
}

pub fn despawn_food(mut commands: Commands, food_query: Query<(Entity, &Energy), With<Food>>) {
    // essen ohne energie löschen
    for (entity, energy) in &food_query {
        if **energy <= 0. {
            commands.entity(entity).despawn();
        }
    }
}

pub fn despawn_cells(
    mut commands: Commands,
    mut child_count_statistic_ui: ResMut<ChildCountStatisticUi>,
    mut neuron_count_statistic_ui: ResMut<NeuronCountStatisticUi>,
    tick: Res<Tick>,
    cell_query: Query<(Entity, &Brain, &Energy, &CellStats), With<Cell>>,
) {
    // zellen ohne energie löschen
    for (entity, brain, energy, stats) in &cell_query {
        if **energy <= 0. {
            commands.entity(entity).despawn();
            child_count_statistic_ui
                .points
                .push([**tick as f64, stats.children_count as f64]);
            neuron_count_statistic_ui
                .points
                .push([**tick as f64, brain.neurons().len() as f64]);
        }
    }
}

#[derive(Default, Deref, DerefMut)]
pub struct TickWatch(pub Stopwatch);

#[derive(Resource, Default, Deref, DerefMut)]
pub struct Tick(pub u64);

pub fn run_on_tick(
    mut tick_timer: Local<TickWatch>,
    mut control_center_ui: ResMut<ControlCenterUi>,
    mut tick: ResMut<Tick>,
    simulation_settings: Res<SimulationSettings>,
    time: Res<Time>,
) -> ShouldRun {
    if !simulation_settings.paused
        && tick_timer.tick(time.delta()).elapsed_secs() >= simulation_settings.tick_delta_seconds
    {
        control_center_ui.actual_tick_delta_seconds_label =
            format!("{:.3}", tick_timer.elapsed_secs());
        tick_timer.reset();
        **tick += 1;
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

pub struct SpawnCell {
    pub energy: f32,
}

/// Event-Handler für `SpawnCell` events
pub fn spawn_cells(mut commands: Commands, mut spawn_cell_events: EventReader<SpawnCell>) {
    for spawn_cell_event in spawn_cell_events.iter() {
        let mut brain = Brain::new();
        brain.mutate();
        commands.spawn(CellBundle {
            position: Position {
                x: random::<f32>() * MAP_SIZE as f32 * CHUNK_SIZE,
                y: random::<f32>() * MAP_SIZE as f32 * CHUNK_SIZE,
            },
            rotation: Rotation(random::<f32>() * 2. * PI),
            energy: Energy(spawn_cell_event.energy),
            brain,
            ..default()
        });
    }
}

pub struct Clear;

/// Event-Handler für `Clear` Event
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

pub struct ApplyChunkSettings;

/// Event-Handler für `Clear` Event
pub fn apply_chunk_settings(
    mut apply_chunk_settings_events: EventReader<ApplyChunkSettings>,
    mut chunk_query: Query<(&mut ChunkSettings, &Position), With<Chunk>>,
    control_center_ui: Res<ControlCenterUi>,
) {
    let spawn_chance_left = control_center_ui.food_spawn_chance_slider_left;
    let spawn_chance_right = control_center_ui.food_spawn_chance_slider_right;
    let velocity_damping_bottom = control_center_ui.velocity_damping_slider_bottom;
    let velocity_damping_top = control_center_ui.velocity_damping_slider_top;
    for _ in apply_chunk_settings_events.iter() {
        for (mut chunk_settings, chunk_position) in &mut chunk_query {
            let chunk_idx = (chunk_position.x / CHUNK_SIZE) as f32;
            let chunk_idy = (chunk_position.y / CHUNK_SIZE) as f32;
            *chunk_settings = ChunkSettings {
                spawn_chance: spawn_chance_left
                    + (spawn_chance_right - spawn_chance_left) * chunk_idx / MAP_SIZE as f32,
                spawned_food_energy: control_center_ui.food_energy_drag_value,
                rotation_speed_max: control_center_ui.rotation_speed_max_drag_value,
                acceleration_max: control_center_ui.acceleration_max_drag_value,
                velocity_damping: velocity_damping_bottom
                    + (velocity_damping_top - velocity_damping_bottom) * chunk_idy
                        / MAP_SIZE as f32,
            };
        }
    }
}

pub struct ApplySimulationSettings;

/// Event-Handler für `ApplySimulationSettings` Event
pub fn apply_simulation_settings(
    mut apply_simulation_settings_events: EventReader<ApplySimulationSettings>,
    mut simulation_settings: ResMut<SimulationSettings>,
    control_center_ui: Res<ControlCenterUi>,
) {
    for _ in apply_simulation_settings_events.iter() {
        *simulation_settings = SimulationSettings {
            cell_radius: control_center_ui.cell_radius_drag_value,
            food_radius: control_center_ui.food_radius_drag_value,
            tick_delta_seconds: control_center_ui.tick_delta_seconds_slider,
            base_energy_drain: control_center_ui.base_energy_drain_drag_value,
            neuron_energy_drain: control_center_ui.neuron_energy_drain_drag_value,
            connection_energy_drain: control_center_ui.connection_energy_drain_drag_value,
            ..*simulation_settings
        };
    }
}

pub struct TogglePause;

/// Event-Handler für `TogglePause` Event
pub fn toggle_pause(
    mut toggle_pause_events: EventReader<TogglePause>,
    mut simulation_settings: ResMut<SimulationSettings>,
    mut control_center_ui: ResMut<ControlCenterUi>,
) {
    for _ in toggle_pause_events.iter() {
        simulation_settings.paused = !simulation_settings.paused;
        if simulation_settings.paused {
            control_center_ui.pause_button_text = "Play".to_string();
        } else {
            control_center_ui.pause_button_text = "Pause".to_string();
        }
    }
}
