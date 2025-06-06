use bevy::prelude::*;

use crate::game::tile::Tile;

pub struct TileEvent {
    pub tile: Tile,
    pub entity: Entity,
}

#[derive(Event)]
pub struct TileMouseDown(pub TileEvent);

#[derive(Event)]
pub struct TileMouseUp(pub TileEvent);

#[derive(Event)]
pub struct TileMouseMove(pub TileEvent);

#[derive(Event)]
pub struct GoldBarCollected(pub TileEvent);

pub fn plugin(app: &mut App) {
    app.add_event::<TileMouseDown>();
    app.add_event::<TileMouseUp>();
    app.add_event::<TileMouseMove>();
    app.add_event::<GoldBarCollected>();
}
