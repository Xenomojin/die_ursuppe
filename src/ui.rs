use bevy::prelude::*;
use bevy_egui::{
    egui::{
        plot::{Plot, PlotPoints, Points},
        DragValue, Grid, Rgba, Slider, Window,
    },
    EguiContext,
};

use crate::sim::{
    ApplyChunkSettings, ApplySimulationSettings, Cell, Clear, Food, Position, SimulationSettings,
    SpawnCell,
};

#[derive(Resource)]
pub struct ControlCenterUi {
    pub cell_radius_drag_value: f32,
    pub food_radius_drag_value: f32,
    pub tick_delta_seconds_slider: f32,
    pub actual_tick_delta_seconds_label: String,
    pub rotation_speed_max_drag_value: f32,
    pub acceleration_max_drag_value: f32,
    /// Wert zwischen 0 (kein damping) und 1 (100% damping)
    pub velocity_damping_slider_top: f32,
    pub velocity_damping_slider_bottom: f32,
    pub base_energy_drain_drag_value: f32,
    pub autospawn_food_checkbox: bool,
    /// Start Energy-Wert für zukünftige manuell gespawnte cells
    pub cell_energy_drag_value: f32,
    pub cell_amount_slider: u32,
    /// Energy-Wert für zukünftiges food
    pub food_energy_drag_value: f32,
    pub food_spawn_chance_slider_left: f32,
    pub food_spawn_chance_slider_right: f32,
}

impl Default for ControlCenterUi {
    fn default() -> Self {
        Self {
            cell_radius_drag_value: 5.,
            food_radius_drag_value: 3.,
            tick_delta_seconds_slider: 0.02,
            actual_tick_delta_seconds_label: "-".to_string(),
            rotation_speed_max_drag_value: 1.,
            acceleration_max_drag_value: 2.,
            velocity_damping_slider_bottom: 0.5,
            velocity_damping_slider_top: 0.5,
            base_energy_drain_drag_value: 0.8,
            autospawn_food_checkbox: false,
            cell_energy_drag_value: 199.,
            cell_amount_slider: 50,
            food_energy_drag_value: 200.,
            food_spawn_chance_slider_left: 0.,
            food_spawn_chance_slider_right: 0.,
        }
    }
}

pub fn display_control_center_ui(
    mut egui_context: ResMut<EguiContext>,
    mut control_center_ui: ResMut<ControlCenterUi>,
    mut spawn_cell_events: EventWriter<SpawnCell>,
    mut apply_chunk_settings_events: EventWriter<ApplyChunkSettings>,
    mut apply_simulation_settings_events: EventWriter<ApplySimulationSettings>,
    mut clear_events: EventWriter<Clear>,
) {
    Window::new("Control Center")
        .resizable(true)
        .show(egui_context.ctx_mut(), |ui| {
            ui.heading("Simulation Settings");
            Grid::new("simulation_settings_grid").show(ui, |grid_ui| {
                grid_ui.colored_label(Rgba::GREEN, "- Time -");
                grid_ui.end_row();
                grid_ui.label("Tick Delta Seconds: ");
                grid_ui.add(Slider::new(
                    &mut control_center_ui.tick_delta_seconds_slider,
                    0.0..=0.2,
                ));
                grid_ui.end_row();
                grid_ui.label("Actual Delta Seconds: ");
                grid_ui.colored_label(
                    Rgba::WHITE,
                    &control_center_ui.actual_tick_delta_seconds_label,
                );
                grid_ui.end_row();
                grid_ui.colored_label(Rgba::GREEN, "- Miscellaneous -");
                grid_ui.end_row();
                grid_ui.label("Cell Radius: ");
                grid_ui
                    .add(DragValue::new(&mut control_center_ui.cell_radius_drag_value).speed(0.01));
                grid_ui.end_row();
                grid_ui.label("Food Radius: ");
                grid_ui
                    .add(DragValue::new(&mut control_center_ui.food_radius_drag_value).speed(0.01));
                grid_ui.end_row();
                if grid_ui.button("Apply").clicked() {
                    apply_simulation_settings_events.send(ApplySimulationSettings);
                }
                grid_ui.end_row();
            });
            ui.separator();
            ui.heading("Spawn Cells");
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
                if grid_ui.button("Spawn").clicked() {
                    for _ in 0..(control_center_ui.cell_amount_slider as usize) {
                        spawn_cell_events.send(SpawnCell {
                            energy: control_center_ui.cell_energy_drag_value,
                        });
                    }
                }
                grid_ui.end_row();
            });
            ui.separator();
            ui.heading("Chunk Settings");
            Grid::new("chunk_settings_grid").show(ui, |grid_ui| {
                grid_ui.colored_label(Rgba::GREEN, "- Cells -");
                grid_ui.end_row();
                grid_ui.label("Rotation Speed Max.: ");
                grid_ui.add(
                    DragValue::new(&mut control_center_ui.rotation_speed_max_drag_value)
                        .speed(0.01),
                );
                grid_ui.end_row();
                grid_ui.label("Acceleration Max.: ");
                grid_ui.add(
                    DragValue::new(&mut control_center_ui.acceleration_max_drag_value).speed(0.01),
                );
                grid_ui.end_row();
                grid_ui.label("Velocity Damping Bottom: ");
                grid_ui.add(Slider::new(
                    &mut control_center_ui.velocity_damping_slider_bottom,
                    0.0..=1.0,
                ));
                grid_ui.end_row();
                grid_ui.label("Velocity Damping Top: ");
                grid_ui.add(Slider::new(
                    &mut control_center_ui.velocity_damping_slider_top,
                    0.0..=1.0,
                ));
                grid_ui.end_row();
                grid_ui.label("Base Energy Drain: ");
                grid_ui.add(
                    DragValue::new(&mut control_center_ui.base_energy_drain_drag_value).speed(0.01),
                );
                grid_ui.end_row();
                grid_ui.colored_label(Rgba::GREEN, "- Food -");
                grid_ui.end_row();
                grid_ui.label("Energy: ");
                grid_ui.add(DragValue::new(
                    &mut control_center_ui.food_energy_drag_value,
                ));
                grid_ui.end_row();
                grid_ui.label("Spawn Chance Left: ");
                grid_ui.add(Slider::new(
                    &mut control_center_ui.food_spawn_chance_slider_left,
                    0.0..=0.25,
                ));
                grid_ui.end_row();
                grid_ui.label("Spawn Chance Right: ");
                grid_ui.add(Slider::new(
                    &mut control_center_ui.food_spawn_chance_slider_right,
                    0.0..=0.25,
                ));
                grid_ui.end_row();
                if grid_ui.button("Apply").clicked() {
                    apply_chunk_settings_events.send(ApplyChunkSettings);
                }
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
    simulation_settings: Res<SimulationSettings>,
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
                            .radius(simulation_settings.food_radius)
                            .color(Rgba::GREEN)
                            .name("food"),
                    );
                    let mut cell_points = Vec::new();
                    for position in &cell_query {
                        cell_points.push([position.x as f64, position.y as f64]);
                    }
                    plot_ui.points(
                        Points::new(PlotPoints::new(cell_points))
                            .radius(simulation_settings.cell_radius)
                            .color(Rgba::RED)
                            .name("cell"),
                    );
                });
        });
}
