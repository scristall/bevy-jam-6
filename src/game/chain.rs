use bevy::prelude::*;

use crate::game::events::{
    ChainFinished, ChainPlaced, CrateSpawned, PlayClickSFX, TileMouseDown, TileMouseMove,
};
use crate::game::game_state::GameState;
use crate::game::mouse::MousePos;
use crate::game::pirate::{BOAT_POINT, HOLD_POINT, get_pathing_grid};
use crate::game::tile::{Direction, TILE_SIZE, Tile};

pub const CHAIN_BUTTON_SIZE: f32 = 64.0;

#[derive(Component, Debug)]
pub struct Chain {
    #[allow(unused)]
    length: u32,
}

#[derive(Component, Debug)]
pub struct MainInventory;

#[derive(Component, Debug, Default)]
pub struct MainInventoryChainButton;

#[derive(Component, Debug)]
pub struct ChainButton {
    pub stock: u32,
    pub length: u32,
}

#[derive(Component, Debug)]
pub struct ChainButtonStock;

#[derive(Component, Debug)]
pub struct ChainButtonLength;

#[derive(Component, Debug)]
pub struct SelectedChain;

#[derive(Component, Debug)]
pub struct DraggingChain {
    remaining_length: u32,
    e_chain: Entity,
}

#[derive(Component, Debug)]
pub struct ChainSegment {
    pub prev_tile: Option<Tile>,
    pub tile: Tile,
}

#[derive(Component)]
pub struct Obstacle {
    pub tile: Tile,
}

#[derive(Component)]
pub struct Crate;

fn spawn_chain_segment(
    e_chain: Entity,
    commands: &mut Commands,
    prev_tile: Option<Tile>,
    tile: Tile,
    asset_server: &ResMut<AssetServer>,
) {
    let (sprite, rot_degrees) = if let Some(prev_tile) = prev_tile {
        let direction = tile.get_adjacent_tile_direction(&prev_tile);
        let sprite =
            Sprite::from_image(asset_server.load("images/chain-link-single-to-straight.png"));
        let rot_degrees = match direction {
            Some(Direction::Up) => 180.0,
            Some(Direction::Down) => 0.0,
            Some(Direction::Left) => 270.0,
            Some(Direction::Right) => 90.0,
            None => 0.0,
        };
        (sprite, rot_degrees)
    } else {
        (
            Sprite::from_image(asset_server.load("images/chain-link-single.png")),
            0.0,
        )
    };
    let rot = Quat::from_rotation_z((rot_degrees as f32).to_radians());
    commands.entity(e_chain).with_children(|parent| {
        parent.spawn((
            tile,
            Sprite {
                custom_size: Some(Vec2::splat(TILE_SIZE)),
                ..sprite
            },
            ChainSegment { prev_tile, tile },
            Obstacle { tile },
            tile.grid_coord_to_transform(3.0).with_rotation(rot),
        ));
    });
}

fn update_chain_segment(
    chain_segment: &mut ChainSegment,
    next_tile: Tile,
    sprite: &mut Sprite,
    transform: &mut Transform,
    asset_server: &ResMut<AssetServer>,
) {
    let direction_to_next = chain_segment.tile.get_adjacent_tile_direction(&next_tile);
    let direction_to_prev = if chain_segment.prev_tile.is_none() {
        None
    } else {
        let prev_tile = chain_segment.prev_tile.unwrap();
        chain_segment.tile.get_adjacent_tile_direction(&prev_tile)
    };

    // if direction_to_prev is none, we are the first in the chain
    if direction_to_prev.is_none() {
        // we still need to update the transform, based on direction_to_next
        let rot_degrees = match direction_to_next {
            Some(Direction::Up) => 180.0,
            Some(Direction::Down) => 0.0,
            Some(Direction::Left) => 270.0,
            Some(Direction::Right) => 90.0,
            None => 0.0,
        };
        let rot = Quat::from_rotation_z((rot_degrees as f32).to_radians());
        *transform = transform.with_rotation(rot);
        sprite.image = asset_server.load("images/chain-link-single-to-straight.png");
        return;
    }

    // we should only be here if both direction_to_next and direction_to_prev are some
    if direction_to_next.is_none() || direction_to_prev.is_none() {
        return;
    }

    let direction_to_next = direction_to_next.unwrap();
    let direction_to_prev = direction_to_prev.unwrap();

    // if direction_to_next is the opposite of direction_to_prev, we are on a straight
    if direction_to_next == direction_to_prev.opposite() {
        // rotation is either 90 or 0
        let rot_degrees = match direction_to_next {
            Direction::Up => 0.0,
            Direction::Down => 180.0,
            Direction::Left => 270.0,
            Direction::Right => 90.0,
        };
        let rot = Quat::from_rotation_z((rot_degrees as f32).to_radians());
        *transform = transform.with_rotation(rot);
        sprite.image = asset_server.load("images/chain-link-link-to-straight.png");
        return;
    }

    // if we are here, we are on a corner
    let rot_degrees = match direction_to_next {
        Direction::Up => {
            if direction_to_prev == Direction::Right {
                0.0
            } else {
                90.0
            }
        }
        Direction::Down => {
            if direction_to_prev == Direction::Left {
                180.0
            } else {
                270.0
            }
        }
        Direction::Left => {
            if direction_to_prev == Direction::Down {
                180.0
            } else {
                90.0
            }
        }
        Direction::Right => {
            if direction_to_prev == Direction::Up {
                0.0
            } else {
                270.0
            }
        }
    };

    let rot = Quat::from_rotation_z((rot_degrees as f32).to_radians());
    *transform = transform.with_rotation(rot);
    sprite.image = asset_server.load("images/chain-link-link-to-corner.png");
}

pub fn spawn_chain_button<T: Component + Default>(
    commands: &mut Commands,
    e_parent: Entity,
    stock: u32,
    length: u32,
    asset_server: &ResMut<AssetServer>,
    pos: Vec2,
) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let text_font = TextFont {
        font: font.clone(),
        font_size: 50.0,
        ..default()
    };

    commands.entity(e_parent).with_children(|parent| {
        let t: T = T::default();
        parent
            .spawn((
                t,
                Sprite::from_image(asset_server.load("images/chain.png")),
                ChainButton { stock, length },
                Transform::from_xyz(pos.x, pos.y, 0.0),
            ))
            .with_children(|parent| {
                parent.spawn((
                    ChainButtonStock,
                    Text2d::new(format!("{}", stock)),
                    text_font.clone(),
                    Transform::from_xyz(30.0, 30.0, 0.5),
                    TextColor(Color::linear_rgb(0.0, 0.0, 1.0)),
                ));

                parent.spawn((
                    ChainButtonLength,
                    Text2d::new(format!("{}", length)),
                    text_font.clone(),
                    Transform::from_xyz(-30.0, -30.0, 0.5),
                    TextColor(Color::linear_rgb(1.0, 0.0, 0.0)),
                ));
            });
    });
}

fn mouse_down_on_chain_button_in_inventory(
    mut commands: Commands,
    mouse_pos: Res<MousePos>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut q_unselected_chain: Query<
        (Entity, &ChainButton, &mut Sprite, &GlobalTransform),
        (With<MainInventoryChainButton>, Without<SelectedChain>),
    >,
    mut q_selected_chain: Query<
        (
            Entity,
            &SelectedChain,
            &mut Sprite,
            &GlobalTransform,
            &ChainButton,
        ),
        With<MainInventoryChainButton>,
    >,
    mut evw_sfx: EventWriter<PlayClickSFX>,
) {
    let mut new_chain_selected = false;

    for (entity, _, mut sprite, transform) in q_unselected_chain.iter_mut() {
        if mouse_pos.is_in(
            transform.translation().truncate(),
            Vec2::splat(CHAIN_BUTTON_SIZE),
        ) {
            if mouse_button.just_pressed(MouseButton::Left) {
                sprite.color = Color::linear_rgba(0.0, 1.0, 0.0, 1.0);
                commands.entity(entity).insert(SelectedChain);
                evw_sfx.write(PlayClickSFX);
                new_chain_selected = true;
                break;
            } else {
                sprite.color = Color::linear_rgba(0.0, 0.0, 1.0, 1.0);
            }
        } else {
            sprite.color = Color::linear_rgba(1.0, 1.0, 1.0, 1.0);
        }
    }

    if !new_chain_selected {
        return;
    }

    for (entity, _, mut sprite, _, _) in q_selected_chain.iter_mut() {
        sprite.color = Color::linear_rgba(1.0, 1.0, 1.0, 1.0);
        commands.entity(entity).remove::<SelectedChain>();
    }
}

fn setup(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    let e_main_inventory = commands
        .spawn((
            MainInventory,
            Transform::from_xyz(0.0, 0.0, 5.0),
            Visibility::Visible,
        ))
        .id();
    spawn_chain_button::<MainInventoryChainButton>(
        &mut commands,
        e_main_inventory,
        1,
        9,
        &asset_server,
        Vec2::new(-300.0, -400.0),
    );
    spawn_chain_button::<MainInventoryChainButton>(
        &mut commands,
        e_main_inventory,
        2,
        4,
        &asset_server,
        Vec2::new(0.0, -400.0),
    );
    spawn_chain_button::<MainInventoryChainButton>(
        &mut commands,
        e_main_inventory,
        3,
        2,
        &asset_server,
        Vec2::new(300.0, -400.0),
    );
}

fn begin_chain(
    mut commands: Commands,
    mut tile_clicked_events: EventReader<TileMouseDown>,
    mut evw_chain_placed: EventWriter<ChainPlaced>,
    asset_server: ResMut<AssetServer>,
    q_dragging_chain: Query<&DraggingChain>,
    q_selected_chain: Query<(&SelectedChain, &ChainButton), Without<DraggingChain>>,
    q_chain_segments: Query<&ChainSegment>,
) {
    // if there is a chain already being dragged, do nothing
    if q_dragging_chain.single().is_ok() {
        return;
    }

    // if there is no selected chain, do nothing
    if q_selected_chain.single().is_err() {
        return;
    }

    let (_, chain_in_inventory) = q_selected_chain.single().unwrap();

    // if there is no stock, do nothing
    if chain_in_inventory.stock == 0 {
        return;
    }

    // if there is a tile clicked, create a new chain
    for event in tile_clicked_events.read() {
        // if there is a chain segment at this position, do nothing
        if q_chain_segments
            .iter()
            .any(|segment| segment.tile == event.0.tile)
        {
            continue;
        }

        let e_chain = commands
            .spawn((
                Chain {
                    length: chain_in_inventory.length,
                },
                Transform::from_xyz(0.0, 0.0, 0.0),
                Visibility::Visible,
            ))
            .id();

        commands.spawn((
            DraggingChain {
                remaining_length: chain_in_inventory.length - 1,
                e_chain,
            },
            // reference to current segment tile
            ChainSegment {
                prev_tile: None,
                tile: event.0.tile,
            },
        ));

        evw_chain_placed.write(ChainPlaced);
        spawn_chain_segment(e_chain, &mut commands, None, event.0.tile, &asset_server);
    }
}

fn drag_chain(
    mut commands: Commands,
    mut tile_mouse_move_events: EventReader<TileMouseMove>,
    asset_server: ResMut<AssetServer>,
    mut q_dragging_chain: Query<(&mut DraggingChain, &mut ChainSegment)>,
    mut evw_chain_placed: EventWriter<ChainPlaced>,
    mut q_chain_segments: Query<
        (&mut ChainSegment, &mut Sprite, &mut Transform),
        Without<DraggingChain>,
    >,
    q_chain: Query<(Entity, &Children), With<Chain>>,
) {
    // get the dragging chain
    if q_dragging_chain.single().is_err() {
        return;
    }

    let (mut dragging_chain, mut current_chain_segment) = q_dragging_chain.single_mut().unwrap();

    // if no remaining length, do nothing
    if dragging_chain.remaining_length == 0 {
        return;
    }

    // get the tile mouse move events
    for event in tile_mouse_move_events.read() {
        // make sure the tile is in an adjacent tile to the current chain segment
        if !current_chain_segment.tile.is_adjacent(&event.0.tile) {
            continue;
        }

        // make sure there isn't already a chain segment at this position
        if q_chain_segments
            .iter()
            .any(|segment| segment.0.tile == event.0.tile)
        {
            continue;
        }

        let prev_chain_segs = q_chain
            .iter()
            .find(|(entity, _)| entity == &dragging_chain.e_chain);

        if let Some((_, children)) = prev_chain_segs {
            for child in children.iter() {
                let (mut segment, mut sprite, mut transform) =
                    q_chain_segments.get_mut(child).unwrap();
                if segment.tile != current_chain_segment.tile {
                    continue;
                }
                update_chain_segment(
                    &mut segment,
                    event.0.tile,
                    &mut sprite,
                    &mut transform,
                    &asset_server,
                );
            }
        }

        // update_chain_segment(
        //     current_chain_segment.tile,
        //     event.0.tile,
        //     &mut sprite,
        //     &mut transform,
        //     &asset_server,
        // );

        spawn_chain_segment(
            dragging_chain.e_chain,
            &mut commands,
            Some(current_chain_segment.tile),
            event.0.tile,
            &asset_server,
        );
        evw_chain_placed.write(ChainPlaced);

        dragging_chain.remaining_length -= 1;
        current_chain_segment.prev_tile = Some(current_chain_segment.tile);
        current_chain_segment.tile = event.0.tile;
    }
}

fn end_chain(
    mut commands: Commands,
    mouse_button: Res<ButtonInput<MouseButton>>,
    q_dragging_chain: Query<(Entity, &DraggingChain)>,
    mut q_selected_chain: Query<(&mut ChainButton, &Children), With<SelectedChain>>,
    mut q_stock_text: Query<&mut Text2d, With<ChainButtonStock>>,
    mut evw_chain_finished: EventWriter<ChainFinished>,
    q_obstacles: Query<&Obstacle>,
) {
    if !mouse_button.just_released(MouseButton::Left) {
        return;
    }

    // remove any dragging chains (should only be one, but lets be safe)
    for (entity, dragging_chain) in q_dragging_chain.iter() {
        commands.entity(entity).despawn();

        // if we didn't finish the chain, remove it
        if dragging_chain.remaining_length > 0 {
            commands.entity(dragging_chain.e_chain).despawn();
            continue;
        }

        // do a naive pathfind to make sure we didn't block the ship hold
        let pathing_grid = get_pathing_grid(q_obstacles);

        let start = BOAT_POINT;
        let end = HOLD_POINT;

        let path = pathing_grid.get_path_single_goal(start, end, false);

        if path.is_none() {
            commands.entity(dragging_chain.e_chain).despawn();
            continue;
        }

        // We placed a chain, update the stock
        let (mut chain_in_inventory, children) = q_selected_chain.single_mut().unwrap();
        chain_in_inventory.stock -= 1;
        for child in children.iter() {
            if let Ok(mut text) = q_stock_text.get_mut(child) {
                text.0 = format!("{}", chain_in_inventory.stock);
            }
        }
        evw_chain_finished.write(ChainFinished);
    }
}

// for now, we just spawn crates as chain segments
fn handle_crate_spawned(
    mut commands: Commands,
    mut crate_spawned_events: EventReader<CrateSpawned>,
    asset_server: ResMut<AssetServer>,
) {
    for event in crate_spawned_events.read() {
        let sprite = Sprite::from_image(asset_server.load("images/crate.png"));
        let tile = event.tile;
        commands.spawn((
            tile,
            Crate,
            Sprite {
                custom_size: Some(Vec2::splat(TILE_SIZE)),
                ..sprite
            },
            Obstacle { tile },
            tile.grid_coord_to_transform(3.0),
        ));
    }
}
pub fn plugin(app: &mut App) {
    app.add_systems(Startup, setup);
    app.add_systems(
        Update,
        (
            mouse_down_on_chain_button_in_inventory,
            begin_chain,
            drag_chain,
            end_chain,
        )
            .run_if(in_state(GameState::Building)),
    );
    app.add_systems(Update, handle_crate_spawned);
}
