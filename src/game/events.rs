#![allow(unused)]

use bevy::prelude::*;

use crate::game::tile::Tile;

pub struct TileEvent {
    pub tile: Tile,
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
}

#[derive(Event)]
pub struct GoldBarLost;

#[derive(Event)]
pub struct WaveStarted;

#[derive(Event)]
pub struct WaveComplete;

#[derive(Event)]
pub struct PirateDeath;

#[derive(Event)]
pub struct PrizeCollected;

#[derive(Event)]
pub struct GameOver;

#[derive(Event)]
pub struct PlayClickSFX;

#[derive(Event)]
pub struct PlayLongClickSFX;

#[derive(Event)]
pub struct ChainPlaced;

#[derive(Event)]
pub struct ChainFinished;

#[derive(Event)]
pub struct FoolsGoldSpawned {
    pub tile: Tile,
}

#[derive(Event)]
pub struct CrateSpawned {
    pub tile: Tile,
}

#[derive(Event)]
pub struct GlueSpawned {
    pub tile: Tile,
}

#[derive(Event)]
pub struct TreeSpawned {
    pub tile: Tile,
}

pub fn plugin(app: &mut App) {
    app.add_event::<TileMouseDown>();
    app.add_event::<TileMouseUp>();
    app.add_event::<TileMouseMove>();
    app.add_event::<GoldBarCollected>();
    app.add_event::<GoldBarDropped>();
    app.add_event::<GoldBarLost>();
    app.add_event::<WaveStarted>();
    app.add_event::<WaveComplete>();
    app.add_event::<PirateDeath>();
    app.add_event::<PrizeCollected>();
    app.add_event::<GameOver>();
    app.add_event::<FoolsGoldSpawned>();
    app.add_event::<CrateSpawned>();
    app.add_event::<PlayClickSFX>();
    app.add_event::<PlayLongClickSFX>();
    app.add_event::<ChainPlaced>();
    app.add_event::<ChainFinished>();
    app.add_event::<GlueSpawned>();
    app.add_event::<TreeSpawned>();
}
