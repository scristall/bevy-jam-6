use bevy::prelude::*;

use crate::game::{
    chain::Obstacle,
    events::{GlueSpawned, TreeSpawned},
    tile::{TILE_SIZE, Tile},
};

#[derive(Component)]
pub struct GluePuddle;

#[derive(Component)]
pub struct Tree;

#[derive(Component)]
pub struct Sticky {
    pub duration: f32,
}

pub fn sticky_system(
    time: Res<Time>,
    mut commands: Commands,
    mut q_sticky_pirates: Query<(Entity, &mut Sticky)>,
) {
    for (entity, mut sticky) in q_sticky_pirates.iter_mut() {
        sticky.duration -= time.delta().as_secs_f32();
        if sticky.duration <= 0.0 {
            commands.entity(entity).remove::<Sticky>();
        }
    }
}

pub fn glue_puddle_system(
    mut commands: Commands,
    mut q_non_sticky_pirates: Query<(Entity, &Transform), Without<Sticky>>,
    mut q_sticky_pirates: Query<(&Transform, &mut Sticky), With<Sticky>>,
    q_puddles: Query<&Transform, With<GluePuddle>>,
) {
    // first do pirates without sticky
    for (entity, transform) in q_non_sticky_pirates.iter_mut() {
        for puddle in q_puddles.iter() {
            if transform.translation.distance(puddle.translation) < TILE_SIZE {
                commands.entity(entity).insert(Sticky { duration: 2.0 });
            }
        }
    }

    // for pirates with sticky, refresh their duration
    for (transform, mut sticky) in q_sticky_pirates.iter_mut() {
        for puddle in q_puddles.iter() {
            if transform.translation.distance(puddle.translation) < TILE_SIZE {
                sticky.duration = 2.0;
            }
        }
    }
}

fn spawn_glue(
    mut commands: Commands,
    mut evr_glue_spawned: EventReader<GlueSpawned>,
    asset_server: Res<AssetServer>,
) {
    for event in evr_glue_spawned.read() {
        commands.spawn((
            GluePuddle,
            Sprite {
                custom_size: Some(Vec2::splat(TILE_SIZE)),
                ..Sprite::from_image(asset_server.load("images/glue.png"))
            },
            Tile::grid_coord_to_transform(&event.tile, 3.0),
        ));
    }
}

fn spawn_tree(
    mut commands: Commands,
    mut evr_tree_spawned: EventReader<TreeSpawned>,
    asset_server: Res<AssetServer>,
) {
    for event in evr_tree_spawned.read() {
        let mut transform = Tile::grid_coord_to_transform(&event.tile, 4.0);
        transform.translation.y += TILE_SIZE * 0.5;
        commands.spawn((
            Tree,
            Obstacle { tile: event.tile },
            Sprite {
                custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE * 2.0)),
                ..Sprite::from_image(asset_server.load("images/tree.png"))
            },
            transform,
        ));
    }
}

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (sticky_system, glue_puddle_system, spawn_glue, spawn_tree),
    );
}
