use bevy::prelude::*;
use bevy_egui::{
    egui::{
        plot::{Plot, PlotPoints, Points},
        DragValue, Grid, Rgba, Slider, Window,
    },
    EguiContext,
};

use crate::sim::{Cell, Clear, Food, Position, SpawnCell, SpawnFood};

#[derive(Resource)]
pub struct ControlCenterUi {
    pub rotation_speed_max_drag_value: f32,
    pub acceleration_max_drag_value: f32,
    /// Wert zwischen 0 (kein damping) und 1 (100% damping)
    pub velocity_damping_slider: f32,
    pub base_energy_drain_drag_value: f32,
    pub cell_radius_drag_value: f32,
    pub food_radius_drag_value: f32,
    pub autospawn_food_checkbox: bool,
    /// Start Energy-Wert für zukünftige manuell gespawnte cells
    pub cell_energy_drag_value: f32,
    pub cell_amount_slider: u32,
    /// Energy-Wert für zukünftiges food
    pub food_energy_drag_value: f32,
    pub food_amount_slider: u32,
}

impl Default for ControlCenterUi {
    fn default() -> Self {
        Self {
            rotation_speed_max_drag_value: 1.,
            acceleration_max_drag_value: 2.,
            velocity_damping_slider: 0.5,
            base_energy_drain_drag_value: 0.8,
            cell_radius_drag_value: 5.,
            food_radius_drag_value: 3.,
            autospawn_food_checkbox: false,
            cell_energy_drag_value: 199.,
            cell_amount_slider: 50,
            food_energy_drag_value: 200.,
            food_amount_slider: 5,
        }
    }
}

pub fn display_control_center_ui(
    mut egui_context: ResMut<EguiContext>,
    mut control_center_ui: ResMut<ControlCenterUi>,
    mut spawn_cell_events: EventWriter<SpawnCell>,
    mut spawn_food_events: EventWriter<SpawnFood>,
    mut clear_events: EventWriter<Clear>,
) {
    Window::new("Control Center")
        .resizable(true)
        .show(egui_context.ctx_mut(), |ui| {
            ui.heading("Cell settings");
            Grid::new("cell_settings_grid").show(ui, |grid_ui| {
                grid_ui.label("Rotation speed max.: ");
                grid_ui.add(DragValue::new(
                    &mut control_center_ui.rotation_speed_max_drag_value,
                ).speed(0.01));
                grid_ui.end_row();
                grid_ui.label("Acceleration max.: ");
                grid_ui.add(DragValue::new(
                    &mut control_center_ui.acceleration_max_drag_value,
                ).speed(0.01));
                grid_ui.end_row();
                grid_ui.label("Velocity damping: ");
                grid_ui.add(Slider::new(
                    &mut control_center_ui.velocity_damping_slider,
                    0.0..=1.0,
                ));
                grid_ui.end_row();
                grid_ui.label("Base energy drain: ");
                grid_ui.add(DragValue::new(
                    &mut control_center_ui.base_energy_drain_drag_value,
                ).speed(0.01));
                grid_ui.end_row();
                grid_ui.label("Cell raidus: ");
                grid_ui.add(DragValue::new(
                    &mut control_center_ui.cell_radius_drag_value,
                ).speed(0.01));
                grid_ui.end_row();
            });
            ui.separator();
            ui.heading("Spawn cells");
            Grid::new("spawn_cells_grid").show(ui, |grid_ui| {
                grid_ui.label("Energy: ");
                grid_ui.add(DragValue::new(
                    &mut control_center_ui.cell_energy_drag_value,
                ));
                grid_ui.end_row();
                grid_ui.label("Amount: ");
                grid_ui.add(Slider::new(
                    &mut control_center_ui.cell_amount_slider,
                    0..=500,
                ));
                grid_ui.end_row();
                if grid_ui.button("Spawn Cells").clicked() {
                    for _ in 0..(control_center_ui.cell_amount_slider as usize) {
                        spawn_cell_events.send(SpawnCell {
                            energy: control_center_ui.cell_energy_drag_value,
                        });
                    }
                }
                grid_ui.end_row();
            });
            ui.separator();
            ui.heading("Food settings");
            Grid::new("food_settings_grid").show(ui, |grid_ui| {
                grid_ui.label("Food radius: ");
                grid_ui.add(DragValue::new(
                    &mut control_center_ui.food_radius_drag_value,
                ).speed(0.01));
                grid_ui.end_row();
            });
            ui.separator();
            ui.heading("Spawn food");
            Grid::new("spawn_food_grid").show(ui, |grid_ui| {
                grid_ui.label("Energy: ");
                grid_ui.add(DragValue::new(
                    &mut control_center_ui.food_energy_drag_value,
                ));
                grid_ui.end_row();
                grid_ui.label("Amount: ");
                grid_ui.add(Slider::new(
                    &mut control_center_ui.food_amount_slider,
                    0..=150,
                ));
                grid_ui.end_row();
                if grid_ui.button("Spawn Food").clicked() {
                    for _ in 0..(control_center_ui.food_amount_slider as usize) {
                        spawn_food_events.send(SpawnFood {
                            energy: control_center_ui.food_energy_drag_value,
                        });
                    }
                }
                grid_ui.checkbox(&mut control_center_ui.autospawn_food_checkbox, "Autospawn");
                grid_ui.end_row();
            });
            ui.separator();
            ui.heading("Destroy");
            if ui.button("Clear").clicked() {
                clear_events.send(Clear);
            }
        });
}

pub fn display_simulation_ui(
    mut egui_context: ResMut<EguiContext>,
    control_center_ui: Res<ControlCenterUi>,
    cell_query: Query<&Position, With<Cell>>,
    food_query: Query<&Position, With<Food>>,
) {
    Window::new("Simulation")
        .resizable(true)
        .show(egui_context.ctx_mut(), |ui| {
            Plot::new("sim")
                .data_aspect(1.)
                .view_aspect(1.)
                .legend(default())
                .show(ui, |plot_ui| {
                    let mut food_points = Vec::new();
                    for position in &food_query {
                        food_points.push([position.x as f64, position.y as f64]);
                    }
                    plot_ui.points(
                        Points::new(PlotPoints::new(food_points))
                            .radius(control_center_ui.food_radius_drag_value)
                            .color(Rgba::GREEN)
                            .name("food"),
                    );
                    let mut cell_points = Vec::new();
                    for position in &cell_query {
                        cell_points.push([position.x as f64, position.y as f64]);
                    }
                    plot_ui.points(
                            Points::new(PlotPoints::new(cell_points))
                            .radius(control_center_ui.cell_radius_drag_value)
                            .color(Rgba::RED)
                            .name("cell"),
                    );
                });
        });
}
