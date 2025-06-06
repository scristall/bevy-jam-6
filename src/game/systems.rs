use bevy::prelude::*;
use crate::game::chain::ChainSegment;
use crate::game::components::*;

// Core Gameplay Systems
pub fn pirate_spawn_system(
    mut commands: Commands,
    time: Res<Time>,
    mut spawners: Query<(&mut SpawnTimer, &Transform), With<Spawner>>,
    game_config: Res<GameConfig>,
    mut wave_state: ResMut<WaveState>,
    asset_server: Res<AssetServer>
) {
    for (mut timer, transform) in spawners.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.just_finished() && wave_state.pirates_spawned < wave_state.pirates_per_wave {
            println!("spawned a pirate");
            
            let y_coord: f32 = (wave_state.pirates_spawned as f32 - 2.0) * 100.0;

            commands.spawn((
                Pirate,
                Sprite {
                    image: asset_server.load("images/pirate.png"),
                    ..default()
                },
                Transform::from_xyz(-600.0, y_coord, 2.0).with_scale(vec3(0.5, 0.5, 0.5)),
                MovementSpeed(100.0),
                CurrentTarget(vec2(800.0, 0.0))
            ));

            wave_state.pirates_spawned += 1;

        }
    }
}

pub fn pathfinding_system(
    mut pirates: Query<(&Transform, &Path, &CurrentTarget), With<Pirate>>,
    tiles: Query<(&Position, &TileType, &TileProperties), With<Tile>>,
) {
    for (transform, path, target) in pirates.iter() {
        // TODO: Implement A* pathfinding
    }
}

pub fn oxygen_drain_system(
    time: Res<Time>,
    mut pirates: Query<(&mut Oxygen, &Transform), With<Pirate>>,
    tiles: Query<(&Transform, &TileProperties), With<Tile>>,
) {
    for (mut oxygen, pirate_transform) in pirates.iter_mut() {
        // TODO: Implement oxygen drain based on current tile
    }
}

pub fn death_system(
    mut commands: Commands,
    pirates: Query<(Entity, &Oxygen), With<Pirate>>,
) {
    for (entity, oxygen) in pirates.iter() {
        if oxygen.0 <= 0.0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn goal_reached_system(
    pirates: Query<&Transform, With<Pirate>>,
    gold: Query<&Transform, With<Gold>>,
) {
    // TODO: Implement goal reached check
}

// Player Interaction Systems
pub fn chain_placement_system(
    mut commands: Commands,
    windows: Query<&Window>,
    camera: Query<(&Camera, &GlobalTransform)>,
    mouse: Res<ButtonInput<MouseButton>>,
    tiles: Query<(Entity, &Position, &TileType), With<Tile>>,
) {
    // TODO: Implement chain placement on mouse click
}

pub fn ui_update_system(
    mut oxygen_bars: Query<&mut Text, (With<OxygenBar>, Without<WaveText>)>,
    mut wave_text: Query<&mut Text, (With<WaveText>, Without<OxygenBar>)>,
    pirates: Query<&Oxygen, With<Pirate>>,
    wave_state: Res<WaveState>,
) {
    // TODO: Implement UI updates
}

// Wave + Timing Systems
pub fn wave_control_system(
    mut wave_state: ResMut<WaveState>,
    pirates: Query<(), With<Pirate>>,
) {
    if pirates.is_empty() && wave_state.pirates_spawned >= wave_state.pirates_per_wave {
        // TODO: Implement wave progression
    }
}

pub fn game_over_system(
    mut commands: Commands,
) {
}