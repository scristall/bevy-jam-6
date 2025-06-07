use bevy::prelude::*;

use crate::game::events::PrizeCollected;
use crate::game::game_state::GameState;
use crate::game::mouse::MousePos;

const MODIFIER_WINDOW_WIDTH: f32 = 1400.0;
const MODIFIER_WINDOW_HEIGHT: f32 = 800.0;

const MODIFIER_CHOICE_BUTTON_SIZE: Vec2 = Vec2::new(200.0, 100.0);

#[derive(Component)]
pub struct ModifierScreen;

#[derive(Component)]
pub struct ModifierWindow;

#[derive(Component, Debug)]
pub struct ModifierChoiceButton;

#[derive(Component, Debug)]
pub struct ModifierChoiceButtonText;

fn on_wave_complete(
    mut evr_prize_collected: EventReader<PrizeCollected>,
    asset_server: ResMut<AssetServer>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if evr_prize_collected.is_empty() {
        return;
    }

    evr_prize_collected.clear();

    let rect = Rectangle::new(MODIFIER_WINDOW_WIDTH, MODIFIER_WINDOW_HEIGHT);
    let color = Color::linear_rgba(0.8, 0.8, 0.8, 1.0);

    // spawn modifier window
    let e_modifier_window = commands
        .spawn((
            ModifierWindow,
            Mesh2d(meshes.add(rect)),
            MeshMaterial2d(materials.add(color)),
            Transform::from_xyz(0.0, 0.0, 15.0),
        ))
        .id();

    commands.entity(e_modifier_window).with_children(|parent| {
        let font = asset_server.load("fonts/FiraSans-Bold.ttf");
        parent.spawn((
            Text2d::new("Choose a New Modifier"),
            TextFont {
                font: font.clone(),
                font_size: 50.0,
                ..default()
            },
            TextColor(Color::linear_rgb(0.0, 0.0, 0.0)),
            Transform::from_xyz(0.0, 300.0, 0.5),
        ));

        // spawn modifier choice buttons
        let rect = Rectangle::new(MODIFIER_CHOICE_BUTTON_SIZE.x, MODIFIER_CHOICE_BUTTON_SIZE.y);
        let color = Color::linear_rgba(0.0, 0.0, 1.0, 1.0);
        let text_font = TextFont {
            font: font.clone(),
            font_size: 30.0,
            ..default()
        };

        parent
            .spawn((
                ModifierChoiceButton,
                Transform::from_xyz(-300.0, 0.0, 5.0),
                Mesh2d(meshes.add(rect)),
                MeshMaterial2d(materials.add(color)),
            ))
            .with_child((
                ModifierChoiceButtonText,
                Text2d::new("Fool's Gold"),
                text_font.clone(),
                TextColor(Color::linear_rgba(1.0, 1.0, 1.0, 1.0)),
            ));

        parent
            .spawn((
                ModifierChoiceButton,
                Transform::from_xyz(0.0, 0.0, 5.0),
                Mesh2d(meshes.add(rect)),
                MeshMaterial2d(materials.add(color)),
            ))
            .with_child((
                ModifierChoiceButtonText,
                Text2d::new("Glue Puddle"),
                text_font.clone(),
                TextColor(Color::linear_rgba(1.0, 1.0, 1.0, 1.0)),
            ));

        parent
            .spawn((
                ModifierChoiceButton,
                Transform::from_xyz(300.0, 0.0, 5.0),
                Mesh2d(meshes.add(rect)),
                MeshMaterial2d(materials.add(color)),
            ))
            .with_child((
                ModifierChoiceButtonText,
                Text2d::new("Crates"),
                text_font.clone(),
                TextColor(Color::linear_rgba(1.0, 1.0, 1.0, 1.0)),
            ));
    });
}

fn mouse_down_on_modifier_choice_button(
    mut commands: Commands,
    mouse_pos: Res<MousePos>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut state: ResMut<NextState<GameState>>,
    mut q_modifier_choice_buttons: Query<&GlobalTransform, With<ModifierChoiceButton>>,
    q_modifier_window: Query<(Entity, &ModifierWindow)>,
) {
    for transform in q_modifier_choice_buttons.iter_mut() {
        if mouse_pos.is_in(
            transform.translation().truncate(),
            MODIFIER_CHOICE_BUTTON_SIZE,
        ) {
            if mouse_button.just_pressed(MouseButton::Left) {
                state.set(GameState::Building);
                let (e_modifier_window, _) = q_modifier_window.single().unwrap();
                commands.entity(e_modifier_window).despawn();
            }
        }
    }
}

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (on_wave_complete, mouse_down_on_modifier_choice_button)
            .run_if(in_state(GameState::Modifier)),
    );
}
