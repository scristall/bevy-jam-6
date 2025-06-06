use bevy::prelude::*;

pub const TILE_SIZE: f32 = 20.0;

#[derive(Component, Debug)]
pub struct Tile;

impl Tile {
    pub fn contains(&self, cursor: Vec2, transform: &GlobalTransform) -> bool {
        let pos = transform.translation().truncate();
        let size = Vec2::splat(TILE_SIZE);
        let min = pos - size / 2.0;
        let max = pos + size / 2.0;
        min.x < cursor.x && cursor.x <= max.x && min.y < cursor.y && cursor.y <= max.y
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let width = 47;
    let height = 10;
    let x_start = -440.0;
    let y_start =  -32.0;

    let rect = Rectangle::new(TILE_SIZE, TILE_SIZE);
    let color = Color::linear_rgba(0.0, 1.0, 0.0, 1.0);

    for x in 0..width {
        for y in 0..height {
            commands.spawn((
                Tile,
                Mesh2d(meshes.add(rect)),
                MeshMaterial2d(materials.add(color)),
                Transform::from_scale(Vec3::splat(TILE_SIZE)).with_translation(Vec3::new(
                    x_start + x as f32 * TILE_SIZE,
                    y_start + y as f32 * TILE_SIZE,
                    1.0,
                )),
            ));
        }
    }
}

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, setup);
}
