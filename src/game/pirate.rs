use bevy::prelude::*;

use crate::game::chain::ChainSegment;
use crate::game::components::Position;
use crate::game::events::{GoldBarCollected, PirateDeath, WaveComplete, WaveStarted};
use crate::game::game_state::GameState;
use crate::game::oxygen::Oxygen;
use crate::game::tile::{GRID_X_START, GRID_Y_START, TILE_SIZE, Tile};

use grid_pathfinding::PathingGrid;
use grid_util::grid::Grid;
use grid_util::point::Point;

const SPAWN_INTERVAL: f32 = 2.0;

pub enum PirateState {
    PathingGold,
    PathingExit,
}

#[derive(Component)]
pub struct MovementSpeed(pub f32);

#[derive(Component)]
pub struct Path(pub Vec<Vec2>);

#[derive(Component)]
pub struct CurrentTarget(pub Vec2);

#[derive(Component)]
pub struct Spawner;

#[derive(Component)]
pub struct SpawnTimer(pub Timer);

#[derive(Component)]
pub struct Pirate {
    state: PirateState,
}

#[derive(Component)]
pub struct WaveState {
    pub pirates_per_wave: u32,
    pub pirates_spawned: u32,
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
        x: ((v.x - GRID_X_START + TILE_SIZE / 2.0) / TILE_SIZE) as i32,
        y: ((v.y - GRID_Y_START + TILE_SIZE / 2.0) / TILE_SIZE) as i32,
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
                    let tile = Tile {
                        x: nearest_gold_point.x,
                        y: nearest_gold_point.y,
                    };
                    event_gold_picked_up.write(GoldBarCollected {
                        tile: tile,
                        entity: entity,
                    });
                }

                nearest_gold_point
            }
            None => BOAT_POINT,
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
            None => grid_coord_to_transform(&end),
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
    mut spawners: Query<(Entity, &mut SpawnTimer, &mut WaveState), With<Spawner>>,
    asset_server: Res<AssetServer>,
) {
    for (entity, mut timer, mut wave_state) in spawners.iter_mut() {
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
                Transform::from_xyz(GRID_X_START, y_coord, 2.0).with_scale(vec3(0.5, 0.5, 0.5)),
                MovementSpeed(200.0),
                Oxygen(100.0),
            ));

            wave_state.pirates_spawned += 1;
            if wave_state.pirates_spawned == wave_state.pirates_per_wave {
                commands.entity(entity).despawn();
            }
        }
    }
}

fn pirate_touching_chain_system(
    time: Res<Time>,
    mut commands: Commands,
    mut q_pirates: Query<(&mut Oxygen, &Transform, Entity)>,
    q_chain: Query<(&ChainSegment, &Transform)>,
    mut event_pirate_death: EventWriter<PirateDeath>,
) {
    for (mut oxygen, transform, entity) in q_pirates.iter_mut() {
        for chain_seg in q_chain.iter() {
            // if pirate is next to a chain, deplete oxygen
            let chain_point = chain_seg.1.translation.xy();
            let pirate_point = transform.translation.xy();
            let dx = (chain_point.x - pirate_point.x).abs() as f32;
            let dy = (chain_point.y - pirate_point.y).abs() as f32;

            // add a little buffer
            if dx <= TILE_SIZE * 1.2 && dy <= TILE_SIZE * 1.2 {
                oxygen.0 -= 10.0 * time.delta().as_secs_f32();
                if oxygen.0 <= 0.0 {
                    event_pirate_death.write(PirateDeath { entity });
                    commands.entity(entity).despawn();
                    break;
                }
            }
        }
    }
}

fn end_wave_system(
    q_pirates: Query<Entity, With<Pirate>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut evr_pirate_death: EventReader<PirateDeath>,
    mut evw_wave_complete: EventWriter<WaveComplete>,
) {
    for _ in evr_pirate_death.read() {
        if q_pirates.iter().count() == 0 {
            next_state.set(GameState::Prize);
            evw_wave_complete.write(WaveComplete);
        }
    }
}

fn spawn_setup(mut commands: Commands, mut evr_wave_started: EventReader<WaveStarted>) {
    for _ in evr_wave_started.read() {
        // Spawn spawner
        commands.spawn((
            Spawner,
            SpawnTimer(Timer::from_seconds(SPAWN_INTERVAL, TimerMode::Repeating)),
            WaveState {
                pirates_per_wave: 5,
                pirates_spawned: 0,
            },
        ));
    }
}

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            spawn_setup,
            pirate_spawn_system,
            pirate_movement_system,
            pirate_touching_chain_system,
            end_wave_system,
        )
            .run_if(in_state(GameState::WaveInProgress)),
    );
}
