mod components;

use bevy::prelude::*;
use components::*;

mod background;
mod camera;
mod chain;
mod controls;
mod events;
mod game_state;
mod goldbar;
mod goldbar_text;
mod mouse;
mod pirate;
mod tile;
mod prizes;
mod tutorial;
mod music;
mod ship;

use crate::game::game_state::GameState;
use crate::game::goldbar_text::{GoldBarTextPlugin, GoldAmount};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_plugins(controls::plugin)
            .add_plugins(camera::plugin)
            .add_plugins(mouse::plugin)
            .add_plugins(background::plugin)
            .add_plugins(tile::plugin)
            .add_plugins(chain::plugin)
            .add_plugins(events::plugin)
            .add_plugins(pirate::plugin)
            .add_plugins(prizes::plugin)
            .add_plugins(tutorial::plugin)
            .add_plugins(ship::plugin)
            .add_plugins(GoldBarTextPlugin)
            .add_plugins(goldbar::plugin)
            .add_plugins(music::plugin);
    }
}
