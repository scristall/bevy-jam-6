use bevy::prelude::*;
use bevy::text::{LineBreak, TextBounds};

use crate::game::events::GameOver;
use crate::game::pirate::WaveNumber;

const GAME_OVER_WINDOW_WIDTH: f32 = 1400.0;
const GAME_OVER_WINDOW_HEIGHT: f32 = 800.0;

const GAME_OVER_WINDOW_PADDING: f32 = 20.0;

const GAME_OVER_TEXT_BOX_WIDTH: f32 = GAME_OVER_WINDOW_WIDTH - 2.0 * GAME_OVER_WINDOW_PADDING;
const GAME_OVER_TEXT_BOX_HEIGHT: f32 = GAME_OVER_WINDOW_HEIGHT - 2.0 * GAME_OVER_WINDOW_PADDING;

#[derive(Component, Debug)]
pub struct GameOverWindow;

#[derive(Component, Debug)]
pub struct OkButton;

#[derive(Component, Debug)]
pub struct OkButtonText;

fn spawn_game_over_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    wave_number: Res<WaveNumber>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut evr_game_over: EventReader<GameOver>,
) {
    for _ in evr_game_over.read() {
        let rect = Rectangle::new(GAME_OVER_WINDOW_WIDTH, GAME_OVER_WINDOW_HEIGHT);
        let color = Color::linear_rgba(0.8, 0.8, 0.8, 1.0);

        let game_over_text = format!(
            "The pirates have escaped with all of your gold!\n\nYou made it to wave {}\n\nThank you for playing!",
            wave_number.0
        );

        let e_game_over_window = commands
            .spawn((
                GameOverWindow,
                Mesh2d(meshes.add(rect)),
                MeshMaterial2d(materials.add(color)),
                Transform::from_xyz(0.0, 0.0, 15.0),
            ))
            .id();

        commands.entity(e_game_over_window).with_children(|parent| {
            let font = asset_server.load("fonts/FiraSans-Bold.ttf");
            let title_font = TextFont {
                font: font.clone(),
                font_size: 50.0,
                ..default()
            };
            let body_text_font = TextFont {
                font: font.clone(),
                font_size: 35.0,
                ..default()
            };

            parent.spawn((
                Text2d::new("Game Over"),
                title_font,
                TextColor(Color::linear_rgb(0.0, 0.0, 0.0)),
                Transform::from_xyz(0.0, 300.0, 0.5),
            ));

            parent.spawn((
                Text2d::new(game_over_text),
                body_text_font,
                TextLayout::new(JustifyText::Center, LineBreak::WordBoundary),
                TextColor(Color::linear_rgb(0.0, 0.0, 0.0)),
                Transform::from_xyz(0.0, -300.0, 0.5),
                TextBounds::from(Vec2::new(
                    GAME_OVER_TEXT_BOX_WIDTH,
                    GAME_OVER_TEXT_BOX_HEIGHT,
                )),
            ));
        });
    }
}

pub fn plugin(app: &mut App) {
    app.add_systems(Update, spawn_game_over_screen);
}
