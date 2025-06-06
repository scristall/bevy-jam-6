use bevy::prelude::*;
use crate::game::chain::ChainSegment;
use crate::game::components::*;

use grid_pathfinding::PathingGrid;
use grid_util::grid::Grid;
use grid_util::point::Point;

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

// TODO: this hurts me
const TILE_SIZE: f32 = 53.0;
const GRID_X_START: f32 = -620.0;
const GRID_Y_START: f32 = -200.0;

fn grid_coord_to_transform(p: &Point) -> Vec2 {
    Vec2::new(
        GRID_X_START + p.x as f32 * TILE_SIZE,
        GRID_Y_START + p.y as f32 * TILE_SIZE
    )
}

pub fn pathfinding_system(
    mut pirates: Query<(&Transform, &Path, &CurrentTarget), With<Pirate>>,
    tiles: Query<(&Position, &TileType, &TileProperties), With<Tile>>,
) {
    for (transform, path, target) in pirates.iter() {
        // TODO: Implement A* pathfinding
    }
}

fn find_closest_point_idx(points: &Vec<Point>, location: Vec2) -> usize {
    let mut closest_distance: f32 = 100000000000.0;
    let mut closest_idx: usize = 0;
    for (i, point) in points.iter().enumerate() {
        let point_vec: Vec2 = grid_coord_to_transform(point);

        let distance: f32 = (location - point_vec).length();
        if distance < closest_distance {
            closest_distance = distance;
            closest_idx = i;
        }
    }
    return closest_idx;
}

pub fn pirate_movement_system(
    time: Res<Time>,
    mut pirates: Query<(&mut Transform, &MovementSpeed, &CurrentTarget), With<Pirate>>,
    chain_segs: Query<&ChainSegment>,
) {

    let mut pathing_grid: PathingGrid = PathingGrid::new(25, 11, false);
    pathing_grid.allow_diagonal_move = false;


    for chain_seg in chain_segs.iter() {
        pathing_grid.set(chain_seg.0.x as usize, chain_seg.0.y as usize, true);
    }

    pathing_grid.generate_components();

    let start = Point::new(0, 5);
    let end = Point::new(24, 5);
    let path: Option<Vec<Point>> = pathing_grid
        .get_path_single_goal(start, end, false);


    for (mut transform, speed, target) in pirates.iter_mut() {

        match path {
            Some(ref val) => {
                let closest_point_index: usize = find_closest_point_idx(
                    val, 
                    transform.translation.xy()
                );

                let mut target_point = val[val.len() - 1];
                if closest_point_index < val.len() - 1 {
                    target_point = val[closest_point_index+1];
                }
            
                let target_vec: Vec2 = grid_coord_to_transform(&target_point);

                let mut direction_vec: Vec2 = target_vec - transform.translation.xy();
                let distance: f32 = speed.0 * time.delta().as_secs_f32();
                direction_vec = direction_vec.normalize() * distance;
                transform.translation.x += direction_vec.x;
                transform.translation.y += direction_vec.y;
            },
            None => {
                let mut direction_vec: Vec2 = target.0 - transform.translation.xy();
                let distance: f32 = speed.0 * time.delta().as_secs_f32();
                direction_vec = direction_vec.normalize() * distance;
                transform.translation.x += direction_vec.x;
                transform.translation.y += direction_vec.y;
            }
        }
        
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