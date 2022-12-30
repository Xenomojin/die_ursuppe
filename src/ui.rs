use crate::brain::Brain;
use crate::sim::{
    ApplyChunkSettings, ApplySimulationSettings, Cell, CellStats, Clear, Energy, Food, Load,
    Position, Save, SimulationSettings, SpawnCell, TogglePause,
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
    pub cell_radius_drag_value: f32,
    pub food_radius_drag_value: f32,
    pub base_energy_drain_drag_value: f32,
    pub neuron_energy_drain_drag_value: f32,
    pub connection_energy_drain_drag_value: f32,
    pub energy_required_for_split_drag_value: f32,
    pub rotation_speed_max_drag_value: f32,
    pub acceleration_max_drag_value: f32,
    /// Start Energy-Wert für zukünftige manuell gespawnte cells
    pub cell_energy_drag_value: f32,
    pub cell_amount_slider: u32,
    /// Energy-Wert für zukünftiges food
    pub food_energy_drag_value: f32,
    pub food_spawn_chance_slider_left: f32,
    pub food_spawn_chance_slider_right: f32,
    /// Wert zwischen 0 (kein damping) und 1 (100% damping)
    pub velocity_damping_slider_top: f32,
    /// Wert zwischen 0 (kein damping) und 1 (100% damping)
    pub velocity_damping_slider_bottom: f32,
    pub clear_food_checkbox: bool,
    pub clear_cells_checkbox: bool,
    pub clear_statistics_checkbox: bool,
    pub save_name_text_edit: String,
}

impl Default for ControlCenterUi {
    fn default() -> Self {
        Self {
            tick_delta_seconds_slider: 0.02,
            actual_tick_delta_seconds_label: "-".to_string(),
            cell_radius_drag_value: 5.,
            food_radius_drag_value: 3.,
            base_energy_drain_drag_value: 0.4,
            neuron_energy_drain_drag_value: 0.01,
            connection_energy_drain_drag_value: 0.004,
            energy_required_for_split_drag_value: 10.,
            rotation_speed_max_drag_value: 1.,
            acceleration_max_drag_value: 1.7,
            cell_energy_drag_value: 199.,
            cell_amount_slider: 50,
            velocity_damping_slider_bottom: 0.4,
            velocity_damping_slider_top: 0.4,
            food_energy_drag_value: 200.,
            food_spawn_chance_slider_left: 0.018,
            food_spawn_chance_slider_right: 0.018,
            clear_food_checkbox: true,
            clear_cells_checkbox: true,
            clear_statistics_checkbox: false,
            save_name_text_edit: "save".to_string(),
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
    mut load_events: EventWriter<Load>,
    mut statistic_query: Query<(&Label, &mut IsOpen), With<Statistic>>,
    simulation_settings: Res<SimulationSettings>,
) {
    Window::new("Control Center")
        .resizable(true)
        .show(egui_context.ctx_mut(), |ui| {
            ui.collapsing("Simulation Settings", |collapsing_ui| {
                Grid::new("simulation_settings_grid").show(collapsing_ui, |grid_ui| {
                    grid_ui.colored_label(Rgba::from_rgb(0.145, 0.569, 0.129), "- Time -");
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
                    grid_ui.colored_label(Rgba::from_rgb(0.145, 0.569, 0.129), "- Cells -");
                    grid_ui.end_row();
                    grid_ui.label("Base energy drain: ");
                    grid_ui.add(
                        DragValue::new(&mut control_center_ui.base_energy_drain_drag_value)
                            .speed(0.01),
                    );
                    grid_ui.end_row();
                    grid_ui.label("Neuron energy drain: ");
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
                    grid_ui.label("Cell radius: ");
                    grid_ui.add(
                        DragValue::new(&mut control_center_ui.cell_radius_drag_value).speed(0.01),
                    );
                    grid_ui.end_row();
                    grid_ui.colored_label(Rgba::from_rgb(0.145, 0.569, 0.129), "- Miscellaneous -");
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
                            .button(if simulation_settings.is_paused {
                                "Play"
                            } else {
                                "Pause"
                            })
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
                    grid_ui.colored_label(Rgba::from_rgb(0.145, 0.569, 0.129), "- Cells -");
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
                    grid_ui.colored_label(Rgba::from_rgb(0.145, 0.569, 0.129), "- Food -");
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
                collapsing_ui.checkbox(&mut control_center_ui.clear_food_checkbox, "Clear food");
                collapsing_ui.checkbox(&mut control_center_ui.clear_cells_checkbox, "Clear cells");
                collapsing_ui.checkbox(
                    &mut control_center_ui.clear_statistics_checkbox,
                    "Clear statistics",
                );
                if collapsing_ui.button("Clear").clicked() {
                    clear_events.send(Clear {
                        clear_food: control_center_ui.clear_food_checkbox,
                        clear_cells: control_center_ui.clear_cells_checkbox,
                        clear_statistics: control_center_ui.clear_statistics_checkbox,
                    });
                }
            });
            ui.collapsing("Save & Load", |collapsing_ui| {
                Grid::new("save_and_load_grid").show(collapsing_ui, |grid_ui| {
                    grid_ui.label("Save name: ");
                    grid_ui.text_edit_singleline(&mut control_center_ui.save_name_text_edit);
                    grid_ui.end_row();
                    if grid_ui.button("Save").clicked() {
                        save_events.send(Save {
                            save_name: control_center_ui.save_name_text_edit.clone(),
                        });
                    }
                    if grid_ui.button("Load").clicked() {
                        load_events.send(Load {
                            save_name: control_center_ui.save_name_text_edit.clone(),
                        });
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
    mut cell_inspector_ui: ResMut<CellInspectorUi>,
    simulation_settings: Res<SimulationSettings>,
    cell_query: Query<(Entity, &Position, &Energy), With<Cell>>,
    food_query: Query<&Position, With<Food>>,
) {
    CentralPanel::default().show(egui_context.ctx_mut(), |ui| {
        Plot::new("simulation_plot")
            .data_aspect(1.)
            .view_aspect(1.)
            .legend(default())
            .show(ui, |plot_ui| {
                // Food daten sammeln
                let mut food_points = Vec::new();
                for position in &food_query {
                    food_points.push([position.x as f64, position.y as f64]);
                }

                // Food zeichnen
                plot_ui.points(
                    Points::new(PlotPoints::new(food_points))
                        .radius(simulation_settings.food_radius)
                        .color(Rgba::from_rgb(0.145, 0.569, 0.129))
                        .name("Food"),
                );

                // Cell daten sammeln
                let mut cell_point_groups = vec![
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                ];
                let group_colors = [
                    Rgba::from_gray(0.8),
                    Rgba::from_rgb(0.569, 0.129, 0.145) * 0.2,
                    Rgba::from_rgb(0.569, 0.129, 0.145) * 0.4,
                    Rgba::from_rgb(0.569, 0.129, 0.145) * 0.6,
                    Rgba::from_rgb(0.569, 0.129, 0.145) * 0.8,
                    Rgba::from_rgb(0.569, 0.129, 0.145),
                ];
                let group_labels = [
                    "Selected cell",
                    "Cell < 50 energy",
                    "Cell < 100 energy",
                    "Cell < 200 energy",
                    "Cell < 400 energy",
                    "Cell > 400 energy",
                ];
                for (entity, position, energy) in &cell_query {
                    let group = if Some(entity) == cell_inspector_ui.selected_cell {
                        0
                    } else if **energy < 50. {
                        1
                    } else if **energy < 100. {
                        2
                    } else if **energy < 200. {
                        3
                    } else if **energy < 400. {
                        4
                    } else {
                        5
                    };
                    cell_point_groups[group].push([position.x as f64, position.y as f64]);
                }

                // Cells zeichnen
                for index in 0..cell_point_groups.len() {
                    plot_ui.points(
                        Points::new(PlotPoints::new(cell_point_groups[index].clone()))
                            .radius(simulation_settings.cell_radius)
                            .color(group_colors[index])
                            .name(group_labels[index]),
                    );
                }

                if plot_ui.plot_clicked() {
                    // Zelle finden, die am nächsten zu Cursor ist und selecten
                    let curser_postition = plot_ui.pointer_coordinate().unwrap();
                    let mut nearest_cell_entity = None;
                    let mut nearest_cell_distance_squared = f32::INFINITY;
                    for (entiy, position, _) in &cell_query {
                        let relative_position = Position {
                            x: curser_postition.x as f32 - position.x,
                            y: curser_postition.y as f32 - position.y,
                        };
                        let distance_squared = relative_position.x * relative_position.x
                            + relative_position.y * relative_position.y;
                        if distance_squared < nearest_cell_distance_squared {
                            nearest_cell_distance_squared = distance_squared;
                            nearest_cell_entity = Some(entiy);
                        }
                    }
                    cell_inspector_ui.selected_cell = nearest_cell_entity;
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
                legend_name: "Avg. child count".to_string(),
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
                    legend_name: "Avg. neuron count".to_string(),
                    color: [0., 1., 0.],
                    data_points: Vec::new(),
                },
                StatisticLine {
                    legend_name: "Avg. connection count".to_string(),
                    color: [0., 1., 0.5],
                    data_points: Vec::new(),
                },
                StatisticLine {
                    legend_name: "Avg. connection count / neuron count".to_string(),
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
                Plot::new("statistic_plot")
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

#[derive(Default, Resource)]
pub struct CellInspectorUi {
    pub selected_cell: Option<Entity>,
}

pub fn display_cell_inspector(
    mut egui_context: ResMut<EguiContext>,
    mut cell_inspector_ui: ResMut<CellInspectorUi>,
    cell_query: Query<(&Brain, &Energy, &CellStats), With<Cell>>,
) {
    let Some(selected_cell) = cell_inspector_ui.selected_cell else {
        return
    };
    let Ok((brain, energy, stats)) = cell_query.get(selected_cell) else {
        cell_inspector_ui.selected_cell = None;
        return
    };
    let mut is_open = true;
    Window::new("Cell Inspector")
        .resizable(true)
        .open(&mut is_open)
        .show(egui_context.ctx_mut(), |ui| {
            Grid::new("cell_inspector_grid").show(ui, |grid_ui| {
                grid_ui.colored_label(Rgba::from_rgb(0.145, 0.569, 0.129), "- Stats -");
                grid_ui.end_row();
                grid_ui.label("Energy: ");
                grid_ui.colored_label(Rgba::WHITE, format!("{:.0}", **energy));
                grid_ui.end_row();
                grid_ui.label("Age: ");
                grid_ui.colored_label(Rgba::WHITE, format!("{} ticks", stats.age));
                grid_ui.end_row();
                grid_ui.label("Child count: ");
                grid_ui.colored_label(Rgba::WHITE, format!("{}", stats.child_count));
                grid_ui.end_row();
                grid_ui.colored_label(Rgba::from_rgb(0.145, 0.569, 0.129), "- Brain -");
                grid_ui.end_row();
            });
            Plot::new("brain_plot")
                .data_aspect(1.)
                .legend(default())
                .show(ui, |plot_ui| {
                    // Neuronen daten sammeln
                    let mut neuron_positons = Vec::new();
                    for index in 0..brain.neurons().len() {
                        neuron_positons.push([(index % 4) as f64, (index / 4) as f64]);
                    }

                    // Connection daten sammeln
                    let mut connection_position_origins = Vec::new();
                    let mut connection_position_tips = Vec::new();
                    let mut connection_weights = Vec::new();
                    for (index, neuron) in brain.neurons().iter().enumerate() {
                        for input in &neuron.inputs {
                            connection_position_origins.push(neuron_positons[index]);
                            connection_position_tips.push(neuron_positons[input.neuron_index]);
                            connection_weights.push(input.weight);
                        }
                    }

                    // Neuronen zeichnen
                    plot_ui.points(
                        Points::new(PlotPoints::new(neuron_positons))
                            .radius(8.)
                            .color(Rgba::from_rgb(0.129, 0.145, 0.569))
                            .name("Neuron"),
                    );

                    // Connections zeichnen
                    for index in 0..connection_position_origins.len() {
                        plot_ui.line(
                            Line::new(vec![
                                connection_position_origins[index],
                                connection_position_tips[index],
                            ])
                            .width(connection_weights[index].abs() * 2.)
                            .color(if connection_weights[index].is_sign_positive() {
                                Rgba::from_rgb(0.145, 0.569, 0.129)
                            } else {
                                Rgba::from_rgb(0.569, 0.129, 0.145)
                            })
                            .name(
                                if connection_weights[index].is_sign_positive() {
                                    "Positive connection"
                                } else {
                                    "Negative connection"
                                },
                            ),
                        );
                    }
                });
        });
    if !is_open {
        cell_inspector_ui.selected_cell = None;
    }
}
