use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use sim::{
    ApplyChunkSettings, ApplySimulationSettings, ChunkList, Clear, SimulationSettings, SpawnCell,
    Tick, TogglePause,
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
        // Ui Events
        .add_event::<SpawnCell>()
        .add_event::<Clear>()
        .add_event::<ApplySimulationSettings>()
        .add_event::<TogglePause>()
        .add_event::<ApplyChunkSettings>()
        // Simulation Ressources
        .init_resource::<Tick>()
        .init_resource::<SimulationSettings>()
        .init_resource::<ChunkList>()
        // Ui State Ressources
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
