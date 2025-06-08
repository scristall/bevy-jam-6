use bevy::prelude::*;
use grid_util::grid::Grid;
use rand_chacha::rand_core::TryRngCore;

use crate::game::chain::ChainSegment;
use crate::game::events::{CrateSpawned, FoolsGoldSpawned, PrizeCollected};
use crate::game::game_state::GameState;
use crate::game::mouse::MousePos;
use crate::game::pirate::{BOAT_POINT, HOLD_POINT, get_pathing_grid};
use crate::game::random::RandomSource;
use crate::game::tile::{GRID_HEIGHT, GRID_WIDTH, Tile};

const MODIFIER_WINDOW_WIDTH: f32 = 1400.0;
const MODIFIER_WINDOW_HEIGHT: f32 = 800.0;

const MODIFIER_CHOICE_BUTTON_SIZE: Vec2 = Vec2::new(260.0, 100.0);

#[derive(Component)]
pub struct ModifierScreen;

#[derive(Component)]
pub struct ModifierWindow;

#[derive(Component, Debug)]
pub enum ModifierChoiceButton {
    #[allow(unused)]
    Glue,
    FoolsGold,
    Crates,
}

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
                ModifierChoiceButton::FoolsGold,
                Transform::from_xyz(-300.0, 0.0, 5.0),
                Mesh2d(meshes.add(rect)),
                MeshMaterial2d(materials.add(color)),
            ))
            .with_child((
                ModifierChoiceButtonText,
                Text2d::new("1x Fool's Gold"),
                text_font.clone(),
                TextColor(Color::linear_rgba(1.0, 1.0, 1.0, 1.0)),
            ));

        // Not yet implemented
        //parent
        //    .spawn((
        //        ModifierChoiceButton::Glue,
        //        Transform::from_xyz(0.0, 0.0, 5.0),
        //        Mesh2d(meshes.add(rect)),
        //        MeshMaterial2d(materials.add(color)),
        //        Visibility::Hidden,
        //    ))
        //    .with_child((
        //        ModifierChoiceButtonText,
        //        Text2d::new("2x Glue Puddle"),
        //        text_font.clone(),
        //        TextColor(Color::linear_rgba(1.0, 1.0, 1.0, 1.0)),
        //    ));

        parent
            .spawn((
                ModifierChoiceButton::Crates,
                Transform::from_xyz(300.0, 0.0, 5.0),
                Mesh2d(meshes.add(rect)),
                MeshMaterial2d(materials.add(color)),
            ))
            .with_child((
                ModifierChoiceButtonText,
                Text2d::new("4x Crates"),
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
    mut random_source: ResMut<RandomSource>,
    mut q_modifier_choice_buttons: Query<(&GlobalTransform, &ModifierChoiceButton)>,
    q_modifier_window: Query<(Entity, &ModifierWindow)>,
    q_chain_segments: Query<&ChainSegment>,
    mut evw_fools_gold_spawned: EventWriter<FoolsGoldSpawned>,
    mut evw_crate_spawned: EventWriter<CrateSpawned>,
) {
    for (transform, modifier_choice_button) in q_modifier_choice_buttons.iter_mut() {
        if mouse_pos.is_in(
            transform.translation().truncate(),
            MODIFIER_CHOICE_BUTTON_SIZE,
        ) && mouse_button.just_pressed(MouseButton::Left)
        {
            state.set(GameState::Building);
            let (e_modifier_window, _) = q_modifier_window.single().unwrap();
            commands.entity(e_modifier_window).despawn();

            // get a list of tiles occupied by the chain
            let chain_tiles = q_chain_segments
                .iter()
                .map(|seg| seg.tile)
                .collect::<Vec<_>>();

            // create list of tiles that are not occupied by the chain
            let mut free_tiles = Vec::new();
            for x in 1..GRID_WIDTH {
                for y in 0..GRID_HEIGHT {
                    let tile = Tile { x, y };
                    if !chain_tiles.contains(&tile) {
                        free_tiles.push(tile);
                    }
                }
            }

            let rng = &mut random_source.0;

            match modifier_choice_button {
                ModifierChoiceButton::FoolsGold => {
                    // choose a random free tile
                    let random_index = (rng.try_next_u32().unwrap() as usize) % free_tiles.len();
                    let random_tile = free_tiles[random_index];
                    evw_fools_gold_spawned.write(FoolsGoldSpawned { tile: random_tile });
                }
                ModifierChoiceButton::Glue => {
                    println!("Glue");
                }
                ModifierChoiceButton::Crates => {
                    let mut non_blocking_free_tiles = Vec::new();
                    for tile in free_tiles.iter() {
                        let mut pathing_grid = get_pathing_grid(q_chain_segments);
                        pathing_grid.set(tile.x as usize, tile.y as usize, true);
                        pathing_grid.generate_components();

                        let start = BOAT_POINT;
                        let end = HOLD_POINT;

                        let path = pathing_grid.get_path_single_goal(start, end, false);
                        if path.is_some() {
                            non_blocking_free_tiles.push(tile);
                        }
                    }

                    for _ in 0..4 {
                        let random_index =
                            (rng.try_next_u32().unwrap() as usize) % non_blocking_free_tiles.len();
                        let random_tile = non_blocking_free_tiles[random_index];
                        evw_crate_spawned.write(CrateSpawned { tile: *random_tile });
                    }
                }
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
