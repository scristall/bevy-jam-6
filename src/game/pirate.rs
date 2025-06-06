use bevy::prelude::*;

use crate::game::components::*;
use crate::game::chain::ChainSegment;
use crate::game::tile::{TILE_SIZE, GRID_X_START, GRID_Y_START};

use grid_pathfinding::PathingGrid;
use grid_util::grid::Grid;
use grid_util::point::Point;

impl Pirate {

}

fn grid_coord_to_transform(p: &Point) -> Vec2 {
    Vec2::new(
        GRID_X_START + p.x as f32 * TILE_SIZE,
        GRID_Y_START + p.y as f32 * TILE_SIZE
    )
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

fn pirate_movement_system(
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



pub fn plugin(app: &mut App) {
    app.add_systems(Update, pirate_movement_system);
}
