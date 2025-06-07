use bevy::prelude::*;

use crate::game::chain::{
    CHAIN_BUTTON_SIZE, ChainButton, ChainButtonStock, MainInventoryChainButton, spawn_chain_button,
};
use crate::game::events::WaveComplete;
use crate::game::game_state::GameState;
use crate::game::mouse::MousePos;

const PRIZE_WINDOW_WIDTH: f32 = 1400.0;
const PRIZE_WINDOW_HEIGHT: f32 = 800.0;

#[derive(Component)]
pub struct PrizeWindow;

#[derive(Component, Debug, Default)]
pub struct PrizeWindowChainButton;

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

    evr_wave_complete.clear();

    println!("Wave complete");

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
    spawn_chain_button::<PrizeWindowChainButton>(
        &mut commands,
        e_prize_window,
        1,
        9,
        &asset_server,
        Vec2::new(-400.0, 0.0),
    );

    spawn_chain_button::<PrizeWindowChainButton>(
        &mut commands,
        e_prize_window,
        1,
        6,
        &asset_server,
        Vec2::new(0.0, 0.0),
    );

    spawn_chain_button::<PrizeWindowChainButton>(
        &mut commands,
        e_prize_window,
        1,
        3,
        &asset_server,
        Vec2::new(400.0, 0.0),
    );
}

fn mouse_down_on_chain_button_in_prize_window(
    mut commands: Commands,
    mouse_pos: Res<MousePos>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut state: ResMut<NextState<GameState>>,
    mut q_chain_buttons: Query<
        (&ChainButton, &mut Sprite, &GlobalTransform),
        (
            With<PrizeWindowChainButton>,
            Without<MainInventoryChainButton>,
        ),
    >,
    mut q_chain_button_in_inventory: Query<
        (&mut ChainButton, &Children),
        (
            With<MainInventoryChainButton>,
            Without<PrizeWindowChainButton>,
        ),
    >,
    mut q_chain_button_stock_text: Query<&mut Text2d, With<ChainButtonStock>>,
    q_prize_window: Query<(Entity, &PrizeWindow)>,
) {
    for (selected_chain_button, mut sprite, transform) in q_chain_buttons.iter_mut() {
        if mouse_pos.is_in(
            transform.translation().truncate(),
            Vec2::splat(CHAIN_BUTTON_SIZE),
        ) {
            if mouse_button.just_pressed(MouseButton::Left) {
                let (e_prize_window, _) = q_prize_window.single().unwrap();
                commands.entity(e_prize_window).despawn();
                state.set(GameState::Building);
                for (mut chain_button_in_inventory, children) in
                    q_chain_button_in_inventory.iter_mut()
                {
                    if chain_button_in_inventory.length == selected_chain_button.length {
                        chain_button_in_inventory.stock += selected_chain_button.stock;
                    }

                    for child in children.iter() {
                        if let Ok(mut text) = q_chain_button_stock_text.get_mut(child) {
                            text.0 = format!("{}", chain_button_in_inventory.stock);
                        }
                    }
                }
                break;
            } else {
                sprite.color = Color::linear_rgba(0.0, 0.0, 1.0, 1.0);
            }
        } else {
            sprite.color = Color::linear_rgba(1.0, 1.0, 1.0, 1.0);
        }
    }
}

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (on_wave_complete, mouse_down_on_chain_button_in_prize_window)
            .run_if(in_state(GameState::Prize)),
    );
}
