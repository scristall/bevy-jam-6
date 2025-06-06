use bevy::prelude::*;

use crate::game::events::{TileMouseDown, TileMouseMove, TileMouseUp};
use crate::game::tile::{TILE_SIZE, Tile};

#[derive(Component, Debug)]
pub struct Chain {
    length: u32,
}

#[derive(Component, Debug)]
pub struct ChainInInventory {
    stock: u32,
    length: u32,
}

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

fn setup(mut commands: Commands) {
    // For now, just set a selected chain
    commands.spawn((
        SelectedChain,
        ChainInInventory {
            stock: 1,
            length: 5,
        },
    ));
}

fn begin_chain(
    mut commands: Commands,
    mut tile_clicked_events: EventReader<TileMouseDown>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    q_dragging_chain: Query<&DraggingChain>,
    q_selected_chain: Query<(&SelectedChain, &ChainInInventory), Without<DraggingChain>>,
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

    // if there is a tile clicked, create a new chain
    for event in tile_clicked_events.read() {
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
) {
    for _ in tile_mouse_up_events.read() {
        // remove any dragging chains (should only be one, but lets be safe)
        for (entity, dragging_chain) in q_dragging_chain.iter() {
            // if we didn't finish the chain, remove it
            if dragging_chain.remaining_length > 0 {
                commands.entity(dragging_chain.e_chain).despawn();
            }

            commands.entity(entity).despawn();
        }
    }
}

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, setup);
    app.add_systems(Update, begin_chain);
    app.add_systems(Update, drag_chain);
    app.add_systems(Update, end_chain);
}
