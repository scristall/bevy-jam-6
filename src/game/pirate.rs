use bevy::prelude::*;

use crate::game::chain::ChainSegment;
use crate::game::components::*;
use crate::game::game_state::GameState;
use crate::game::goldbar::Gold;
use crate::game::tile::{GRID_X_START, GRID_Y_START, TILE_SIZE, Tile};
use crate::game::events::{TileEvent, GoldBarCollected, GoldBarDropped};



use grid_pathfinding::PathingGrid;
use grid_util::grid::Grid;
use grid_util::point::Point;

pub enum PirateState {
    PathingGold,
    PathingExit,
}

#[derive(Component)]
pub struct Pirate {
    state: PirateState,
}

impl Pirate {}

fn grid_coord_to_transform(p: &Point) -> Vec2 {
    Vec2::new(
        GRID_X_START + p.x as f32 * TILE_SIZE,
        GRID_Y_START + p.y as f32 * TILE_SIZE,
    )
}

fn vec_to_grid_coord(v: &Vec2) -> Point {
    Point {
        x: ((v.x - GRID_X_START) / TILE_SIZE) as i32,
        y: ((v.y - GRID_Y_START) / TILE_SIZE) as i32,
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

fn find_closest_gold(
    gold_tiles: Query<(Entity, &Transform, &Position), Without<Pirate>>,
    location: Vec2,
) -> Option<(Entity, Transform)> {
    let mut closest_distance: f32 = 100000000000.0;
    let mut result: Option<(Entity, Transform)> = None;
    for (i, (entity, transform, position)) in gold_tiles.iter().enumerate() {
        let distance: f32 = (transform.translation.xy() - location).length();
        if distance < closest_distance {
            closest_distance = distance;
            result = Some((entity, *transform));
        }
    }
    return result;
}

const BOAT_POINT: Point = Point { x: 0, y: 5 };


fn pirate_movement_system(
    time: Res<Time>,
    mut pirates: Query<(&mut Pirate, &mut Transform, &MovementSpeed)>,
    chain_segs: Query<&ChainSegment>,
    gold_tiles: Query<(Entity, &Transform, &Position), Without<Pirate>>,
    mut event_gold_picked_up: EventWriter<GoldBarCollected>,
) {
    let mut pathing_grid: PathingGrid = PathingGrid::new(29, 11, false);
    pathing_grid.allow_diagonal_move = false;

    for chain_seg in chain_segs.iter() {
        pathing_grid.set(chain_seg.0.x as usize, chain_seg.0.y as usize, true);
    }

    pathing_grid.set(25, 0, true);
    pathing_grid.set(25, 1, true);
    pathing_grid.set(25, 2, true);
    pathing_grid.set(25, 3, true);
    pathing_grid.set(25, 4, true);
    pathing_grid.set(25, 6, true);
    pathing_grid.set(25, 7, true);
    pathing_grid.set(25, 8, true);
    pathing_grid.set(25, 9, true);
    pathing_grid.set(25, 10, true);

    pathing_grid.generate_components();

    for (mut pirate, mut transform, speed) in pirates.iter_mut() {
        let pirate_location = transform.translation.xy();
        let pirate_point = vec_to_grid_coord(&pirate_location);

        let nearest_gold_point = match find_closest_gold(gold_tiles, pirate_location) {
            Some((entity, transform)) => {
                let nearest_gold_location = transform.translation.xy();
                let nearest_gold_point = vec_to_grid_coord(&nearest_gold_location);

                if pirate_location.distance(nearest_gold_location) < 2.0 {
                    pirate.state = PirateState::PathingExit;
                    let tile = Tile { x: nearest_gold_point.x, y: nearest_gold_point.y };
                    event_gold_picked_up.write(GoldBarCollected{
                        tile: tile,
                        entity: entity,
                    });
                }

                nearest_gold_point
            }
            None => BOAT_POINT
        };

        let start: Point;
        let end: Point;
        match pirate.state {
            PirateState::PathingGold => {
                start = pirate_point;
                end = nearest_gold_point;
            }
            PirateState::PathingExit => {
                start = pirate_point;
                end = BOAT_POINT;
            }
        }


        let path: Option<Vec<Point>> = pathing_grid.get_path_single_goal(start, end, false);

        let target_vec: Vec2 = match &path {
            Some(val) => {
                let closest_point_index: usize =
                    find_closest_point_idx(val, transform.translation.xy());

                let mut target_point = val[val.len() - 1];
                if closest_point_index < val.len() - 1 {
                    target_point = val[closest_point_index + 1];
                }

                grid_coord_to_transform(&target_point)
            }
            None => grid_coord_to_transform(&end)
        };

        let mut direction_vec: Vec2 = target_vec - pirate_location;
        let distance: f32 = speed.0 * time.delta().as_secs_f32();
        direction_vec = direction_vec.normalize() * distance;
        transform.translation.x += direction_vec.x;
        transform.translation.y += direction_vec.y;
    }
}

pub fn pirate_spawn_system(
    mut commands: Commands,
    time: Res<Time>,
    mut spawners: Query<(&mut SpawnTimer, &Transform), With<Spawner>>,
    game_config: Res<GameConfig>,
    mut wave_state: ResMut<WaveState>,
    asset_server: Res<AssetServer>,
) {
    for (mut timer, transform) in spawners.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.just_finished() && wave_state.pirates_spawned < wave_state.pirates_per_wave {
            let y_coord: f32 = (wave_state.pirates_spawned as f32 - 2.0) * 100.0;

            commands.spawn((
                Pirate {
                    state: PirateState::PathingGold,
                },
                Sprite {
                    image: asset_server.load("images/pirate.png"),
                    ..default()
                },
                Transform::from_xyz(-600.0, y_coord, 2.0).with_scale(vec3(0.5, 0.5, 0.5)),
                MovementSpeed(200.0),
                Oxygen(100.0),
            ));

            wave_state.pirates_spawned += 1;
        }
    }
}

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (pirate_spawn_system, pirate_movement_system).run_if(in_state(GameState::WaveInProgress)),
    );
}
