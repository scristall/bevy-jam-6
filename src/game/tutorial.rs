use bevy::prelude::*;
use bevy::text::{LineBreak, TextBounds};

use crate::game::game_state::GameState;
use crate::game::mouse::MousePos;

const TUTORIAL_WINDOW_WIDTH: f32 = 1400.0;
const TUTORIAL_WINDOW_HEIGHT: f32 = 940.0;

const TUTORIAL_WINDOW_PADDING: f32 = 20.0;

const TUTORIAL_TEXT_BOX_WIDTH: f32 = TUTORIAL_WINDOW_WIDTH - 2.0 * TUTORIAL_WINDOW_PADDING;
const TUTORIAL_TEXT_BOX_HEIGHT: f32 = TUTORIAL_WINDOW_HEIGHT - 2.0 * TUTORIAL_WINDOW_PADDING;

const OK_BUTTON_SIZE: Vec2 = Vec2::new(150.0, 100.0);
const OK_BUTTON_POS: Vec2 = Vec2::new(0.0, -400.0);

const TUTORIAL_TEXT: &str = "
Welcome to Chain Lockers!

Chain Lockers are holds within ships to hold massive chains. These chains go through an oxidation recation, which rapidly depletes the oxygen in a ship's hold.

In this game, you will place chains to asphyxiate pirates trying to steal your gold.

To build a chain, click on a chain button in the main inventory to select it, then drag inside the hold to construct a maze.

The number in the top right of each chain type is your stock, and the bottom left is the length of that chain.

At the end of each wave, you will be able to choose chains to add and modifiers to apply to the hold.
";

#[derive(Component, Debug)]
pub struct TutorialWindow;

#[derive(Component, Debug)]
pub struct OkButton;

#[derive(Component, Debug)]
pub struct OkButtonText;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let rect = Rectangle::new(TUTORIAL_WINDOW_WIDTH, TUTORIAL_WINDOW_HEIGHT);
    let color = Color::linear_rgba(0.8, 0.8, 0.8, 1.0);

    let e_tutorial_window = commands
        .spawn((
            TutorialWindow,
            Mesh2d(meshes.add(rect)),
            MeshMaterial2d(materials.add(color)),
            Transform::from_xyz(0.0, 0.0, 15.0),
        ))
        .id();

    commands.entity(e_tutorial_window).with_children(|parent| {
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
            Text2d::new("Chain Lockers"),
            title_font,
            TextColor(Color::linear_rgb(0.0, 0.0, 0.0)),
            Transform::from_xyz(0.0, 400.0, 0.5),
        ));
        parent.spawn((
            Text2d::new(TUTORIAL_TEXT),
            body_text_font,
            TextLayout::new(JustifyText::Left, LineBreak::WordBoundary),
            TextColor(Color::linear_rgb(0.0, 0.0, 0.0)),
            Transform::from_xyz(0.0, -100.0, 0.5),
            TextBounds::from(Vec2::new(TUTORIAL_TEXT_BOX_WIDTH, TUTORIAL_TEXT_BOX_HEIGHT)),
        ));

        // spawn ok button
        let rect = Rectangle::new(OK_BUTTON_SIZE.x, OK_BUTTON_SIZE.y);
        let color = Color::linear_rgba(0.0, 0.0, 1.0, 1.0);
        let text_font = TextFont {
            font: font.clone(),
            font_size: 30.0,
            ..default()
        };

        parent
            .spawn((
                OkButton,
                Transform::from_xyz(OK_BUTTON_POS.x, OK_BUTTON_POS.y, 5.0),
                Mesh2d(meshes.add(rect)),
                MeshMaterial2d(materials.add(color)),
            ))
            .with_child((
                OkButtonText,
                Text2d::new("OK"),
                text_font.clone(),
                TextColor(Color::linear_rgba(1.0, 1.0, 1.0, 1.0)),
            ));
    });
}

fn ok_button(
    mut commands: Commands,
    mouse_pos: Res<MousePos>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut state: ResMut<NextState<GameState>>,
    q_tutorial_window: Query<(Entity, &TutorialWindow)>,
) {
    if mouse_button.just_pressed(MouseButton::Left)
        && mouse_pos.is_in(OK_BUTTON_POS, OK_BUTTON_SIZE)
    {
        state.set(GameState::Building);
        let (e_tutorial_window, _) = q_tutorial_window.single().unwrap();
        commands.entity(e_tutorial_window).despawn();
    }
}

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, setup);
    app.add_systems(Update, (ok_button).run_if(in_state(GameState::Tutorial)));
}
