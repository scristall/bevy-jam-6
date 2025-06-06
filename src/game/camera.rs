use bevy::{prelude::*, render::camera::ScalingMode};

pub const VERTICAL_RESOLUTION: f32 = 1080.0;
pub const HORIZONTAL_RESOLUTION: f32 = 1920.0;

#[derive(Component)]
pub struct MainCamera;

#[allow(clippy::field_reassign_with_default)]
fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Camera { ..default() },
        Projection::custom(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: VERTICAL_RESOLUTION,
            },
            ..OrthographicProjection::default_2d()
        }),
        MainCamera,
    ));
}

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, setup);
}
