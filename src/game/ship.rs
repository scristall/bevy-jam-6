use bevy::prelude::*;
use crate::game::game_state::GameState;

#[derive(Component)]
pub struct Ship {
    pub speed: f32,
    pub target_y: f32,
    pub start_y: f32,
}

pub fn spawn_ship(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window>,
) {
    let window = window_query.single().unwrap();
    let target_y = window.height() / 6.0; // Even lower on the screen
    let start_y = window.height() / 1.0;

    commands.spawn((
        Ship {
            speed: 100.0,
            target_y,
            start_y,
        },
        Sprite {
            image: asset_server.load("images/ship.png"),
            ..default()
        },
        Transform::from_xyz(-window.width() * 0.6, start_y, 1.0),
    ));
}

pub fn move_ship(
    mut ship_query: Query<(&mut Transform, &Ship)>,
    time: Res<Time>,
) {
    for (mut transform, ship) in ship_query.iter_mut() {
        let current_y = transform.translation.y;
        let target_y = ship.target_y;

        if current_y > target_y {
            let movement = ship.speed * time.delta().as_secs_f32();
            transform.translation.y -= movement.min(current_y - target_y);
        }
    }
}

pub fn reset_ship_position(
    mut ship_query: Query<(&mut Transform, &Ship)>,
    game_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if game_state.get() == &GameState::Building {
        for (mut transform, ship) in ship_query.iter_mut() {
            transform.translation.y = ship.start_y;
        }
    }
}

pub fn plugin(app: &mut App) {
    app.add_systems(Update, (move_ship, reset_ship_position));
}



