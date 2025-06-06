use bevy::prelude::*;

use crate::game::events::TileClicked;

#[derive(Component, Debug)]
pub struct Chain {
    length: u32
}

#[derive(Component, Debug)]
pub struct SelectedChain;

#[derive(Component, Debug)]
pub struct DraggingChain;

#[derive(Component, Debug)]
pub struct ChainSegment {
    x: i32,
    y: i32,
}

pub fn begin_chain(
    mut commands: Commands,
    mut tile_clicked_events: EventReader<TileClicked>,
    mut q_dragging_chain: Query<&SelectedChain, With<DraggingChain>>,
    mut q_selected_chain: Query<&SelectedChain, Without<DraggingChain>>,
) {
    // if there is a chain already being dragged, do nothing
    if q_dragging_chain.single().is_ok() {
        return;
    }

    // if there is a tile clicked, create a new chain
    for event in tile_clicked_events.read() {
        commands.spawn((
            DraggingChain,
            Chain { length: 1 },
        ));
    }
}

pub fn plugin(app: &mut App) {
    app.add_systems(Update, begin_chain);
}
