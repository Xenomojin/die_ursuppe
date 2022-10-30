use bevy::prelude::*;

use crate::brain::Brain;

#[derive(Debug, Component)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Component)]
pub struct Rotation(pub f32);

#[derive(Debug, Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Component)]
pub struct Age(pub u32);

#[derive(Debug, Component)]
pub struct Energy(pub f32);

#[derive(Debug, Component)]
pub struct Cell;

#[derive(Debug, Component)]
pub struct Food;

pub fn tick() {}

pub fn move_cell(mut cell_query: Query<(&mut Position, &mut Velocity), With<Cell>>) {
    for (mut position, mut velocity) in &mut cell_query {}
}
