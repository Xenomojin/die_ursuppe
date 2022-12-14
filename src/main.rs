use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use brain::{Brain, Neuron, NeuronInput};
use sim::{
    ApplyChunkSettings, ApplySimulationSettings, Cell, CellStats, ChildCooldown, Chunk,
    ChunkRegistry, ChunkSettings, Clear, Energy, Food, Foodlist, Load, Position, Rotation, Save,
    SimulationSettings, SpawnCell, TogglePause, Velocity,
};
use ui::{
    BrainSizeStatistic, CellCountStatistic, CellInspectorUi, ChildCountStatistic, ControlCenterUi,
    IsOpen, Label, Statistic, StatisticData, StatisticLine,
};

mod brain;
mod sim;
mod tests;
mod ui;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Die Ursuppe".to_string(),
                ..default()
            },
            ..default()
        }))
        .add_plugin(EguiPlugin)
        // Ui events
        .add_event::<SpawnCell>()
        .add_event::<Clear>()
        .add_event::<ApplySimulationSettings>()
        .add_event::<TogglePause>()
        .add_event::<ApplyChunkSettings>()
        .add_event::<Save>()
        .add_event::<Load>()
        // Register components
        .register_type::<[f32; 3]>()
        .register_type::<Vec<f32>>()
        .register_type::<Vec<Vec<Entity>>>()
        .register_type::<ChunkRegistry>()
        .register_type::<Foodlist>()
        .register_type::<ChunkSettings>()
        .register_type::<Position>()
        .register_type::<Rotation>()
        .register_type::<Velocity>()
        .register_type::<Energy>()
        .register_type::<ChildCooldown>()
        .register_type::<CellStats>()
        .register_type::<Brain>()
        .register_type::<Vec<Neuron>>()
        .register_type::<Neuron>()
        .register_type::<Vec<NeuronInput>>()
        .register_type::<NeuronInput>()
        .register_type::<Label>()
        .register_type::<IsOpen>()
        .register_type::<StatisticData>()
        .register_type::<Vec<StatisticLine>>()
        .register_type::<StatisticLine>()
        .register_type::<Cell>()
        .register_type::<Food>()
        .register_type::<Chunk>()
        .register_type::<Statistic>()
        .register_type::<ChildCountStatistic>()
        .register_type::<CellCountStatistic>()
        .register_type::<BrainSizeStatistic>()
        // Init ressources
        .init_resource::<SimulationSettings>()
        .init_resource::<ControlCenterUi>()
        .init_resource::<CellInspectorUi>()
        // Setup
        .add_startup_system(sim::setup_chunks)
        .add_startup_system(ui::setup_statistics)
        // Ui zeichnen
        .add_system(ui::display_simulation)
        .add_system(ui::display_control_center)
        .add_system(ui::display_statistics)
        .add_system(ui::display_cell_inspector)
        // Ui Event-Handler
        .add_system(sim::spawn_cells)
        .add_system(sim::apply_chunk_settings)
        .add_system(sim::apply_simulation_settings)
        .add_system(sim::toggle_pause)
        .add_system(sim::clear)
        .add_system(sim::save)
        .add_system(sim::load)
        // Simulation Systeme, die an Tick beteiligt sind
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(sim::run_on_tick)
                .with_system(sim::spawn_food)
                .with_system(sim::tick_cells)
                .with_system(sim::despawn_food.after(sim::tick_cells))
                .with_system(sim::despawn_cells.after(sim::tick_cells)),
        )
        .run();
}
