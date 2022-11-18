use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use sim::{
    ApplyChunkSettings, ApplySimulationSettings, ChunkList, Clear, SimulationSettings, SpawnCell,
    TogglePause,
};
use ui::ControlCenterUi;

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
        .add_event::<SpawnCell>()
        .add_event::<Clear>()
        .add_event::<ApplySimulationSettings>()
        .add_event::<TogglePause>()
        .add_event::<ApplyChunkSettings>()
        .init_resource::<SimulationSettings>()
        .init_resource::<ChunkList>()
        .init_resource::<ControlCenterUi>()
        .add_startup_system(sim::setup)
        .add_system(ui::display_control_center_ui)
        .add_system(ui::display_simulation_ui)
        .add_system(sim::spawn_cells)
        .add_system(sim::apply_chunk_settings)
        .add_system(sim::apply_simulation_settings)
        .add_system(sim::toggle_pause)
        .add_system(sim::clear)
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
