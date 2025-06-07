use bevy::prelude::*;

use crate::game::events::{TileEvent, TileMouseDown, TileMouseMove, TileMouseUp};
use crate::game::mouse::MousePos;

pub const TILE_SIZE: f32 = 53.0;

pub const GRID_X_START: f32 = -620.0;
pub const GRID_Y_START: f32 = -200.0;

pub const GRID_WIDTH: i32 = 25;
pub const GRID_HEIGHT: i32 = 11;

#[derive(Component, Copy, Clone, Debug, PartialEq)]
pub struct Tile {
    pub x: i32,
    pub y: i32,
}

#[derive(Component, Debug)]
pub struct BackgroundTile {
    pub is_hovered: bool,
}

impl Tile {
    pub fn contains(&self, cursor: Vec2, transform: &GlobalTransform) -> bool {
        let pos = transform.translation().truncate();
        let size = Vec2::splat(TILE_SIZE);
        let min = pos - size / 2.0;
        let max = pos + size / 2.0;
        min.x < cursor.x && cursor.x <= max.x && min.y < cursor.y && cursor.y <= max.y
    }

    // check for orthogonally adjacent tiles
    pub fn is_adjacent(&self, other: &Tile) -> bool {
        let dx = (self.x - other.x).abs();
        let dy = (self.y - other.y).abs();
        (dx == 1 && dy == 0) || (dx == 0 && dy == 1)
    }

    pub fn grid_coord_to_transform(&self, z: f32) -> Transform {
        Transform::from_translation(Vec3::new(
            GRID_X_START + self.x as f32 * TILE_SIZE,
            GRID_Y_START + self.y as f32 * TILE_SIZE,
            z,
        ))
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let width = GRID_WIDTH;
    let height = GRID_HEIGHT;

    let rect = Rectangle::new(TILE_SIZE, TILE_SIZE);
    let color = Color::linear_rgba(1.0, 1.0, 1.0, 1.0);

    // leave a gap for the pirates to enter
    for x in 1..width {
        for y in 0..height {
            let tile = Tile { x, y };
            commands.spawn((
                tile,
                BackgroundTile { is_hovered: false },
                Mesh2d(meshes.add(rect)),
                MeshMaterial2d(materials.add(color)),
                tile.grid_coord_to_transform(1.0),
            ));
        }
    }
}

fn mouse_events(
    mut q_tile: Query<
        (
            Entity,
            &Tile,
            &mut BackgroundTile,
            &MeshMaterial2d<ColorMaterial>,
            &GlobalTransform,
        ),
        With<BackgroundTile>,
    >,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mouse_pos: Res<MousePos>,
    mut evr_tile_mouse_down: EventWriter<TileMouseDown>,
    mut evr_tile_mouse_up: EventWriter<TileMouseUp>,
    mut evr_tile_mouse_move: EventWriter<TileMouseMove>,
) {
    for (_, tile, mut background_tile, material, transform) in q_tile.iter_mut() {
        let Some(material) = materials.get_mut(material) else {
            continue;
        };

        if !tile.contains(mouse_pos.0, transform) {
            material.color = Color::linear_rgba(1.0, 1.0, 1.0, 1.0);
            background_tile.is_hovered = false;
            continue;
        }

        if mouse_button.just_pressed(MouseButton::Left) {
            evr_tile_mouse_down.write(TileMouseDown(TileEvent { tile: *tile }));
        } else if mouse_button.just_released(MouseButton::Left) {
            evr_tile_mouse_up.write(TileMouseUp(TileEvent { tile: *tile }));
        }

        if background_tile.is_hovered {
            continue;
        }

        background_tile.is_hovered = true;

        material.color = Color::linear_rgba(0.0, 1.0, 0.0, 1.0);
        evr_tile_mouse_move.write(TileMouseMove(TileEvent { tile: *tile }));
    }
}

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, setup);
    app.add_systems(Update, mouse_events);
}
