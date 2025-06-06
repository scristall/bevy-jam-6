use bevy::prelude::*;

use crate::game::chain::{ChainInInventory, spawn_chain_in_inventory};
use crate::game::events::WaveComplete;
use crate::game::game_state::GameState;

const PRIZE_WINDOW_WIDTH: f32 = 1400.0;
const PRIZE_WINDOW_HEIGHT: f32 = 800.0;

#[derive(Component)]
pub struct PrizeWindow;

fn on_wave_complete(
    mut evr_wave_complete: EventReader<WaveComplete>,
    asset_server: ResMut<AssetServer>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if evr_wave_complete.is_empty() {
        return;
    }

    // drain events
    evr_wave_complete.clear();

    let rect = Rectangle::new(PRIZE_WINDOW_WIDTH, PRIZE_WINDOW_HEIGHT);
    let color = Color::linear_rgba(0.8, 0.8, 0.8, 1.0);

    // spawn prize window
    let e_prize_window = commands
        .spawn((
            PrizeWindow,
            Mesh2d(meshes.add(rect)),
            MeshMaterial2d(materials.add(color)),
            Transform::from_xyz(0.0, 0.0, 15.0),
        ))
        .id();

    commands.entity(e_prize_window).with_children(|parent| {
        parent.spawn((
            Text2d::new("Prizes"),
            TextFont {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 50.0,
                ..default()
            },
            TextColor(Color::linear_rgb(0.0, 0.0, 0.0)),
            Transform::from_xyz(0.0, 300.0, 0.5),
        ));
    });

    // spawn chain options
    spawn_chain_in_inventory(
        &mut commands,
        e_prize_window,
        1,
        9,
        &asset_server,
        Vec2::new(-400.0, 0.0),
    );

    spawn_chain_in_inventory(
        &mut commands,
        e_prize_window,
        1,
        6,
        &asset_server,
        Vec2::new(0.0, 0.0),
    );

    spawn_chain_in_inventory(
        &mut commands,
        e_prize_window,
        1,
        3,
        &asset_server,
        Vec2::new(400.0, 0.0),
    );
}

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (on_wave_complete).run_if(in_state(GameState::Prize)),
    );
}
