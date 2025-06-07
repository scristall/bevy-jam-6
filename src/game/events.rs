use bevy::prelude::*;

use crate::game::goldbar::Gold;
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
pub struct GoldBarCollected {
    pub tile: Tile,
    pub entity: Entity,
}

#[derive(Event)]
pub struct GoldBarDropped {
    pub tile: Tile,
    pub entity: Entity,
}

#[derive(Event)]
pub struct WaveStarted;

#[derive(Event)]
pub struct WaveComplete;

#[derive(Event)]
pub struct PirateDeath {
    pub entity: Entity,
}

pub fn plugin(app: &mut App) {
    app.add_event::<TileMouseDown>();
    app.add_event::<TileMouseUp>();
    app.add_event::<TileMouseMove>();
    app.add_event::<GoldBarCollected>();
    app.add_event::<GoldBarDropped>();
    app.add_event::<WaveStarted>();
    app.add_event::<WaveComplete>();
    app.add_event::<PirateDeath>();
}
