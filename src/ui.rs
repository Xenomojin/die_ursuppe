use bevy::prelude::*;
use bevy_egui::{
    egui::{
        plot::{Plot, PlotPoints, Points},
        DragValue, Grid, Rgba, Slider, Window,
    },
    EguiContext,
};

use crate::sim::{Cell, Clear, Food, Position, SpawnCell, SpawnFood};

pub struct UiState {
    cell_energy_drag_value: f32,
    cell_amount_slider_value: u32,
    food_energy_drag_value: f32,
    food_amount_slider_value: u32,
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            cell_energy_drag_value: 100.,
            cell_amount_slider_value: 20,
            food_energy_drag_value: 200.,
            food_amount_slider_value: 100,
        }
    }
}

pub fn display_ui(
    mut egui_context: ResMut<EguiContext>,
    mut ui_state: Local<UiState>,
    mut spawn_cell_events: EventWriter<SpawnCell>,
    mut spawn_food_events: EventWriter<SpawnFood>,
    mut clear_events: EventWriter<Clear>,
    cell_query: Query<&Position, With<Cell>>,
    food_query: Query<&Position, With<Food>>,
) {
    Window::new("Control Center")
        .resizable(true)
        .show(egui_context.ctx_mut(), |ui| {
            ui.heading("Spawn Cells");
            Grid::new("spawn_cells_grid").show(ui, |grid_ui| {
                grid_ui.label("Energy: ");
                grid_ui.add(DragValue::new(&mut ui_state.cell_energy_drag_value));
                grid_ui.end_row();
                grid_ui.label("Amount: ");
                grid_ui.add(Slider::new(&mut ui_state.cell_amount_slider_value, 0..=500));
                grid_ui.end_row();
            });
            if ui.button("Spawn Cells").clicked() {
                for _ in 0..(ui_state.cell_amount_slider_value as usize) {
                    spawn_cell_events.send(SpawnCell {
                        energy: ui_state.cell_energy_drag_value,
                    });
                }
            }
            ui.separator();
            ui.heading("Spawn Food");
            Grid::new("spawn_food_grid").show(ui, |grid_ui| {
                grid_ui.label("Energy: ");
                grid_ui.add(DragValue::new(&mut ui_state.food_energy_drag_value));
                grid_ui.end_row();
                grid_ui.label("Amount: ");
                grid_ui.add(Slider::new(&mut ui_state.food_amount_slider_value, 0..=500));
                grid_ui.end_row();
            });
            if ui.button("Spawn Food").clicked() {
                for _ in 0..(ui_state.food_amount_slider_value as usize) {
                    spawn_food_events.send(SpawnFood {
                        energy: ui_state.food_energy_drag_value,
                    });
                }
            }
            ui.separator();
            ui.heading("Destroy");
            if ui.button("Clear").clicked() {
                clear_events.send(Clear);
            }
        });
    Window::new("Simulation")
        .resizable(true)
        .show(egui_context.ctx_mut(), |ui| {
            Plot::new("sim")
                .data_aspect(1.)
                .view_aspect(1.)
                .legend(default())
                .show(ui, |plot_ui| {
                    let mut cell_points = Vec::new();
                    for position in &cell_query {
                        cell_points.push([position.x as f64, position.y as f64]);
                    }
                    plot_ui.points(
                        Points::new(PlotPoints::new(cell_points))
                            .radius(5.)
                            .color(Rgba::RED)
                            .name("cell"),
                    );
                    let mut food_points = Vec::new();
                    for position in &food_query {
                        food_points.push([position.x as f64, position.y as f64]);
                    }
                    plot_ui.points(
                        Points::new(PlotPoints::new(food_points))
                            .radius(3.)
                            .color(Rgba::GREEN)
                            .name("food"),
                    );
                });
        });
}
