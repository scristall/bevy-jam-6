mod components;

use bevy::prelude::*;

mod background;
mod camera;
mod chain;
mod controls;
mod events;
mod game_state;
mod goldbar;
mod goldbar_text;
mod modifiers;
mod modifier_screen;
mod mouse;
mod music;
mod sound_effects;
mod oxygen;
mod pirate;
mod prizes;
mod ship;
mod tile;
mod tutorial;

use crate::game::game_state::GameState;

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
            .add_plugins(goldbar_text::plugin)
            .add_plugins(goldbar::plugin)
            .add_plugins(oxygen::plugin)
            .add_plugins(music::plugin)
            .add_plugins(sound_effects::plugin)
            .add_plugins(modifiers::plugin)
            .add_plugins(modifier_screen::plugin);
    }
}
