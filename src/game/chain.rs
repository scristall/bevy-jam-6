use bevy::prelude::*;

use crate::game::events::{TileMouseDown, TileMouseMove, TileMouseUp};
use crate::game::game_state::GameState;
use crate::game::mouse::MousePos;
use crate::game::tile::{TILE_SIZE, Tile};

const CHAIN_IN_INVENTORY_SIZE: f32 = 64.0;

#[derive(Component, Debug)]
pub struct Chain {
    length: u32,
}

#[derive(Component, Debug)]
pub struct MainInventory;

#[derive(Component, Debug)]
pub struct ChainInInventory {
    stock: u32,
    length: u32,
}

#[derive(Component, Debug)]
pub struct ChainInInventoryStock;

#[derive(Component, Debug)]
pub struct ChainInInventoryLength;

#[derive(Component, Debug)]
pub struct SelectedChain;

#[derive(Component, Debug)]
pub struct DraggingChain {
    remaining_length: u32,
    e_chain: Entity,
}

#[derive(Component, Debug)]
pub struct ChainSegment(pub Tile);

fn spawn_chain_segment(
    e_chain: Entity,
    commands: &mut Commands,
    tile: Tile,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    let rect = Rectangle::new(TILE_SIZE, TILE_SIZE);
    let color = Color::linear_rgba(1.0, 0.0, 0.0, 1.0);

    commands.entity(e_chain).with_children(|parent| {
        parent.spawn((
            tile,
            Mesh2d(meshes.add(rect)),
            MeshMaterial2d(materials.add(color)),
            ChainSegment(tile),
            tile.grid_coord_to_transform(3.0),
        ));
    });
}

pub fn spawn_chain_in_inventory(
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
        parent
            .spawn((
                Sprite::from_image(asset_server.load("images/chain.png")),
                ChainInInventory { stock, length },
                Transform::from_xyz(pos.x, pos.y, 0.0),
            ))
            .with_children(|parent| {
                parent.spawn((
                    ChainInInventoryStock,
                    Text2d::new(format!("{}", stock)),
                    text_font.clone(),
                    Transform::from_xyz(30.0, 30.0, 0.5),
                    TextColor(Color::linear_rgb(0.0, 0.0, 1.0)),
                ));

                parent.spawn((
                    ChainInInventoryLength,
                    Text2d::new(format!("{}", length)),
                    text_font.clone(),
                    Transform::from_xyz(-30.0, -30.0, 0.5),
                    TextColor(Color::linear_rgb(1.0, 0.0, 0.0)),
                ));
            });
    });
}

fn mouse_down_on_chain_in_inventory(
    mut commands: Commands,
    mouse_pos: Res<MousePos>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut q_unselected_chain: Query<
        (Entity, &ChainInInventory, &mut Sprite, &GlobalTransform),
        Without<SelectedChain>,
    >,
    mut q_selected_chain: Query<(
        Entity,
        &SelectedChain,
        &mut Sprite,
        &GlobalTransform,
        &ChainInInventory,
    )>,
) {
    let mut new_chain_selected = false;

    for (entity, _, mut sprite, transform) in q_unselected_chain.iter_mut() {
        if mouse_pos.is_in(
            transform.translation().truncate(),
            Vec2::splat(CHAIN_IN_INVENTORY_SIZE),
        ) {
            if mouse_button.just_pressed(MouseButton::Left) {
                sprite.color = Color::linear_rgba(0.0, 1.0, 0.0, 1.0);
                commands.entity(entity).insert(SelectedChain);
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

fn setup(mut commands: Commands, mut asset_server: ResMut<AssetServer>) {
    let e_main_inventory = commands
        .spawn((
            MainInventory,
            Transform::from_xyz(0.0, 0.0, 5.0),
            Visibility::Visible,
        ))
        .id();
    spawn_chain_in_inventory(
        &mut commands,
        e_main_inventory,
        1,
        9,
        &mut asset_server,
        Vec2::new(-300.0, -400.0),
    );
    spawn_chain_in_inventory(
        &mut commands,
        e_main_inventory,
        1,
        6,
        &mut asset_server,
        Vec2::new(0.0, -400.0),
    );
    spawn_chain_in_inventory(
        &mut commands,
        e_main_inventory,
        2,
        3,
        &mut asset_server,
        Vec2::new(300.0, -400.0),
    );
}

fn begin_chain(
    mut commands: Commands,
    mut tile_clicked_events: EventReader<TileMouseDown>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    q_dragging_chain: Query<&DraggingChain>,
    q_selected_chain: Query<(&SelectedChain, &ChainInInventory), Without<DraggingChain>>,
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
            .any(|segment| segment.0 == event.0.tile)
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
            ChainSegment(event.0.tile),
        ));

        spawn_chain_segment(
            e_chain,
            &mut commands,
            event.0.tile,
            &mut meshes,
            &mut materials,
        );
    }
}

fn drag_chain(
    mut commands: Commands,
    mut tile_mouse_move_events: EventReader<TileMouseMove>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut q_dragging_chain: Query<(&mut DraggingChain, &mut ChainSegment)>,
    q_chain_segments: Query<&ChainSegment, Without<DraggingChain>>,
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
        if !current_chain_segment.0.is_adjacent(&event.0.tile) {
            continue;
        }

        // make sure there isn't already a chain segment at this position
        if q_chain_segments
            .iter()
            .any(|segment| segment.0 == event.0.tile)
        {
            continue;
        }

        dragging_chain.remaining_length -= 1;
        current_chain_segment.0 = event.0.tile;

        spawn_chain_segment(
            dragging_chain.e_chain,
            &mut commands,
            event.0.tile,
            &mut meshes,
            &mut materials,
        );
    }
}

fn end_chain(
    mut commands: Commands,
    mut tile_mouse_up_events: EventReader<TileMouseUp>,
    q_dragging_chain: Query<(Entity, &DraggingChain)>,
    mut q_selected_chain: Query<(&mut ChainInInventory, &Children), With<SelectedChain>>,
    mut q_stock_text: Query<&mut Text2d, With<ChainInInventoryStock>>,
) {
    for _ in tile_mouse_up_events.read() {
        // remove any dragging chains (should only be one, but lets be safe)
        for (entity, dragging_chain) in q_dragging_chain.iter() {
            commands.entity(entity).despawn();

            // if we didn't finish the chain, remove it
            if dragging_chain.remaining_length > 0 {
                commands.entity(dragging_chain.e_chain).despawn();
            } else {
                // We placed a chain, update the stock
                let (mut chain_in_inventory, children) = q_selected_chain.single_mut().unwrap();
                chain_in_inventory.stock -= 1;
                for child in children.iter() {
                    if let Ok(mut text) = q_stock_text.get_mut(child) {
                        text.0 = format!("{}", chain_in_inventory.stock);
                    }
                }
            }
        }
    }
}

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, setup);
    app.add_systems(
        Update,
        (
            mouse_down_on_chain_in_inventory,
            begin_chain,
            drag_chain,
            end_chain,
        )
            .run_if(in_state(GameState::Building)),
    );
}
