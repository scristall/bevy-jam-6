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
mod ship;

use crate::game::game_state::GameState;
use crate::game::goldbar::{Gold, spawn_gold_bars, plugin as goldbar_plugin};
use crate::game::goldbar_text::{GoldBarTextPlugin, GoldAmount};
use crate::game::ship::{spawn_ship, move_ship};

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
            .add_plugins(GoldBarTextPlugin)
            .add_plugins(goldbar_plugin)
            // Add resources
            .init_resource::<GameConfig>()
            .init_resource::<WaveState>()
            // Add startup systems
            .add_systems(Startup, setup_game);
        app.add_systems(Startup, spawn_ship);
    }
}

fn setup_game(
    mut commands: Commands,
    game_config: Res<GameConfig>,
    asset_server: Res<AssetServer>,
    mut gold_amount: ResMut<GoldAmount>,
) {
    // Spawn spawner
    commands.spawn((
        Spawner,
        SpawnTimer(Timer::from_seconds(
            game_config.spawn_interval,
            TimerMode::Repeating,
        )),
        Transform::default(),
    ));

    // TODO: Spawn initial grid of tiles

    // Spawn gold bars
    spawn_gold_bars(&mut commands, &asset_server, gold_amount);

    // TODO: Setup UI
}
