use bevy::prelude::*;
use bevy::text::TextBounds;
use grid_util::grid::Grid;
use rand::RngCore;

use crate::game::chain::{ChainSegment, Obstacle};
use crate::game::events::{
    CrateSpawned, FoolsGoldSpawned, GlueSpawned, PrizeCollected, TreeSpawned,
};
use crate::game::game_state::GameState;
use crate::game::mouse::MousePos;
use crate::game::pirate::{BOAT_POINT, HOLD_POINT, get_pathing_grid};
use crate::game::tile::{GRID_HEIGHT, GRID_WIDTH, Tile};

const MODIFIER_WINDOW_WIDTH: f32 = 1400.0;
const MODIFIER_WINDOW_HEIGHT: f32 = 800.0;

const MODIFIER_CHOICE_BUTTON_SIZE: Vec2 = Vec2::new(260.0, 100.0);

const MODIFIER_EXPLANATION_TEXT_BOX_WIDTH: f32 = 400.0;
const MODIFIER_EXPLANATION_TEXT_BOX_HEIGHT: f32 = 100.0;

#[derive(Component)]
pub struct ModifierScreen;

#[derive(Component)]
pub struct ModifierWindow;

#[derive(Component)]
pub struct ModifierChoiceButton;

#[derive(Component, Clone, Copy, PartialEq)]
pub enum GoodModifier {
    Glue,
    FoolsGold,
}

#[derive(Component, Clone, Copy, PartialEq)]
pub enum BadModifier {
    Crate,
    Tree,
    BrokenChain,
}

#[derive(Component, Debug)]
pub struct ModifierChoiceButtonText;

impl BadModifier {
    pub fn get_text(&self) -> &str {
        match self {
            BadModifier::Crate => "Crates",
            BadModifier::Tree => "Tree",
            BadModifier::BrokenChain => "Broken Chain",
        }
    }
    pub fn get_explanation(&self) -> &str {
        match self {
            BadModifier::Crate => "Spawn crates that block the path of the pirates",
            BadModifier::Tree => "Spawn a tree that and helps the pirates breathe",
            BadModifier::BrokenChain => "Break a random chain link",
        }
    }

    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        match rng.next_u64() % 3 {
            0 => BadModifier::Crate,
            1 => BadModifier::Tree,
            2 => BadModifier::BrokenChain,
            _ => unreachable!(),
        }
    }
}

impl GoodModifier {
    pub fn get_text(&self) -> &str {
        match self {
            GoodModifier::Glue => "Glue Puddle",
            GoodModifier::FoolsGold => "Fool's Gold",
        }
    }

    pub fn get_explanation(&self) -> &str {
        match self {
            GoodModifier::Glue => "Spawn a glue puddle that slows the pirates down",
            GoodModifier::FoolsGold => "Spawn a fool's gold that distracts the pirates",
        }
    }

    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        match rng.next_u64() % 2 {
            0 => GoodModifier::Glue,
            1 => GoodModifier::FoolsGold,
            _ => unreachable!(),
        }
    }
}

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
            Text2d::new("Choose New Modifiers"),
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

        // Choice 1
        let good_modifier = GoodModifier::random();
        let bad_modifier = BadModifier::random();

        parent
            .spawn((
                ModifierChoiceButton,
                good_modifier,
                bad_modifier,
                Transform::from_xyz(-300.0, -200.0, 5.0),
                Mesh2d(meshes.add(rect)),
                MeshMaterial2d(materials.add(color)),
            ))
            .with_children(|parent| {
                parent.spawn((
                    Text2d::new(good_modifier.get_text()),
                    TextFont {
                        font: font.clone(),
                        font_size: 30.0,
                        ..default()
                    },
                    TextColor(Color::linear_rgba(0.14, 0.7, 0.14, 1.0)),
                    Transform::from_xyz(0.0, 400.0, 0.5),
                ));

                parent.spawn((
                    Text2d::new(good_modifier.get_explanation()),
                    TextFont {
                        font: font.clone(),
                        font_size: 30.0,
                        ..default()
                    },
                    TextColor(Color::linear_rgba(0.0, 0.0, 0.0, 1.0)),
                    Transform::from_xyz(0.0, 300.0, 0.5),
                    TextLayout::new(JustifyText::Center, LineBreak::WordBoundary),
                    TextBounds::from(Vec2::new(
                        MODIFIER_EXPLANATION_TEXT_BOX_WIDTH,
                        MODIFIER_EXPLANATION_TEXT_BOX_HEIGHT,
                    )),
                ));

                parent.spawn((
                    Text2d::new(bad_modifier.get_text()),
                    TextFont {
                        font: font.clone(),
                        font_size: 30.0,
                        ..default()
                    },
                    TextColor(Color::linear_rgba(1.0, 0.0, 0.0, 1.0)),
                    Transform::from_xyz(0.0, 200.0, 0.5),
                ));

                parent.spawn((
                    Text2d::new(bad_modifier.get_explanation()),
                    TextFont {
                        font: font.clone(),
                        font_size: 30.0,
                        ..default()
                    },
                    TextColor(Color::linear_rgba(0.0, 0.0, 0.0, 1.0)),
                    Transform::from_xyz(0.0, 100.0, 0.5),
                    TextLayout::new(JustifyText::Center, LineBreak::WordBoundary),
                    TextBounds::from(Vec2::new(
                        MODIFIER_EXPLANATION_TEXT_BOX_WIDTH,
                        MODIFIER_EXPLANATION_TEXT_BOX_HEIGHT,
                    )),
                ));

                parent.spawn((
                    ModifierChoiceButtonText,
                    Text2d::new("Choose"),
                    text_font.clone(),
                    TextColor(Color::linear_rgba(1.0, 1.0, 1.0, 1.0)),
                    Transform::from_xyz(0.0, 0.0, 0.5),
                ));
            });

        // Choice 2
        let mut new_good_modifier;
        let mut new_bad_modifier;
        loop {
            new_good_modifier = GoodModifier::random();
            new_bad_modifier = BadModifier::random();
            if new_good_modifier != good_modifier || new_bad_modifier != bad_modifier {
                break;
            }
        }

        let good_modifier = new_good_modifier;
        let bad_modifier = new_bad_modifier;

        parent
            .spawn((
                ModifierChoiceButton,
                good_modifier,
                bad_modifier,
                Transform::from_xyz(300.0, -200.0, 5.0),
                Mesh2d(meshes.add(rect)),
                MeshMaterial2d(materials.add(color)),
            ))
            .with_children(|parent| {
                parent.spawn((
                    Text2d::new(good_modifier.get_text()),
                    TextFont {
                        font: font.clone(),
                        font_size: 30.0,
                        ..default()
                    },
                    TextColor(Color::linear_rgba(0.14, 0.7, 0.14, 1.0)),
                    Transform::from_xyz(0.0, 400.0, 0.5),
                ));

                parent.spawn((
                    Text2d::new(good_modifier.get_explanation()),
                    TextFont {
                        font: font.clone(),
                        font_size: 30.0,
                        ..default()
                    },
                    TextColor(Color::linear_rgba(0.0, 0.0, 0.0, 1.0)),
                    Transform::from_xyz(0.0, 300.0, 0.5),
                    TextLayout::new(JustifyText::Center, LineBreak::WordBoundary),
                    TextBounds::from(Vec2::new(
                        MODIFIER_EXPLANATION_TEXT_BOX_WIDTH,
                        MODIFIER_EXPLANATION_TEXT_BOX_HEIGHT,
                    )),
                ));

                parent.spawn((
                    Text2d::new(bad_modifier.get_text()),
                    TextFont {
                        font: font.clone(),
                        font_size: 30.0,
                        ..default()
                    },
                    TextColor(Color::linear_rgba(1.0, 0.0, 0.0, 1.0)),
                    Transform::from_xyz(0.0, 200.0, 0.5),
                ));

                parent.spawn((
                    Text2d::new(bad_modifier.get_explanation()),
                    TextFont {
                        font: font.clone(),
                        font_size: 30.0,
                        ..default()
                    },
                    TextColor(Color::linear_rgba(0.0, 0.0, 0.0, 1.0)),
                    Transform::from_xyz(0.0, 100.0, 0.5),
                    TextLayout::new(JustifyText::Center, LineBreak::WordBoundary),
                    TextBounds::from(Vec2::new(
                        MODIFIER_EXPLANATION_TEXT_BOX_WIDTH,
                        MODIFIER_EXPLANATION_TEXT_BOX_HEIGHT,
                    )),
                ));

                parent.spawn((
                    ModifierChoiceButtonText,
                    Text2d::new("Choose"),
                    text_font.clone(),
                    TextColor(Color::linear_rgba(1.0, 1.0, 1.0, 1.0)),
                    Transform::from_xyz(0.0, 0.0, 0.5),
                ));
            });
    });
}

fn mouse_down_on_modifier_choice_button(
    mut commands: Commands,
    mouse_pos: Res<MousePos>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut state: ResMut<NextState<GameState>>,
    mut q_modifier_choice_buttons: Query<
        (&GlobalTransform, &GoodModifier, &BadModifier),
        With<ModifierChoiceButton>,
    >,
    q_modifier_window: Query<(Entity, &ModifierWindow)>,
    mut q_obstacles: Query<(Entity, &Obstacle, Option<&ChainSegment>)>,
    mut evw_fools_gold_spawned: EventWriter<FoolsGoldSpawned>,
    mut evw_crate_spawned: EventWriter<CrateSpawned>,
    mut evw_glue_spawned: EventWriter<GlueSpawned>,
    mut evw_tree_spawned: EventWriter<TreeSpawned>,
) {
    for (transform, good_modifier, bad_modifier) in q_modifier_choice_buttons.iter_mut() {
        if mouse_pos.is_in(
            transform.translation().truncate(),
            MODIFIER_CHOICE_BUTTON_SIZE,
        ) && mouse_button.just_pressed(MouseButton::Left)
        {
            state.set(GameState::Building);
            let (e_modifier_window, _) = q_modifier_window.single().unwrap();
            commands.entity(e_modifier_window).despawn();

            // get a list of tiles occupied by the chain
            let chain_tiles = q_obstacles
                .iter()
                .map(|(_, obstacle, _)| obstacle.tile)
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

            let mut rng = rand::thread_rng();

            match good_modifier {
                GoodModifier::FoolsGold => {
                    // choose a random free tile
                    let random_index = (rng.next_u64() as usize) % free_tiles.len();
                    let random_tile = free_tiles[random_index];
                    evw_fools_gold_spawned.write(FoolsGoldSpawned { tile: random_tile });
                }
                GoodModifier::Glue => {
                    // choose a random free tile
                    let random_index = (rng.next_u64() as usize) % free_tiles.len();
                    let random_tile = free_tiles[random_index];
                    evw_glue_spawned.write(GlueSpawned { tile: random_tile });
                }
            }
            match bad_modifier {
                BadModifier::Crate => {
                    let mut non_blocking_free_tiles = Vec::new();
                    for tile in free_tiles.iter() {
                        let mut q_obstacles_filtered = q_obstacles.transmute_lens::<&Obstacle>();
                        let mut pathing_grid = get_pathing_grid(q_obstacles_filtered.query());
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
                            (rng.next_u64() as usize) % non_blocking_free_tiles.len();
                        let random_tile = non_blocking_free_tiles[random_index];
                        evw_crate_spawned.write(CrateSpawned { tile: *random_tile });
                    }
                }
                BadModifier::Tree => {
                    let mut non_blocking_free_tiles = Vec::new();
                    for tile in free_tiles.iter() {
                        let mut q_obstacles_filtered = q_obstacles.transmute_lens::<&Obstacle>();
                        let mut pathing_grid = get_pathing_grid(q_obstacles_filtered.query());
                        pathing_grid.set(tile.x as usize, tile.y as usize, true);
                        pathing_grid.generate_components();

                        let start = BOAT_POINT;
                        let end = HOLD_POINT;

                        let path = pathing_grid.get_path_single_goal(start, end, false);
                        if path.is_some() {
                            non_blocking_free_tiles.push(tile);
                        }
                    }

                    // choose a random free tile
                    let random_index = (rng.next_u64() as usize) % non_blocking_free_tiles.len();
                    let random_tile = non_blocking_free_tiles[random_index];
                    evw_tree_spawned.write(TreeSpawned { tile: *random_tile });
                }
                BadModifier::BrokenChain => {
                    // get list of tiles occupied by a chain segment
                    let chain_tiles = q_obstacles
                        .iter()
                        .filter_map(|(entity, _, segment)| {
                            if segment.is_some() {
                                Some(entity)
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<_>>();

                    // choose a random tile from the list
                    let random_index = (rng.next_u64() as usize) % chain_tiles.len();
                    let random_chain_seg = chain_tiles[random_index];
                    commands.entity(random_chain_seg).despawn();
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
