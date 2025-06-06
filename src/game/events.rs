use bevy::prelude::*;

use crate::game::tile::Tile;

#[derive(Event)]
pub struct TileClicked(pub Tile);

pub fn plugin(app: &mut App) {
    app.add_event::<TileClicked>();
}
