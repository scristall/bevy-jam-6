mod components;
mod systems;

use bevy::prelude::*;
use components::*;
use systems::*;

mod camera;
mod background;
mod tile;
mod chain;
mod events;
mod mouse;
mod goldbar;
mod pirate;

use crate::game::goldbar::{Gold, spawn_gold_bars};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(camera::plugin)
            .add_plugins(mouse::plugin)
            .add_plugins(background::plugin)
            .add_plugins(tile::plugin)
            .add_plugins(chain::plugin)
            .add_plugins(events::plugin)
            .add_plugins(pirate::plugin)
            // Add resources
            .init_resource::<GameConfig>()
            .init_resource::<WaveState>()

            // Add startup systems
            .add_systems(Startup, setup_game)

            // Add gameplay systems
            .add_systems(Update, (
                pathfinding_system,
                oxygen_drain_system,
                death_system,
                goal_reached_system,
                chain_placement_system,
                ui_update_system,
                wave_control_system,
                game_over_system,
            ));
    }
}

fn setup_game(
    mut commands: Commands,
    game_config: Res<GameConfig>,
    asset_server: Res<AssetServer>,
) {
    // Spawn spawner
    commands.spawn((
        Spawner,
        SpawnTimer(Timer::from_seconds(game_config.spawn_interval, TimerMode::Repeating)),
        Transform::default(),
    ));

    // TODO: Spawn initial grid of tiles

    // Spawn gold bars
    spawn_gold_bars(&mut commands, &asset_server);

    // TODO: Setup UI
}