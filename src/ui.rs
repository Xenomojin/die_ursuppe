use crate::sim::{
    ApplyChunkSettings, ApplySimulationSettings, Cell, Clear, Energy, Food, Position, Save,
    SimulationSettings, SpawnCell, TogglePause,
};
use bevy::prelude::*;
use bevy_egui::{
    egui::{
        plot::{Line, Plot, PlotPoints, Points},
        CentralPanel, DragValue, Grid, Rgba, Slider, Window,
    },
    EguiContext,
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
    pub energy_required_for_split_drag_value: f32,
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
            neuron_energy_drain_drag_value: 0.01,
            connection_energy_drain_drag_value: 0.004,
            energy_required_for_split_drag_value: 20.,
            cell_energy_drag_value: 199.,
            cell_amount_slider: 50,
            rotation_speed_max_drag_value: 1.,
            acceleration_max_drag_value: 2.,
            velocity_damping_slider_bottom: 0.4,
            velocity_damping_slider_top: 0.4,
            food_energy_drag_value: 200.,
            food_spawn_chance_slider_left: 0.,
            food_spawn_chance_slider_right: 0.,
        }
    }
}

pub fn display_control_center(
    mut egui_context: ResMut<EguiContext>,
    mut control_center_ui: ResMut<ControlCenterUi>,
    mut spawn_cell_events: EventWriter<SpawnCell>,
    mut apply_chunk_settings_events: EventWriter<ApplyChunkSettings>,
    mut apply_simulation_settings_events: EventWriter<ApplySimulationSettings>,
    mut toggle_pause_events: EventWriter<TogglePause>,
    mut clear_events: EventWriter<Clear>,
    mut save_events: EventWriter<Save>,
    mut statistic_query: Query<(&Label, &mut IsOpen), With<Statistic>>,
) {
    Window::new("Control Center")
        .resizable(true)
        .show(egui_context.ctx_mut(), |ui| {
            ui.collapsing("Simulation Settings", |collapsing_ui| {
                Grid::new("simulation_settings_grid").show(collapsing_ui, |grid_ui| {
                    grid_ui.colored_label(Rgba::GREEN, "- Time -");
                    grid_ui.end_row();
                    grid_ui.label("Tick delta seconds: ");
                    grid_ui.add(Slider::new(
                        &mut control_center_ui.tick_delta_seconds_slider,
                        0.0..=0.2,
                    ));
                    grid_ui.end_row();
                    grid_ui.label("Actual tick delta seconds: ");
                    grid_ui.colored_label(
                        Rgba::WHITE,
                        &control_center_ui.actual_tick_delta_seconds_label,
                    );
                    grid_ui.end_row();
                    grid_ui.colored_label(Rgba::GREEN, "- Cells -");
                    grid_ui.end_row();
                    grid_ui.label("Base energy drain: ");
                    grid_ui.add(
                        DragValue::new(&mut control_center_ui.base_energy_drain_drag_value)
                            .speed(0.01),
                    );
                    grid_ui.end_row();
                    grid_ui.label("Neuron rnergy drain: ");
                    grid_ui.add(
                        DragValue::new(&mut control_center_ui.neuron_energy_drain_drag_value)
                            .speed(0.001),
                    );
                    grid_ui.end_row();
                    grid_ui.label("Connection energy drain: ");
                    grid_ui.add(
                        DragValue::new(&mut control_center_ui.connection_energy_drain_drag_value)
                            .speed(0.0001),
                    );
                    grid_ui.end_row();
                    grid_ui.label("Energy required for split: ");
                    grid_ui.add(
                        DragValue::new(&mut control_center_ui.energy_required_for_split_drag_value)
                            .speed(0.1),
                    );
                    grid_ui.end_row();
                    grid_ui.label("Cell radius: ");
                    grid_ui.add(
                        DragValue::new(&mut control_center_ui.cell_radius_drag_value).speed(0.01),
                    );
                    grid_ui.end_row();
                    grid_ui.colored_label(Rgba::GREEN, "- Miscellaneous -");
                    grid_ui.end_row();
                    grid_ui.label("Food radius: ");
                    grid_ui.add(
                        DragValue::new(&mut control_center_ui.food_radius_drag_value).speed(0.01),
                    );
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
            });
            ui.collapsing("Spawn Cells", |collapsing_ui| {
                Grid::new("spawn_cells_grid").show(collapsing_ui, |grid_ui| {
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
            });
            ui.collapsing("Chunk Settings", |collapsing_ui| {
                Grid::new("chunk_settings_grid").show(collapsing_ui, |grid_ui| {
                    grid_ui.colored_label(Rgba::GREEN, "- Cells -");
                    grid_ui.end_row();
                    grid_ui.label("Rotation speed max.: ");
                    grid_ui.add(
                        DragValue::new(&mut control_center_ui.rotation_speed_max_drag_value)
                            .speed(0.01),
                    );
                    grid_ui.end_row();
                    grid_ui.label("Acceleration max.: ");
                    grid_ui.add(
                        DragValue::new(&mut control_center_ui.acceleration_max_drag_value)
                            .speed(0.01),
                    );
                    grid_ui.end_row();
                    grid_ui.label("Velocity damping bottom: ");
                    grid_ui.add(Slider::new(
                        &mut control_center_ui.velocity_damping_slider_bottom,
                        0.0..=1.0,
                    ));
                    grid_ui.end_row();
                    grid_ui.label("Velocity damping top: ");
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
                    grid_ui.label("Spawn chance left: ");
                    grid_ui.add(Slider::new(
                        &mut control_center_ui.food_spawn_chance_slider_left,
                        0.0..=0.1,
                    ));
                    grid_ui.end_row();
                    grid_ui.label("Spawn chance right: ");
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
            });
            ui.collapsing("Destroy", |collapsing_ui| {
                if collapsing_ui.button("Clear").clicked() {
                    clear_events.send(Clear);
                }
            });
            ui.collapsing("Save & Load", |collapsing_ui| {
                Grid::new("save_and_load_grid").show(collapsing_ui, |grid_ui| {
                    if grid_ui.button("Save").clicked() {
                        save_events.send(Save);
                    }
                    if grid_ui.button("Load").clicked() {
                        todo!();
                    }
                    grid_ui.end_row();
                });
            });
            ui.collapsing("Statistics", |collapsing_ui| {
                for (label, mut is_open) in &mut statistic_query {
                    collapsing_ui.checkbox(&mut **is_open, label.as_str());
                }
            });
        });
}

pub fn display_simulation(
    mut egui_context: ResMut<EguiContext>,
    simulation_settings: Res<SimulationSettings>,
    cell_query: Query<(&Position, &Energy), With<Cell>>,
    food_query: Query<&Position, With<Food>>,
) {
    CentralPanel::default().show(egui_context.ctx_mut(), |ui| {
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
                let mut cell_points_by_energy =
                    vec![Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new()];
                let colors_by_energy = [
                    Rgba::from_rgb(0.2, 0., 0.),
                    Rgba::from_rgb(0.4, 0., 0.),
                    Rgba::from_rgb(0.6, 0., 0.),
                    Rgba::from_rgb(0.8, 0., 0.),
                    Rgba::from_rgb(1., 0., 0.),
                ];
                let labels_by_energy = [
                    "Cell below 50 energy",
                    "Cell below 100 energy",
                    "Cell below 200 energy",
                    "Cell below 400 energy",
                    "Cell above 400 energy",
                ];
                for (position, energy) in &cell_query {
                    if **energy < 50. {
                        cell_points_by_energy[0].push([position.x as f64, position.y as f64]);
                    } else if **energy < 100. {
                        cell_points_by_energy[1].push([position.x as f64, position.y as f64]);
                    } else if **energy < 200. {
                        cell_points_by_energy[2].push([position.x as f64, position.y as f64]);
                    } else if **energy < 400. {
                        cell_points_by_energy[3].push([position.x as f64, position.y as f64]);
                    } else {
                        cell_points_by_energy[4].push([position.x as f64, position.y as f64]);
                    }
                }
                for idx in 0..5 {
                    plot_ui.points(
                        Points::new(PlotPoints::new(cell_points_by_energy[idx].clone()))
                            .radius(simulation_settings.cell_radius)
                            .color(colors_by_energy[idx])
                            .name(labels_by_energy[idx]),
                    );
                }
            });
    });
}

#[derive(Default, Component, Reflect)]
#[reflect(Component)]
pub struct Statistic;

#[derive(Deref, DerefMut, Default, Component, Reflect)]
#[reflect(Component)]
pub struct Label(pub String);

#[derive(Deref, DerefMut, Default, Component, Reflect)]
#[reflect(Component)]
pub struct IsOpen(pub bool);

#[derive(Default, Component, Reflect)]
#[reflect(Component)]
pub struct StatisticData {
    pub lines: Vec<StatisticLine>,
}

#[derive(Reflect, FromReflect)]
pub struct StatisticLine {
    pub legend_name: String,
    pub color: [f32; 3],
    pub data_points: Vec<f32>,
}

#[derive(Default, Bundle)]
pub struct StatisticBundle<T: Sync + Send + Component + 'static> {
    pub statistic: Statistic,
    pub unique_tag_component: T,
    pub label: Label,
    pub is_open: IsOpen,
    pub data: StatisticData,
}

#[derive(Default, Component, Reflect)]
#[reflect(Component)]
pub struct ChildCountStatistic;

#[derive(Default, Component, Reflect)]
#[reflect(Component)]
pub struct CellCountStatistic;

#[derive(Default, Component, Reflect)]
#[reflect(Component)]
pub struct BrainSizeStatistic;

pub fn setup_statistics(mut commands: Commands) {
    commands.spawn(StatisticBundle {
        label: Label("Child Count Statistic".to_string()),
        unique_tag_component: ChildCountStatistic,
        data: StatisticData {
            lines: vec![StatisticLine {
                legend_name: "Avg. child count of living cells".to_string(),
                color: [0., 1., 0.],
                data_points: Vec::new(),
            }],
        },
        ..default()
    });
    commands.spawn(StatisticBundle {
        label: Label("Cell Count Statistic".to_string()),
        unique_tag_component: CellCountStatistic,
        data: StatisticData {
            lines: vec![
                StatisticLine {
                    legend_name: "Toatal cell count".to_string(),
                    color: [1., 1., 1.],
                    data_points: Vec::new(),
                },
                StatisticLine {
                    legend_name: "Cells born".to_string(),
                    color: [1., 0.5, 0.],
                    data_points: Vec::new(),
                },
                StatisticLine {
                    legend_name: "Cells died".to_string(),
                    color: [1., 0., 0.5],
                    data_points: Vec::new(),
                },
            ],
        },
        ..default()
    });
    commands.spawn(StatisticBundle {
        label: Label("Brain Size Statistic".to_string()),
        unique_tag_component: BrainSizeStatistic,
        data: StatisticData {
            lines: vec![
                StatisticLine {
                    legend_name: "Avg. neuron count of living cells".to_string(),
                    color: [0., 1., 0.],
                    data_points: Vec::new(),
                },
                StatisticLine {
                    legend_name: "Avg. connection count of living cells".to_string(),
                    color: [0., 1., 0.5],
                    data_points: Vec::new(),
                },
                StatisticLine {
                    legend_name: "Avg. connection count / neuron count of living cells".to_string(),
                    color: [1., 1., 1.],
                    data_points: Vec::new(),
                },
            ],
        },
        ..default()
    });
}

pub fn display_statistics(
    mut egui_context: ResMut<EguiContext>,
    mut statistic_query: Query<(&Label, &mut IsOpen, &StatisticData), With<Statistic>>,
) {
    for (label, mut is_open, data) in &mut statistic_query {
        Window::new(&**label)
            .resizable(true)
            .open(&mut **is_open)
            .show(egui_context.ctx_mut(), |ui| {
                Plot::new("brain_size_statistic")
                    .legend(default())
                    .show(ui, |plot_ui| {
                        for line in &data.lines {
                            plot_ui.line(
                                Line::new(PlotPoints::from_ys_f32(&line.data_points))
                                    .color(Rgba::from_rgb(
                                        line.color[0],
                                        line.color[1],
                                        line.color[2],
                                    ))
                                    .name(&line.legend_name),
                            );
                        }
                    });
            });
    }
}
