use bevy::prelude::*;
use bevy_egui::EguiPlugin;

mod brain;
mod sim;
mod ui;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Die Ursuppe".to_string(),
            ..bevy::prelude::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_system(ui::ui)
        .add_system(sim::tick)
        .run();
}
