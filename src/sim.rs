use crate::{
    brain::Brain,
    ui::{
        BrainSizeStatistic, CellCountStatistic, ChildCountStatistic, ControlCenterUi, Statistic,
        StatisticData,
    },
};
use bevy::{
    ecs::{
        entity::{EntityMap, MapEntities, MapEntitiesError},
        reflect::ReflectMapEntities,
        schedule::ShouldRun,
    },
    prelude::*,
    scene,
    time::Stopwatch,
};
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use std::{f32::consts::PI, fs, path::Path};

/// Einstellungen für den Verlauf der Simulation
#[derive(Resource, Serialize, Deserialize)]
pub struct SimulationSettings {
    /// Radius einer Zelle
    pub cell_radius: f32,
    /// Radius von Nahrung
    pub food_radius: f32,
    pub base_energy_drain: f32,
    pub neuron_energy_drain: f32,
    pub connection_energy_drain: f32,
    pub age_energy_drain: f32,
    pub energy_required_for_split: f32,
    pub rotation_speed_max: f32,
    pub acceleration_max: f32,
    /// Die angestrebte Dauer in Sekunden zwischen Ticks
    pub tick_delta_seconds: f32,
    pub child_cooldown: u32,
    /// Ob die Simulation pausiert ist
    pub is_paused: bool,
}

// Setzt die Standartwerte für Simulation-Settings
impl Default for SimulationSettings {
    fn default() -> Self {
        Self {
            cell_radius: 5.,
            food_radius: 3.,
            base_energy_drain: 0.4,
            neuron_energy_drain: 0.01,
            connection_energy_drain: 0.004,
            age_energy_drain: 0.00008,
            energy_required_for_split: 10.,
            rotation_speed_max: 1.,
            acceleration_max: 1.7,
            tick_delta_seconds: 0.02,
            child_cooldown: 10,
            is_paused: true,
        }
    }
}

#[derive(Default, Component, Serialize, Deserialize, Reflect)]
#[reflect(Component, MapEntities)]
pub struct ChunkRegistry {
    /// Größe der Chunks
    pub chunk_size: f32,
    /// Map Größe in Chunks
    pub map_size: u32,
    pub entries: Vec<Vec<Entity>>,
}

impl MapEntities for ChunkRegistry {
    fn map_entities(&mut self, entity_map: &EntityMap) -> Result<(), MapEntitiesError> {
        for index_x in 0..self.entries.len() {
            for index_y in 0..self.entries[index_x].len() {
                if let Ok(new_entity) =
                    entity_map.get(Entity::from_raw(self.entries[index_x][index_y].index()))
                {
                    self.entries[index_x][index_y] = new_entity;
                }
            }
        }
        Ok(())
    }
}

#[derive(Default, Component, Deref, DerefMut, Reflect)]
#[reflect(Component, MapEntities)]
pub struct Foodlist(Vec<Entity>);

impl MapEntities for Foodlist {
    fn map_entities(&mut self, entity_map: &EntityMap) -> Result<(), MapEntitiesError> {
        for entity in &mut **self {
            if let Ok(new_entity) = entity_map.get(Entity::from_raw(entity.index())) {
                *entity = new_entity;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Component, Reflect)]
#[reflect(Component)]
pub struct ChunkSettings {
    /// Die Wahrscheinlichkeit, dass in diesem Chunk Essen spawnt.
    /// Werte über 1 werden akzeptiert (1.5 bedeutet, dass mindestens
    /// ein Essen gespawned wird und ein 50%-ige Chance besteht,
    /// dass ein weiteres gespawned wird)
    pub spawn_chance: f32,
    /// Die Energie mit der das Essen gespawned wird.
    pub spawned_food_energy: f32,
    /// Wert zwischen 0 (kein damping) und 1 (100% damping)
    pub velocity_damping: f32,
}

impl Default for ChunkSettings {
    fn default() -> Self {
        Self {
            spawn_chance: 0.018,
            spawned_food_energy: 200.,
            velocity_damping: 0.4,
        }
    }
}

#[derive(Default, Clone, Copy, Debug, Component, Reflect)]
#[reflect(Component)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Default, Debug, Component, Deref, DerefMut, Reflect)]
#[reflect(Component)]
pub struct Rotation(pub f32);

#[derive(Default, Debug, Component, Reflect)]
#[reflect(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

#[derive(Default, Debug, Component, Deref, DerefMut, Reflect)]
#[reflect(Component)]
pub struct Energy(pub f32);

#[derive(Default, Debug, Component, Deref, DerefMut, Reflect)]
#[reflect(Component)]
pub struct ChildCooldown(pub u32);

#[derive(Default, Debug, Component, Reflect)]
#[reflect(Component)]
pub struct CellStats {
    pub age: u32,
    pub child_count: u32,
}

#[derive(Default, Component, Reflect)]
#[reflect(Component)]
pub struct Cell;

#[derive(Default, Component, Reflect)]
#[reflect(Component)]
pub struct Food;

#[derive(Default, Component, Reflect)]
#[reflect(Component)]
pub struct Chunk;

#[derive(Default, Bundle)]
pub struct CellBundle {
    pub cell: Cell,
    pub brain: Brain,
    pub position: Position,
    pub rotation: Rotation,
    pub velocity: Velocity,
    pub energy: Energy,
    pub child_cooldown: ChildCooldown,
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

pub fn setup_chunks(mut commands: Commands) {
    // Neues Chunk-Registry erstellen
    let mut chunk_registry = ChunkRegistry {
        chunk_size: 50.,
        map_size: 40,
        entries: Vec::new(),
    };

    // Neue Chunks erzeugen und Referenz im Chunk-Registry speichern
    for index in 0..chunk_registry.map_size {
        chunk_registry.entries.push(Vec::new());
        for idy in 0..chunk_registry.map_size {
            let chunk_entity = commands
                .spawn(ChunkBundle {
                    position: Position {
                        x: (index as f32 * chunk_registry.chunk_size
                            + chunk_registry.chunk_size / 2.),
                        y: (idy as f32 * chunk_registry.chunk_size
                            + chunk_registry.chunk_size / 2.),
                    },
                    ..default()
                })
                .id();
            chunk_registry.entries[index as usize].push(chunk_entity);
        }
    }

    // Chunk-Registry zu Welt hinzufügen
    commands.spawn(chunk_registry);
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
            &mut ChildCooldown,
            &mut CellStats,
        ),
        (With<Cell>, Without<Food>, Without<Chunk>),
    >,
    mut food_query: Query<
        (Entity, &mut Position, &mut Energy),
        (With<Food>, Without<Cell>, Without<Chunk>),
    >,
    mut cell_count_statistic_query: Query<
        &mut StatisticData,
        (
            With<CellCountStatistic>,
            Without<ChildCountStatistic>,
            Without<BrainSizeStatistic>,
        ),
    >,
    mut child_count_statistic_query: Query<
        &mut StatisticData,
        (
            With<ChildCountStatistic>,
            Without<CellCountStatistic>,
            Without<BrainSizeStatistic>,
        ),
    >,
    mut brain_size_statistic_query: Query<
        &mut StatisticData,
        (
            With<BrainSizeStatistic>,
            Without<CellCountStatistic>,
            Without<ChildCountStatistic>,
        ),
    >,
    chunk_query: Query<(&Foodlist, &ChunkSettings), (With<Chunk>, Without<Cell>, Without<Food>)>,
    chunk_registry_query: Query<&ChunkRegistry>,
    simulation_settings: Res<SimulationSettings>,
) {
    let chunk_registry = chunk_registry_query.single();

    // Statistik Informationen deklarieren
    let mut cell_count = 0;
    let mut children_count_sum = 0;
    let mut cells_born = 0;
    let mut neuron_count_sum = 0;
    let mut connection_count_sum = 0;
    for (
        mut brain,
        mut position,
        mut rotation,
        mut velocity,
        mut energy,
        mut child_cooldown,
        mut stats,
    ) in &mut cell_query
    {
        let neuron_count = brain.neurons().len();
        let mut connection_count = 0;
        for neuron in brain.neurons() {
            connection_count += neuron.inputs.len();
        }

        // Statistik Informationen sammeln
        cell_count += 1;
        children_count_sum += stats.child_count;
        neuron_count_sum += neuron_count;
        connection_count_sum += connection_count;

        // Chunk berechnen
        let chunk_index = (position.x / chunk_registry.chunk_size) as i32;
        let chunk_idy = (position.y / chunk_registry.chunk_size) as i32;
        let chunk_settings = chunk_query
            .get_component::<ChunkSettings>(
                chunk_registry.entries[chunk_index as usize][chunk_idy as usize],
            )
            .unwrap();

        // Inputs berechnen und in Input-Neuronen schreiben
        let mut chunk_entities = Vec::with_capacity(9);
        for jdx in -1..1 {
            if chunk_index + jdx >= 0 && chunk_index + jdx < chunk_registry.map_size as i32 {
                for jdy in -1..1 {
                    if chunk_idy + jdy >= 0 && chunk_idy + jdy < chunk_registry.map_size as i32 {
                        chunk_entities.push(
                            chunk_registry.entries[(chunk_index + jdx) as usize]
                                [(chunk_idy + jdy) as usize],
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
        let nearest_food_relative_angle = nearest_food_angle - **rotation;
        brain.write_neuron(0, nearest_food_relative_angle);
        brain.write_neuron(1, nearest_food_distance_squared);
        brain.write_neuron(2, stats.age as f32);
        brain.write_neuron(3, **energy as f32);
        brain.write_neuron(4, (stats.age as f32 * 0.1).sin());

        // Brain rechnen lassen
        brain.tick();

        // Output-Neuronen auslesen
        let rotation_neuron_output = brain.read_neuron(5).unwrap();
        let acceleration_neuron_output = brain.read_neuron(6).unwrap();
        let want_child_neuron_output = brain.read_neuron(7).unwrap();

        // Rotieren und Geschwindigkeit passend verändern
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

        // Kinetische Energie berechnen und von Energie abziehen
        let kinetic_energy = velocity.x * velocity.x + velocity.y * velocity.y;
        let new_kinetic_energy = new_velocity.x * new_velocity.x + new_velocity.y * new_velocity.y;
        **energy -= (new_kinetic_energy - kinetic_energy).abs();

        // Geschwindikeit und Position berechen
        *velocity = new_velocity;
        position.x += velocity.x;
        position.y += velocity.y;
        if position.x < 0.
            || position.y < 0.
            || position.x >= (chunk_registry.map_size as f32 * chunk_registry.chunk_size) as f32
            || position.y >= (chunk_registry.map_size as f32 * chunk_registry.chunk_size) as f32
        {
            **energy = 0.;
            continue;
        }
        velocity.x *= 1. - chunk_settings.velocity_damping;
        velocity.y *= 1. - chunk_settings.velocity_damping;

        // Essen einsammeln

        // Benötigte Distanz berechen (squared um sqrt(x) zu vermeiden)
        let distance_min_squared = (simulation_settings.cell_radius
            + simulation_settings.food_radius)
            * (simulation_settings.cell_radius + simulation_settings.food_radius);

        // Tatsächliche Kollisionen berechnen
        let mut chunk_entities = Vec::with_capacity(9);
        for jdx in -1..1 {
            if chunk_index + jdx >= 0 && chunk_index + jdx < chunk_registry.map_size as i32 {
                for jdy in -1..1 {
                    if chunk_idy + jdy >= 0 && chunk_idy + jdy < chunk_registry.map_size as i32 {
                        chunk_entities.push(
                            chunk_registry.entries[(chunk_index + jdx) as usize]
                                [(chunk_idy + jdy) as usize],
                        );
                    }
                }
            }
        }
        for (foodlist, _) in chunk_query.iter_many(chunk_entities) {
            let mut food_query_iter = food_query.iter_many_mut(&**foodlist);
            while let Some((_, food_position, mut food_energy)) = food_query_iter.fetch_next() {
                let relative_position = Position {
                    x: food_position.x - position.x,
                    y: food_position.y - position.y,
                };
                let distance_squared = relative_position.x * relative_position.x
                    + relative_position.y * relative_position.y;
                if distance_squared < distance_min_squared {
                    // Essen leersaugen
                    **energy += **food_energy;
                    **food_energy = 0.;
                }
            }
        }

        // Kind erzeugen
        if want_child_neuron_output.is_sign_positive()
            && **energy > simulation_settings.energy_required_for_split
            && **child_cooldown == 0
        {
            // Stats aktualisieren
            cells_born += 1;
            stats.child_count += 1;

            **child_cooldown = simulation_settings.child_cooldown;

            // Child-Brain erstellen
            let mut child_brain = brain.clone();
            child_brain.mutate();

            // Neuen Energiewerte berechnen
            let new_energy = **energy / 2.;
            **energy = new_energy;

            // Kind in Welt spawnen
            commands.spawn(CellBundle {
                position: Position {
                    x: position.x,
                    y: position.y,
                },
                rotation: Rotation(**rotation),
                energy: Energy(new_energy),
                brain: child_brain,
                child_cooldown: ChildCooldown(simulation_settings.child_cooldown),
                ..default()
            });
        }
        **energy -= simulation_settings.base_energy_drain
            + neuron_count as f32 * simulation_settings.neuron_energy_drain
            + connection_count as f32 * simulation_settings.connection_energy_drain
            + stats.age as f32 * simulation_settings.age_energy_drain;
        stats.age += 1;
        if **child_cooldown > 0 {
            **child_cooldown -= 1;
        }
    }

    // Statistiken schreiben
    cell_count_statistic_query.single_mut().lines[0]
        .data_points
        .push(cell_count as f32);
    cell_count_statistic_query.single_mut().lines[1]
        .data_points
        .push(cells_born as f32);
    if cell_count > 0 {
        child_count_statistic_query.single_mut().lines[0]
            .data_points
            .push(children_count_sum as f32 / cell_count as f32);
        brain_size_statistic_query.single_mut().lines[0]
            .data_points
            .push(neuron_count_sum as f32 / cell_count as f32);
        brain_size_statistic_query.single_mut().lines[1]
            .data_points
            .push(connection_count_sum as f32 / cell_count as f32);
        brain_size_statistic_query.single_mut().lines[2]
            .data_points
            .push(
                (connection_count_sum as f32 / cell_count as f32)
                    / (neuron_count_sum as f32 / cell_count as f32),
            );
    }
}

pub fn spawn_food(
    mut commands: Commands,
    mut chunk_query: Query<(&mut Foodlist, &ChunkSettings, &Position), With<Chunk>>,
    chunk_registry_query: Query<&ChunkRegistry>,
) {
    let chunk_registry = chunk_registry_query.single();

    for (mut foodlist, chunk_settings, chunk_position) in &mut chunk_query {
        let mut to_place = chunk_settings.spawn_chance;
        while to_place > 1. {
            let food_entity = commands
                .spawn(FoodBundle {
                    position: Position {
                        x: chunk_position.x + (random::<f32>() - 0.5) * chunk_registry.chunk_size,
                        y: chunk_position.y + (random::<f32>() - 0.5) * chunk_registry.chunk_size,
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
                        x: chunk_position.x + (random::<f32>() - 0.5) * chunk_registry.chunk_size,
                        y: chunk_position.y + (random::<f32>() - 0.5) * chunk_registry.chunk_size,
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
    // Essen ohne Energie löschen
    for (entity, energy) in &food_query {
        if **energy <= 0. {
            commands.entity(entity).despawn();
        }
    }
}

pub fn despawn_cells(
    mut commands: Commands,
    mut cell_count_statistic_query: Query<&mut StatisticData, With<CellCountStatistic>>,
    cell_query: Query<(Entity, &Energy), With<Cell>>,
) {
    // Zellen ohne Energie löschen
    let mut cells_died = 0;
    for (entity, energy) in &cell_query {
        if **energy <= 0. {
            cells_died += 1;
            commands.entity(entity).despawn();
        }
    }

    // Statistiken schreiben
    cell_count_statistic_query.single_mut().lines[2]
        .data_points
        .push(cells_died as f32);
}

pub fn run_on_tick(
    mut tick_watch: Local<Stopwatch>,
    mut control_center_ui: ResMut<ControlCenterUi>,
    simulation_settings: Res<SimulationSettings>,
    time: Res<Time>,
) -> ShouldRun {
    // Prüfen ob ein Tick aussteht
    if !simulation_settings.is_paused
        && tick_watch.tick(time.delta()).elapsed_secs() >= simulation_settings.tick_delta_seconds
    {
        control_center_ui.actual_tick_delta_seconds_label =
            format!("{:.3}", tick_watch.elapsed_secs());
        tick_watch.reset();
        // Tick in Auftrag geben
        ShouldRun::Yes
    } else {
        // Keinen Tick in Auftrag geben
        ShouldRun::No
    }
}

pub struct SpawnCell {
    pub energy: f32,
}

/// Event-Handler für `SpawnCell` events
pub fn spawn_cells(
    mut commands: Commands,
    mut spawn_cell_events: EventReader<SpawnCell>,
    chunk_registry_query: Query<&ChunkRegistry>,
) {
    for spawn_cell_event in spawn_cell_events.iter() {
        let chunk_registry = chunk_registry_query.single();
        let mut brain = Brain::new();
        brain.mutate();
        commands.spawn(CellBundle {
            position: Position {
                x: random::<f32>() * chunk_registry.map_size as f32 * chunk_registry.chunk_size,
                y: random::<f32>() * chunk_registry.map_size as f32 * chunk_registry.chunk_size,
            },
            rotation: Rotation(random::<f32>() * 2. * PI),
            energy: Energy(spawn_cell_event.energy),
            brain,
            ..default()
        });
    }
}

pub struct Clear {
    pub clear_food: bool,
    pub clear_cells: bool,
    pub clear_statistics: bool,
}

/// Event-Handler für `Clear` Event
pub fn clear(
    mut commands: Commands,
    mut clear_events: EventReader<Clear>,
    mut statistic_query: Query<&mut StatisticData, With<Statistic>>,
    food_query: Query<Entity, With<Food>>,
    cell_query: Query<Entity, With<Cell>>,
) {
    for clear_event in clear_events.iter() {
        if clear_event.clear_food {
            for entity in &food_query {
                commands.entity(entity).despawn();
            }
        }
        if clear_event.clear_cells {
            for entity in &cell_query {
                commands.entity(entity).despawn();
            }
        }
        if clear_event.clear_statistics {
            for mut statistic_data in &mut statistic_query {
                for mut statistic_line in &mut statistic_data.lines {
                    statistic_line.data_points = Vec::new();
                }
            }
        }
    }
}

pub struct ApplyChunkSettings;

/// Event-Handler für `ApplyChunkSettings` Event
pub fn apply_chunk_settings(
    mut chunk_query: Query<(&mut ChunkSettings, &Position), With<Chunk>>,
    mut apply_chunk_settings_events: EventReader<ApplyChunkSettings>,
    chunk_registry_query: Query<&ChunkRegistry>,
    control_center_ui: Res<ControlCenterUi>,
) {
    let spawn_chance_left = control_center_ui.food_spawn_chance_slider_left;
    let spawn_chance_right = control_center_ui.food_spawn_chance_slider_right;
    let velocity_damping_bottom = control_center_ui.velocity_damping_slider_bottom;
    let velocity_damping_top = control_center_ui.velocity_damping_slider_top;
    for _ in apply_chunk_settings_events.iter() {
        let chunk_registry = chunk_registry_query.single();
        for (mut chunk_settings, chunk_position) in &mut chunk_query {
            let chunk_index = (chunk_position.x / chunk_registry.chunk_size) as f32;
            let chunk_idy = (chunk_position.y / chunk_registry.chunk_size) as f32;
            *chunk_settings = ChunkSettings {
                spawn_chance: spawn_chance_left
                    + (spawn_chance_right - spawn_chance_left) * chunk_index
                        / chunk_registry.map_size as f32,
                spawned_food_energy: control_center_ui.food_energy_drag_value,
                velocity_damping: velocity_damping_bottom
                    + (velocity_damping_top - velocity_damping_bottom) * chunk_idy
                        / chunk_registry.map_size as f32,
            };
        }
    }
}

pub struct ApplySimulationSettings;

/// Event-Handler für `ApplySimulationSettings` Event
pub fn apply_simulation_settings(
    mut simulation_settings: ResMut<SimulationSettings>,
    mut apply_simulation_settings_events: EventReader<ApplySimulationSettings>,
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
            age_energy_drain: control_center_ui.age_energy_drain_drag_value,
            energy_required_for_split: control_center_ui.energy_required_for_split_drag_value,
            child_cooldown: control_center_ui.child_cooldown_drag_value,
            rotation_speed_max: control_center_ui.rotation_speed_max_drag_value,
            acceleration_max: control_center_ui.acceleration_max_drag_value,
            is_paused: simulation_settings.is_paused,
        };
    }
}

pub struct TogglePause;

/// Event-Handler für `TogglePause` Event
pub fn toggle_pause(
    mut simulation_settings: ResMut<SimulationSettings>,
    mut toggle_pause_events: EventReader<TogglePause>,
) {
    for _ in toggle_pause_events.iter() {
        simulation_settings.is_paused = !simulation_settings.is_paused;
    }
}

pub struct Save {
    pub save_name: String,
}

/// Event-Handler für `Save` Event
pub fn save(
    mut save_events: EventReader<Save>,
    world: &World,
    simulation_settings: Res<SimulationSettings>,
) {
    for save_event in save_events.iter() {
        // Ordner erstellen
        fs::create_dir_all(Path::new(&format!("assets/{}", &save_event.save_name))).unwrap();

        // Scene speichern
        let type_registry = world.resource::<AppTypeRegistry>();
        let scene = DynamicScene::from_world(world, type_registry);
        let serialized_scene = scene.serialize_ron(type_registry).unwrap();
        fs::write(
            Path::new(&format!("assets/{}/scene.scn.ron", &save_event.save_name)),
            serialized_scene,
        )
        .unwrap();

        // Simulation-Settings speichern
        let serialized_simulation_settings = scene::serialize_ron(&*simulation_settings).unwrap();
        fs::write(
            Path::new(&format!(
                "assets/{}/simulation_settings.ron",
                &save_event.save_name
            )),
            serialized_simulation_settings,
        )
        .unwrap();
    }
}

pub struct Load {
    pub save_name: String,
}

/// Event-Handler für `Load` Event
pub fn load(
    mut commands: Commands,
    mut load_events: EventReader<Load>,
    mut simulation_settings: ResMut<SimulationSettings>,
    entity_query: Query<Entity>,
    asset_server: Res<AssetServer>,
) {
    for load_event in load_events.iter() {
        // Alle bestehenden Entities despawnen
        for entity in &entity_query {
            commands.entity(entity).despawn();
        }

        // Scene laden
        commands.spawn(DynamicSceneBundle {
            scene: asset_server.load(Path::new(&format!(
                "{}/scene.scn.ron",
                &load_event.save_name
            ))),
            ..default()
        });

        // Simulation-Settings laden
        let serialized_simulation_settings = fs::read_to_string(Path::new(&format!(
            "assets/{}/simulation_settings.ron",
            &load_event.save_name
        )))
        .unwrap();
        *simulation_settings = ron::from_str(&serialized_simulation_settings).unwrap();
        // Simulation pausieren um zu verhindern, dass in diesem Frame noch ein Tick durchgeführt wird
        simulation_settings.is_paused = true;
    }
}
