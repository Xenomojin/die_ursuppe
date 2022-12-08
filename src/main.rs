use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use brain::{Brain, Neuron, NeuronInput};
use sim::{
    ApplyChunkSettings, ApplySimulationSettings, Cell, CellStats, Chunk, ChunkList, ChunkSettings,
    Clear, Energy, Food, Foodlist, Position, Rotation, Save, SimulationSettings, SpawnCell, Tick,
    TogglePause, Velocity,
};
use ui::{BrainSizeStatisticUi, CellCountStatisticUi, ChildCountStatisticUi, ControlCenterUi};

mod brain;
mod sim;
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
        // Register components
        .register_type::<Foodlist>()
        .register_type::<ChunkSettings>()
        .register_type::<Position>()
        .register_type::<Rotation>()
        .register_type::<Velocity>()
        .register_type::<Energy>()
        .register_type::<CellStats>()
        .register_type::<Brain>()
        .register_type::<Neuron>().register_type::<NeuronInput>()
        .register_type::<Cell>()
        .register_type::<Food>()
        .register_type::<Chunk>()
        // Simulation ressources
        .init_resource::<Tick>()
        .init_resource::<SimulationSettings>()
        .init_resource::<ChunkList>()
        // Ui state ressources
        .init_resource::<ControlCenterUi>()
        .init_resource::<ChildCountStatisticUi>()
        .init_resource::<CellCountStatisticUi>()
        .init_resource::<BrainSizeStatisticUi>()
        // Setup
        .add_startup_system(sim::setup)
        // Ui zeichnen
        .add_system(ui::display_simulation_ui)
        .add_system(ui::display_control_center_ui)
        .add_system(ui::display_child_count_statistic_ui)
        .add_system(ui::display_cell_count_statistic_ui)
        .add_system(ui::display_brain_size_statistic_ui)
        // Ui Event-Handler
        .add_system(sim::spawn_cells)
        .add_system(sim::apply_chunk_settings)
        .add_system(sim::apply_simulation_settings)
        .add_system(sim::toggle_pause)
        .add_system(sim::clear)
        .add_system(sim::save)
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
