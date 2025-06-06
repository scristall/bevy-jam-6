mod components;
mod systems;

use bevy::prelude::*;
use components::*;
use systems::*;

mod background;
mod camera;
mod chain;
mod controls;
mod events;
mod game_state;
mod goldbar;
mod mouse;
mod tile;
mod goldbar_text;

use crate::game::game_state::GameState;
use crate::game::goldbar::{Gold, spawn_gold_bars, plugin as goldbar_plugin};
use crate::game::goldbar_text::{GoldBarTextPlugin, GoldAmount};

use grid_pathfinding::PathingGrid;
use grid_util::grid::Grid;
use grid_util::point::Point;

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
            .add_plugins(GoldBarTextPlugin)
            .add_plugins(goldbar_plugin)
            // Add resources
            .init_resource::<GameConfig>()
            .init_resource::<WaveState>()
            // Add startup systems
            .add_systems(Startup, setup_game)
            // Add gameplay systems
            .add_systems(
                Update,
                (
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
                ),
            );
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

    let mut pathing_grid: PathingGrid = PathingGrid::new(25, 11, false);
    pathing_grid.allow_diagonal_move = false;
    pathing_grid.set(5, 5, true);
    pathing_grid.set(5, 6, true);
    pathing_grid.set(5, 7, true);
    pathing_grid.generate_components();
    println!("{}", pathing_grid);
    let start = Point::new(0, 6);
    let end = Point::new(24, 6);
    let path: Option<Vec<Point>> = pathing_grid.get_path_single_goal(start, end, false);

    match path {
        Some(val) => {
            println!("Path:");
            for point in val {
                println!("{:?}", point);
            }
        }
        None => println!("No Path"),
    }

    // TODO: Spawn initial grid of tiles

    // Spawn gold bars
    spawn_gold_bars(&mut commands, &asset_server, gold_amount);

    // TODO: Setup UI
}
