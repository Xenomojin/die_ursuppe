use bevy::prelude::*;
use bevy_egui::{
    egui::{
        self,
        plot::{Line, Plot, PlotPoints, Points},
        Rgba,
    },
    EguiContext,
};

use crate::{
    brain::Brain,
    sim::{Age, Energy, Position, Rotation},
};

pub fn ui(mut commands: Commands, mut egui_context: ResMut<EguiContext>) {
    egui::Window::new("Simulation")
        .resizable(true)
        .show(egui_context.ctx_mut(), |ui| {
            ui.heading("Die Ursuppe");
            if ui.button("Run").clicked() {
                println!("Starte Simulation...");
                let mut brain = Brain::new();
                for _ in 0..10 {
                    brain.mutate();
                }
                let cell_bundle = (
                    Position { x: 5., y: 3.6 },
                    Rotation(0.),
                    Age(0),
                    Energy(5.),
                    brain,
                );
                println!("spawned {:?}", cell_bundle);
                commands.spawn_bundle(cell_bundle);
            }

            let points = Points::new(PlotPoints::new(vec![[1., 4.], [0.5, 0.2], [-0.4, -2.1]]))
                .radius(5.)
                .color(Rgba::BLUE);
            Plot::new("sim")
                .data_aspect(1.)
                .view_aspect(1.)
                .show(ui, |plot_ui| plot_ui.points(points));
        });
}
