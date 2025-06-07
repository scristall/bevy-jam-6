use bevy::prelude::*;

use crate::game::{
    events::{WaveComplete, WaveStarted},
    game_state::GameState,
    mouse::MousePos,
};

pub const NEXT_WAVE_BUTTON_POS: Vec2 = Vec2::new(700.0, -400.0);
pub const NEXT_WAVE_BUTTON_SIZE: Vec2 = Vec2::new(280.0, 100.0);

#[derive(Component)]
pub struct NextWaveButton;

#[derive(Component)]
pub struct NextWaveButtonText;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: ResMut<AssetServer>,
) {
    let rect = Rectangle::new(NEXT_WAVE_BUTTON_SIZE.x, NEXT_WAVE_BUTTON_SIZE.y);
    let color = Color::linear_rgba(0.0, 0.0, 1.0, 1.0);
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let text_font = TextFont {
        font: font.clone(),
        font_size: 30.0,
        ..default()
    };

    commands
        .spawn((
            NextWaveButton,
            Transform::from_xyz(NEXT_WAVE_BUTTON_POS.x, NEXT_WAVE_BUTTON_POS.y, 5.0),
            Mesh2d(meshes.add(rect)),
            MeshMaterial2d(materials.add(color)),
        ))
        .with_child((
            NextWaveButtonText,
            Text2d::new("Next Wave"),
            text_font.clone(),
            TextColor(Color::linear_rgba(1.0, 1.0, 1.0, 1.0)),
        ));
}

fn next_wave_button(
    mut q_next_wave_button_text: Query<&mut Text2d, With<NextWaveButtonText>>,
    mouse_pos: Res<MousePos>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut evw_wave_started: EventWriter<WaveStarted>,
) {
    if mouse_button.just_pressed(MouseButton::Left)
        && mouse_pos.is_in(NEXT_WAVE_BUTTON_POS, NEXT_WAVE_BUTTON_SIZE)
    {
        let mut text = q_next_wave_button_text.single_mut().unwrap();
        text.0 = "Wave In Progress".to_string();
        game_state.set(GameState::WaveInProgress);
        evw_wave_started.write(WaveStarted);
    }
}

fn on_wave_complete(
    mut evr_wave_complete: EventReader<WaveComplete>,
    mut q_next_wave_button_text: Query<&mut Text2d, With<NextWaveButtonText>>,
) {
    for _ in evr_wave_complete.read() {
        let mut text = q_next_wave_button_text.single_mut().unwrap();
        text.0 = "Next Wave".to_string();
    }
}

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, setup);
    app.add_systems(
        Update,
        (next_wave_button).run_if(in_state(GameState::Building)),
    );

    // This runs in all states, just to make sure the button is updated
    app.add_systems(Update, on_wave_complete);
}
