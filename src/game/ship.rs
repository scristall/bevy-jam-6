use crate::game::events::{WaveComplete, WaveStarted};
use bevy::prelude::*;

pub const SHIP_SPEED: f32 = 100.0;
pub const SHIP_MID_Y: f32 = 0.0;
pub const SHIP_START_Y: f32 = 850.0;
pub const SHIP_OUT_Y: f32 = -850.0;
pub const SHIP_X: f32 = -800.0;

#[derive(Component)]
pub struct Ship {
    pub speed: f32,
    pub target_y: f32,
}

pub fn spawn_ship(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Ship {
            speed: SHIP_SPEED,
            target_y: SHIP_START_Y,
        },
        Sprite {
            image: asset_server.load("images/ship.png"),
            ..default()
        },
        Transform::from_xyz(SHIP_X, SHIP_START_Y, 1.0),
    ));
}

pub fn move_ship(mut ship_query: Query<(&mut Transform, &Ship)>, time: Res<Time>) {
    for (mut transform, ship) in ship_query.iter_mut() {
        let current_y = transform.translation.y;
        let target_y = ship.target_y;

        if current_y > target_y {
            let movement = ship.speed * time.delta().as_secs_f32();
            transform.translation.y -= movement.min(current_y - target_y);
        }
    }
}

pub fn move_ship_in(
    mut ship_query: Query<(&mut Transform, &mut Ship)>,
    mut evr_wave_started: EventReader<WaveStarted>,
) {
    for _ in evr_wave_started.read() {
        for (mut transform, mut ship) in ship_query.iter_mut() {
            transform.translation.y = SHIP_START_Y;
            ship.target_y = SHIP_MID_Y;
        }
    }
}

pub fn move_ship_out(
    mut ship_query: Query<&mut Ship>,
    mut evr_wave_complete: EventReader<WaveComplete>,
) {
    for _ in evr_wave_complete.read() {
        for mut ship in ship_query.iter_mut() {
            ship.target_y = SHIP_OUT_Y;
        }
    }
}

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_ship);
    app.add_systems(Update, (move_ship, move_ship_in, move_ship_out));
}
