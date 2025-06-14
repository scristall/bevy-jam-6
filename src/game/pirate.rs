use bevy::prelude::*;

use crate::game::chain::{ChainSegment, Obstacle};
use crate::game::events::{
    GameOver, GoldBarCollected, GoldBarDropped, GoldBarLost, PirateDeath, WaveComplete, WaveStarted,
};
use crate::game::game_state::GameState;
use crate::game::goldbar::Gold;
use crate::game::modifiers::{Sticky, Tree};
use crate::game::oxygen::Oxygen;
use crate::game::tile::{GRID_HEIGHT, GRID_WIDTH, GRID_X_START, GRID_Y_START, TILE_SIZE, Tile};

use grid_pathfinding::PathingGrid;
use grid_util::grid::Grid;
use grid_util::point::Point;

pub const BOAT_POINT: Point = Point { x: 0, y: 5 };
pub const HOLD_POINT: Point = Point { x: 26, y: 5 };

const SPAWN_INTERVAL: f32 = 2.0;

#[derive(PartialEq)]
pub enum PirateState {
    PathingGold,
    PathingExit,
}

#[derive(Component)]
pub struct MovementSpeed(pub f32);

#[derive(Component)]
pub struct Spawner;

#[derive(Component)]
pub struct SpawnTimer(pub Timer);

#[derive(Component)]
pub struct Pirate {
    state: PirateState,
    carrying_gold: bool,
    marked_for_despawn: bool,
}

#[derive(Component)]
pub struct WaveState {
    pub pirates_per_wave: u32,
    pub pirates_spawned: u32,
}

#[derive(Resource, Default)]
pub struct WaveNumber(pub u32);

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

fn find_closest_point_idx(points: &[Point], location: Vec2) -> usize {
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

    closest_idx
}

fn find_closest_gold(
    gold_tiles: Query<(Entity, &Transform), (Without<Pirate>, With<Gold>)>,
    location: Vec2,
) -> Option<(Entity, Transform)> {
    let mut closest_distance: f32 = 100000000000.0;
    let mut result: Option<(Entity, Transform)> = None;

    for (entity, transform) in gold_tiles.iter() {
        let distance: f32 = (transform.translation.xy() - location).length();
        if distance < closest_distance {
            closest_distance = distance;
            result = Some((entity, *transform));
        }
    }

    result
}

pub fn get_pathing_grid(chain_segs: Query<&Obstacle>) -> PathingGrid {
    let mut pathing_grid: PathingGrid = PathingGrid::new(29, 11, false);
    pathing_grid.allow_diagonal_move = false;

    for chain_seg in chain_segs.iter() {
        pathing_grid.set(chain_seg.tile.x as usize, chain_seg.tile.y as usize, true);
    }

    // set the back wall
    for y in 0..GRID_HEIGHT {
        pathing_grid.set(GRID_WIDTH as usize, y as usize, true);
    }

    // but leave a gap for the pirates to exit
    pathing_grid.set(GRID_WIDTH as usize, HOLD_POINT.y as usize, false);

    pathing_grid.generate_components();

    pathing_grid
}

fn pirate_movement_system(
    time: Res<Time>,
    mut pirates: Query<(&mut Pirate, &mut Transform, &MovementSpeed, Option<&Sticky>)>,
    q_obstacles: Query<&Obstacle>,
    gold_tiles: Query<(Entity, &Transform), (Without<Pirate>, With<Gold>)>,
    mut event_gold_picked_up: EventWriter<GoldBarCollected>,
    mut event_gold_lost: EventWriter<GoldBarLost>,
) {
    let pathing_grid = get_pathing_grid(q_obstacles);

    for (mut pirate, mut transform, speed, sticky) in pirates.iter_mut() {
        let pirate_location = transform.translation.xy();
        let pirate_point = vec_to_grid_coord(&pirate_location);

        let nearest_gold_point = match find_closest_gold(gold_tiles, pirate_location) {
            Some((entity, transform)) => {
                let nearest_gold_location = transform.translation.xy();
                let nearest_gold_point = vec_to_grid_coord(&nearest_gold_location);

                if !pirate.carrying_gold && pirate_location.distance(nearest_gold_location) < 2.0 {
                    pirate.state = PirateState::PathingExit;
                    let tile = Tile {
                        x: nearest_gold_point.x,
                        y: nearest_gold_point.y,
                    };
                    event_gold_picked_up.write(GoldBarCollected { tile, entity });
                    pirate.carrying_gold = true;
                }

                nearest_gold_point
            }
            None => {
                pirate.state = PirateState::PathingExit;
                BOAT_POINT
            }
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

        let speed = if sticky.is_some() {
            speed.0 * 0.5
        } else {
            speed.0
        };

        let mut direction_vec: Vec2 = target_vec - pirate_location;
        let distance = direction_vec.length();
        let travel: f32 = speed * time.delta().as_secs_f32();
        direction_vec = direction_vec.normalize() * travel;
        if distance < travel {
            transform.translation.x = target_vec.x;
            transform.translation.y = target_vec.y;
        } else {
            transform.translation.x += direction_vec.x;
            transform.translation.y += direction_vec.y;
        }

        let new_location = transform.translation.xy();
        let new_point = vec_to_grid_coord(&new_location);

        if pirate.state == PirateState::PathingExit && new_point == BOAT_POINT {
            if pirate.carrying_gold {
                event_gold_lost.write(GoldBarLost);
            }
            pirate.marked_for_despawn = true;
        }
    }
}

pub fn pirate_spawn_system(
    mut commands: Commands,
    time: Res<Time>,
    mut spawners: Query<(Entity, &mut SpawnTimer, &mut WaveState), With<Spawner>>,
    mut wave_number: ResMut<WaveNumber>,
    asset_server: Res<AssetServer>,
) {
    for (entity, mut timer, mut wave_state) in spawners.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.just_finished() && wave_state.pirates_spawned < wave_state.pirates_per_wave {
            let y_coord: f32 = (2.6 - wave_state.pirates_spawned as f32) * 100.0;

            let movement_speed = wave_number.0 as f32 * 50.0 + 200.0;
            let oxygen_level = wave_number.0 as f32 * 10.0 + 100.0;

            commands.spawn((
                Pirate {
                    state: PirateState::PathingGold,
                    carrying_gold: false,
                    marked_for_despawn: false,
                },
                Sprite {
                    image: asset_server.load("images/pirate.png"),
                    ..default()
                },
                Transform::from_xyz(GRID_X_START, y_coord, 4.0).with_scale(vec3(0.5, 0.5, 0.5)),
                MovementSpeed(movement_speed),
                Oxygen(oxygen_level),
            ));

            wave_state.pirates_spawned += 1;
            if wave_state.pirates_spawned == wave_state.pirates_per_wave {
                commands.entity(entity).despawn();
                wave_number.0 += 1;
            }
        }
    }
}

fn pirate_oxygen_system(
    time: Res<Time>,
    mut q_pirates: Query<(&mut Pirate, &mut Oxygen, &Transform)>,
    q_chain: Query<&Transform, With<ChainSegment>>,
    q_trees: Query<&Transform, With<Tree>>,
    mut evw_pirate_death: EventWriter<PirateDeath>,
    mut evw_gold_dropped: EventWriter<GoldBarDropped>,
) {
    for (mut pirate, mut oxygen, transform) in q_pirates.iter_mut() {
        for tree in q_trees.iter() {
            let tree_pos = tree.translation.xy();
            let pirate_pos = transform.translation.xy();
            let dx = (tree_pos.x - pirate_pos.x).abs();
            let dy = (tree_pos.y - pirate_pos.y).abs();

            // add a little buffer
            if dx <= TILE_SIZE * 1.2 && dy <= TILE_SIZE * 1.2 {
                oxygen.0 += 30.0 * time.delta().as_secs_f32();
            }
        }
        for chain_seg in q_chain.iter() {
            // if pirate is next to a chain, deplete oxygen
            let chain_pos = chain_seg.translation.xy();
            let pirate_pos = transform.translation.xy();
            let dx = (chain_pos.x - pirate_pos.x).abs();
            let dy = (chain_pos.y - pirate_pos.y).abs();

            // add a little buffer
            if dx <= TILE_SIZE * 1.2 && dy <= TILE_SIZE * 1.2 {
                let pirate_was_alive = oxygen.0 > 0.0;
                oxygen.0 -= 10.0 * time.delta().as_secs_f32();
                if pirate_was_alive && oxygen.0 <= 0.0 {
                    pirate.marked_for_despawn = true;
                    evw_pirate_death.write(PirateDeath {});
                    if pirate.carrying_gold {
                        let pirate_point = vec_to_grid_coord(&pirate_pos);
                        evw_gold_dropped.write(GoldBarDropped {
                            tile: Tile {
                                x: pirate_point.x,
                                y: pirate_point.y,
                            },
                        });
                    }
                    break;
                }
            }
        }
    }
}

fn end_wave_system(
    q_pirates: Query<Entity, With<Pirate>>,
    q_spawner: Query<Entity, With<Spawner>>,
    q_gold_tiles: Query<Entity, With<Gold>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut evw_wave_complete: EventWriter<WaveComplete>,
    mut evw_game_over: EventWriter<GameOver>,
) {
    if q_pirates.iter().count() == 0 && q_spawner.iter().count() == 0 {
        if q_gold_tiles.iter().count() == 0 {
            next_state.set(GameState::GameOver);
            evw_game_over.write(GameOver);
        } else {
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

fn despawn_pirates(mut commands: Commands, mut q_pirates: Query<(&Pirate, Entity)>) {
    for (pirate, entity) in q_pirates.iter_mut() {
        if pirate.marked_for_despawn {
            commands.entity(entity).despawn();
        }
    }
}

pub fn plugin(app: &mut App) {
    app.init_resource::<WaveNumber>();

    app.add_systems(
        Update,
        (
            spawn_setup,
            pirate_spawn_system,
            pirate_movement_system,
            pirate_oxygen_system,
        )
            .run_if(in_state(GameState::WaveInProgress)),
    );

    // We have to run these here to ensure that:
    // 1. The spawner is created prior to checking if the wave is complete
    // 2. The pirates are despawned safely after any other updates
    app.add_systems(
        PostUpdate,
        (end_wave_system, despawn_pirates).run_if(in_state(GameState::WaveInProgress)),
    );
}
