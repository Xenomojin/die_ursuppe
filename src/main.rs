use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use sim::{SpawnCell, SpawnFood, Clear};

mod brain;
mod sim;
mod ui;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Die Ursuppe".to_string(),
            ..default()
        })
        .add_event::<Clear>()
        .add_event::<SpawnCell>()
        .add_event::<SpawnFood>()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_system(ui::display_ui)
        .add_system(sim::tick)
        .add_system(sim::clear)
        .add_system(sim::spawn_cells)
        .add_system(sim::spawn_food)
        .run();
}
