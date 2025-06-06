use bevy::prelude::*;

pub const TILE_SIZE: f32 = 53.0;

#[derive(Component, Debug)]
pub struct Tile {
    x: i32,
    y: i32,
}

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
    let width = 25;
    let height = 11;
    let x_start = -620.0;
    let y_start = -200.0;

    let rect = Rectangle::new(TILE_SIZE, TILE_SIZE);
    let color = Color::linear_rgba(1.0, 1.0, 1.0, 1.0);

    for x in 0..width {
        for y in 0..height {
            commands.spawn((
                Tile { x, y },
                Mesh2d(meshes.add(rect)),
                MeshMaterial2d(materials.add(color)),
                Transform::from_translation(Vec3::new(
                    x_start + x as f32 * TILE_SIZE,
                    y_start + y as f32 * TILE_SIZE,
                    1.0,
                )),
            ));
        }
    }
}

fn mouse_over(
    q_tile: Query<(&Tile, &MeshMaterial2d<ColorMaterial>, &GlobalTransform)>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut evr_cursor: EventReader<CursorMoved>,
) {
    for event in evr_cursor.read() {
        let cursor = event.position;
        let camera = q_camera.single().unwrap();

        let world_cursor_pos = camera.0.viewport_to_world_2d(&camera.1, cursor).unwrap();

        for (tile, material, transform) in q_tile.iter() {
                let Some(material) = materials.get_mut(material) else {
                    continue;
                };
            if tile.contains(world_cursor_pos, &transform) {
                material.color = Color::linear_rgba(0.0, 1.0, 0.0, 1.0);
            } else {
                material.color = Color::linear_rgba(1.0, 1.0, 1.0, 1.0);
            }
            
        }
    }
}

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, setup);
    app.add_systems(Update, mouse_over);
}
