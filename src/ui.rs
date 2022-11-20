use bevy::prelude::*;
use bevy_egui::{
    egui::{
        plot::{Line, Plot, PlotPoints, Points},
        DragValue, Grid, Rgba, Slider, Window,
    },
    EguiContext,
};

use crate::sim::{
    ApplyChunkSettings, ApplySimulationSettings, Cell, Clear, Food, Position, SimulationSettings,
    SpawnCell, TogglePause,
};

#[derive(Resource)]
pub struct ControlCenterUi {
    pub tick_delta_seconds_slider: f32,
    pub actual_tick_delta_seconds_label: String,
    pub pause_button_text: String,
    pub cell_radius_drag_value: f32,
    pub food_radius_drag_value: f32,
    pub base_energy_drain_drag_value: f32,
    pub neuron_energy_drain_drag_value: f32,
    pub connection_energy_drain_drag_value: f32,
    /// Start Energy-Wert für zukünftige manuell gespawnte cells
    pub cell_energy_drag_value: f32,
    pub cell_amount_slider: u32,
    pub rotation_speed_max_drag_value: f32,
    pub acceleration_max_drag_value: f32,
    /// Energy-Wert für zukünftiges food
    pub food_energy_drag_value: f32,
    pub food_spawn_chance_slider_left: f32,
    pub food_spawn_chance_slider_right: f32,
    /// Wert zwischen 0 (kein damping) und 1 (100% damping)
    pub velocity_damping_slider_top: f32,
    /// Wert zwischen 0 (kein damping) und 1 (100% damping)
    pub velocity_damping_slider_bottom: f32,
    pub child_count_statistic_checkbox: bool,
    pub cell_count_statistic_checkbox: bool,
    pub neuron_count_statistic_checkbox: bool,
}

impl Default for ControlCenterUi {
    fn default() -> Self {
        Self {
            tick_delta_seconds_slider: 0.02,
            actual_tick_delta_seconds_label: "-".to_string(),
            pause_button_text: "Play".to_string(),
            cell_radius_drag_value: 5.,
            food_radius_drag_value: 3.,
            base_energy_drain_drag_value: 0.4,
            neuron_energy_drain_drag_value: 0.02,
            connection_energy_drain_drag_value: 0.004,
            cell_energy_drag_value: 199.,
            cell_amount_slider: 50,
            rotation_speed_max_drag_value: 1.,
            acceleration_max_drag_value: 2.,
            velocity_damping_slider_bottom: 0.5,
            velocity_damping_slider_top: 0.5,
            food_energy_drag_value: 200.,
            food_spawn_chance_slider_left: 0.,
            food_spawn_chance_slider_right: 0.,
            child_count_statistic_checkbox: false,
            cell_count_statistic_checkbox: false,
            neuron_count_statistic_checkbox: false,
        }
    }
}

pub fn display_control_center_ui(
    mut egui_context: ResMut<EguiContext>,
    mut control_center_ui: ResMut<ControlCenterUi>,
    mut spawn_cell_events: EventWriter<SpawnCell>,
    mut apply_chunk_settings_events: EventWriter<ApplyChunkSettings>,
    mut apply_simulation_settings_events: EventWriter<ApplySimulationSettings>,
    mut toggle_pause_events: EventWriter<TogglePause>,
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
                grid_ui.label("Actual Tick Delta Seconds: ");
                grid_ui.colored_label(
                    Rgba::WHITE,
                    &control_center_ui.actual_tick_delta_seconds_label,
                );
                grid_ui.end_row();
                grid_ui.colored_label(Rgba::GREEN, "- Cells -");
                grid_ui.end_row();
                grid_ui.label("Base Energy Drain: ");
                grid_ui.add(
                    DragValue::new(&mut control_center_ui.base_energy_drain_drag_value).speed(0.01),
                );
                grid_ui.end_row();
                grid_ui.label("Neuron Energy Drain: ");
                grid_ui.add(
                    DragValue::new(&mut control_center_ui.neuron_energy_drain_drag_value)
                        .speed(0.001),
                );
                grid_ui.end_row();
                grid_ui.label("Connection Energy Drain: ");
                grid_ui.add(
                    DragValue::new(&mut control_center_ui.connection_energy_drain_drag_value)
                        .speed(0.0001),
                );
                grid_ui.end_row();
                grid_ui.label("Cell Radius: ");
                grid_ui
                    .add(DragValue::new(&mut control_center_ui.cell_radius_drag_value).speed(0.01));
                grid_ui.end_row();
                grid_ui.colored_label(Rgba::GREEN, "- Miscellaneous -");
                grid_ui.end_row();
                grid_ui.label("Food Radius: ");
                grid_ui
                    .add(DragValue::new(&mut control_center_ui.food_radius_drag_value).speed(0.01));
                grid_ui.end_row();
                grid_ui.horizontal(|cell_ui| {
                    if cell_ui.button("Apply").clicked() {
                        apply_simulation_settings_events.send(ApplySimulationSettings);
                    }
                    if cell_ui
                        .button(&control_center_ui.pause_button_text)
                        .clicked()
                    {
                        toggle_pause_events.send(TogglePause);
                    }
                });
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
                    0.0..=0.1,
                ));
                grid_ui.end_row();
                grid_ui.label("Spawn Chance Right: ");
                grid_ui.add(Slider::new(
                    &mut control_center_ui.food_spawn_chance_slider_right,
                    0.0..=0.1,
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
            ui.separator();
            ui.heading("Statistics");
            ui.checkbox(
                &mut control_center_ui.child_count_statistic_checkbox,
                " Child Count Statistic",
            );
            ui.checkbox(
                &mut control_center_ui.cell_count_statistic_checkbox,
                " Cell Count Statistic",
            );
            ui.checkbox(
                &mut control_center_ui.neuron_count_statistic_checkbox,
                " Neuron Count Statistic",
            );
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
            Plot::new("simulation")
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
                            .name("Food"),
                    );
                    let mut cell_points = Vec::new();
                    for position in &cell_query {
                        cell_points.push([position.x as f64, position.y as f64]);
                    }
                    plot_ui.points(
                        Points::new(PlotPoints::new(cell_points))
                            .radius(simulation_settings.cell_radius)
                            .color(Rgba::RED)
                            .name("Cell"),
                    );
                });
        });
}

#[derive(Resource, Default)]
pub struct ChildCountStatisticUi {
    pub points: Vec<[f64; 2]>,
    pub average_points: Vec<[f64; 2]>,
}

pub fn display_child_count_statistic_ui(
    mut egui_context: ResMut<EguiContext>,
    mut control_center_ui: ResMut<ControlCenterUi>,
    child_count_statistic_ui: Res<ChildCountStatisticUi>,
) {
    Window::new("Child Count Statistic")
        .resizable(true)
        .open(&mut control_center_ui.child_count_statistic_checkbox)
        .show(egui_context.ctx_mut(), |ui| {
            Plot::new("child_count_statistic")
                .legend(default())
                .show(ui, |plot_ui| {
                    plot_ui.points(
                        Points::new(child_count_statistic_ui.points.clone())
                            .radius(2.)
                            .color(Rgba::RED)
                            .name("Child count on cell death"),
                    );
                    plot_ui.line(
                        Line::new(child_count_statistic_ui.average_points.clone())
                            .color(Rgba::GREEN)
                            .name("Avg. child count of living cells"),
                    );
                });
        });
}

#[derive(Resource, Default)]
pub struct CellCountStatisticUi {
    pub points: Vec<[f64; 2]>,
}

pub fn display_cell_count_statistic_ui(
    mut egui_context: ResMut<EguiContext>,
    mut control_center_ui: ResMut<ControlCenterUi>,
    cell_count_statistic_ui: Res<CellCountStatisticUi>,
) {
    Window::new("Cell Count Statistic")
        .resizable(true)
        .open(&mut control_center_ui.cell_count_statistic_checkbox)
        .show(egui_context.ctx_mut(), |ui| {
            Plot::new("cell_count_statistic")
                .legend(default())
                .show(ui, |plot_ui| {
                    plot_ui.line(
                        Line::new(cell_count_statistic_ui.points.clone())
                            .color(Rgba::RED)
                            .name("Cell count"),
                    );
                });
        });
}

#[derive(Resource, Default)]
pub struct NeuronCountStatisticUi {
    pub points: Vec<[f64; 2]>,
    pub average_points: Vec<[f64; 2]>,
}

pub fn display_neuron_count_statistic_ui(
    mut egui_context: ResMut<EguiContext>,
    mut control_center_ui: ResMut<ControlCenterUi>,
    neuron_count_statistic_ui: Res<NeuronCountStatisticUi>,
) {
    Window::new("Neuron Count Statistic")
        .resizable(true)
        .open(&mut control_center_ui.neuron_count_statistic_checkbox)
        .show(egui_context.ctx_mut(), |ui| {
            Plot::new("neuron_count_statistic")
                .legend(default())
                .show(ui, |plot_ui| {
                    plot_ui.points(
                        Points::new(neuron_count_statistic_ui.points.clone())
                            .radius(2.)
                            .color(Rgba::RED)
                            .name("Neuron count on cell death"),
                    );
                    plot_ui.line(
                        Line::new(neuron_count_statistic_ui.average_points.clone())
                            .color(Rgba::GREEN)
                            .name("Avg. neuron count of living cells"),
                    );
                });
        });
}
