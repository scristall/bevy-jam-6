use bevy::prelude::*;

use crate::game::{game_state::GameState, mouse::MousePos};

const TITLE_SCREEN_NEW_GAME_TEXT_SIZE: Vec2 = Vec2::new(400.0, 80.0);
const TITLE_SCREEN_NEW_GAME_TEXT_POS: Vec2 = Vec2::new(480.0, -200.0);

#[derive(Component)]
pub struct TitleScreen;

#[derive(Component)]
pub struct TitleScreenNewGameText;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let e_title_screen = commands
        .spawn((
            TitleScreen,
            Sprite::from_image(asset_server.load("images/title-screen.png")),
            Transform::from_xyz(0.0, 0.0, 30.0),
        ))
        .id();

    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let text_font = TextFont {
        font: font.clone(),
        font_size: TITLE_SCREEN_NEW_GAME_TEXT_SIZE.y,
        ..default()
    };

    commands.entity(e_title_screen).with_child((
        TitleScreenNewGameText,
        Text2d::new("NEW GAME"),
        text_font,
        TextColor(Color::WHITE),
        Transform::from_xyz(
            TITLE_SCREEN_NEW_GAME_TEXT_POS.x,
            TITLE_SCREEN_NEW_GAME_TEXT_POS.y,
            5.0,
        ),
    ));
}

fn new_game_text_hover(
    mouse_pos: Res<MousePos>,
    mut q_new_game_text: Query<&mut TextColor, With<TitleScreenNewGameText>>,
) {
    if mouse_pos.is_in(
        TITLE_SCREEN_NEW_GAME_TEXT_POS,
        TITLE_SCREEN_NEW_GAME_TEXT_SIZE,
    ) {
        for mut text_color in q_new_game_text.iter_mut() {
            *text_color = TextColor(Color::linear_rgba(0.7, 0.7, 1.0, 1.0));
        }
    } else {
        for mut text_color in q_new_game_text.iter_mut() {
            *text_color = TextColor(Color::WHITE);
        }
    }
}

fn new_game_text_click(
    mut commands: Commands,
    mouse_pos: Res<MousePos>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut state: ResMut<NextState<GameState>>,
    q_title_screen: Query<Entity, With<TitleScreen>>,
) {
    if mouse_pos.is_in(
        TITLE_SCREEN_NEW_GAME_TEXT_POS,
        TITLE_SCREEN_NEW_GAME_TEXT_SIZE,
    ) && mouse_button.just_pressed(MouseButton::Left)
    {
        commands.entity(q_title_screen.single().unwrap()).despawn();
        state.set(GameState::Tutorial);
    }
}

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, setup);
    app.add_systems(
        Update,
        (new_game_text_hover, new_game_text_click).run_if(in_state(GameState::TitleScreen)),
    );
}
