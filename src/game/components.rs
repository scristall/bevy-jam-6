#![allow(warnings)]

use bevy::prelude::*;

#[derive(Component)]
pub struct Position(pub IVec2);

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub enum TileType {
    Empty,
    Chain,
    Wall,
    Gold,
}

#[derive(Component)]
pub struct TileProperties {
    pub oxygen_drain: f32,
    pub walkable: bool,
}

// Spawner components

#[derive(Component)]
pub struct TreasureCount(pub u32);

// UI components
#[derive(Component)]
pub struct OxygenBar;

#[derive(Component)]
pub struct WaveText;

#[derive(Component)]
pub struct GameOverUI;
