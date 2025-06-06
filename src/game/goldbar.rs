use bevy::prelude::*;
use crate::game::tile::{Tile, TILE_SIZE, GRID_X_START, GRID_Y_START};
use crate::game::components::{Position, TileType};

pub const GOLD_ROOM_X: i32 = 27;
pub const GOLD_ROOM_Y: i32 = 1;

#[derive(Component)]
pub struct Gold;

pub fn spawn_gold_bars(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) {
    let mut gold_positions = Vec::with_capacity(18);
    for x in GOLD_ROOM_X..=GOLD_ROOM_X+1 {
        for y in GOLD_ROOM_Y..=GOLD_ROOM_Y+8 {
            gold_positions.push(IVec2::new(x, y));
        }
    }

    for pos in gold_positions {
        commands.spawn((
            Gold,
            Position(pos),
            TileType::Gold,
            Sprite {
                image: asset_server.load("images/gold_bar.png"),
                ..default()
            },
            Tile { x: pos.x, y: pos.y }.grid_coord_to_transform(2.0),
        ));
    }
}

