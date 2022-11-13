use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use sim::{Clear, SpawnCell, SpawnFood};
use ui::ControlCenterUi;

mod brain;
mod sim;
mod ui;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Die Ursuppe".to_string(),
            ..default()
        })
        .init_resource::<ControlCenterUi>()
        .add_event::<Clear>()
        .add_event::<SpawnCell>()
        .add_event::<SpawnFood>()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_system(ui::display_control_center_ui)
        .add_system(ui::display_simulation_ui)
        .add_system(sim::tick)
        .add_system(sim::clear)
        .add_system(sim::spawn_cells)
        .add_system(sim::spawn_food)
        .run();
}
