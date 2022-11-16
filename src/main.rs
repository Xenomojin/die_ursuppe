use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use sim::{ApplyChunkSettings, ChunkList, Clear, SpawnCell};
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
        .add_event::<ApplyChunkSettings>()
        .init_resource::<ControlCenterUi>()
        .init_resource::<ChunkList>()
        .add_startup_system(sim::setup)
        .add_system(ui::display_control_center_ui)
        .add_system(ui::display_simulation_ui)
        .add_system(sim::tick)
        .add_system(sim::clear)
        .add_system(sim::spawn_cells)
        .add_system(sim::apply_chunk_settings)
        .run();
}
