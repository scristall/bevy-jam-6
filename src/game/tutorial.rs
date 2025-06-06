use bevy::prelude::*;
use bevy::text::{FontSmoothing, LineBreak, TextBounds};

const TUTORIAL_WINDOW_WIDTH: f32 = 1400.0;
const TUTORIAL_WINDOW_HEIGHT: f32 = 800.0;

const TUTORIAL_WINDOW_PADDING: f32 = 20.0;

const TUTORIAL_TEXT_BOX_WIDTH: f32 = TUTORIAL_WINDOW_WIDTH - 2.0 * TUTORIAL_WINDOW_PADDING;
const TUTORIAL_TEXT_BOX_HEIGHT: f32 = TUTORIAL_WINDOW_HEIGHT - 2.0 * TUTORIAL_WINDOW_PADDING;

const TUTORIAL_TEXT: &str = "
Welcome to Chain Lockers!

In this game, you will be placing chains to asphyxiate the enemy through the chains' oxidations deplating the air of the ship's hold.

To build a chain, click on a chain button in the main inventory to select it, then drag inside the hold to construct a maze.

At the end of each wave, you will be able to choose more chains to add to your inventory.
";

#[derive(Component, Debug)]
pub struct TutorialWindow;

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
            Text2d::new("Tutorial"),
            title_font,
            TextColor(Color::linear_rgb(0.0, 0.0, 0.0)),
            Transform::from_xyz(0.0, 300.0, 0.5),
        ));
        parent.spawn((
            Text2d::new(TUTORIAL_TEXT),
            body_text_font,
            TextLayout::new(JustifyText::Left, LineBreak::WordBoundary),
            TextColor(Color::linear_rgb(0.0, 0.0, 0.0)),
            Transform::from_xyz(0.0, -100.0, 0.5),
            TextBounds::from(Vec2::new(TUTORIAL_TEXT_BOX_WIDTH, TUTORIAL_TEXT_BOX_HEIGHT)),
        ));
    });
}

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, setup);
}
