use bevy::prelude::*;

#[derive(Resource)]
pub struct MousePos(pub Vec2);

impl Default for MousePos {
    fn default() -> Self {
        Self(Vec2::ZERO)
    }
}

fn mouse_move(
    mut evr_cursor: EventReader<CursorMoved>,
    mut mouse_pos: ResMut<MousePos>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
) {
    for event in evr_cursor.read() {
        let cursor = event.position;
        let camera = q_camera.single().unwrap();

        let world_cursor_pos = camera.0.viewport_to_world_2d(&camera.1, cursor).unwrap();
        mouse_pos.0 = world_cursor_pos;
    }
}

pub fn plugin(app: &mut App) {
    app.init_resource::<MousePos>()
        .add_systems(Update, mouse_move);
}
