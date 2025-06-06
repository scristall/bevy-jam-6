mod components;
mod systems;

use bevy::prelude::*;
use components::*;
use systems::*;

mod camera;
mod background;
mod tile;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(camera::plugin)
            .add_plugins(background::plugin)
            .add_plugins(tile::plugin)
            // Add resources
            .init_resource::<GameConfig>()
            .init_resource::<WaveState>()
            
            // Add startup systems
            .add_systems(Startup, setup_game)
            
            // Add gameplay systems
            .add_systems(Update, (
                pirate_spawn_system,
                pathfinding_system,
                pirate_movement_system,
                oxygen_drain_system,
                death_system,
                goal_reached_system,
                chain_placement_system,
                ui_update_system,
                wave_control_system,
                game_over_system,
            ).run_if(in_state(GameState::Playing)));
    }
}

fn setup_game(
    mut commands: Commands,
    game_config: Res<GameConfig>,
) {
    // Spawn spawner
    commands.spawn((
        Spawner,
        SpawnTimer(Timer::from_seconds(game_config.spawn_interval, TimerMode::Repeating)),
        Transform::default(),
    ));
    
    // TODO: Spawn initial grid of tiles
    // TODO: Spawn gold
    // TODO: Setup UI
} 