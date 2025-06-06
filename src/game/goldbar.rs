use bevy::prelude::*;
use crate::game::tile::{Tile, TILE_SIZE, GRID_X_START, GRID_Y_START};
use crate::game::components::{Position, TileType};
use crate::game::goldbar_text::GoldAmount;
use crate::game::events::{GoldBarCollected, GoldBarDropped};

pub const TOTAL_GOLD_BARS: i32 = 18;
pub const GOLD_ROOM_X: i32 = 27;
pub const GOLD_ROOM_Y: i32 = 0;

#[derive(Component)]
pub struct Gold;

fn spawn_gold_bar(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    pos: IVec2,
    gold_amount: &mut ResMut<GoldAmount>,
) -> Entity {
    let tile = Tile { x: pos.x, y: pos.y };
    let entity = commands.spawn((
        Gold,
        Position(pos),
        TileType::Gold,
        Sprite {
            image: asset_server.load("images/gold_bar.png"),
            ..default()
        },
        tile.grid_coord_to_transform(2.0),
        tile,
    )).id();
    gold_amount.value += 1;
    return entity;
}

fn despawn_gold_bar(
    commands: &mut Commands,
    entity: Entity,
    gold_amount: &mut ResMut<GoldAmount>,
) {
    gold_amount.value -= 1;
    commands.entity(entity).despawn();
}

pub fn spawn_gold_bars(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    mut gold_amount: ResMut<GoldAmount>,
) {
    let mut gold_positions = Vec::with_capacity(TOTAL_GOLD_BARS as usize);
    for x in GOLD_ROOM_X..=GOLD_ROOM_X+1 {
        for y in GOLD_ROOM_Y..=GOLD_ROOM_Y+TOTAL_GOLD_BARS/2-1 {
            gold_positions.push(IVec2::new(x, y));
        }
    }

    for pos in gold_positions {
        spawn_gold_bar(commands, asset_server, pos, &mut gold_amount);
    }
}

fn handle_gold_collected(
    mut commands: Commands,
    mut gold_amount: ResMut<GoldAmount>,
    mut gold_collected_events: EventReader<GoldBarCollected>,
    gold_bars: Query<&Tile, With<Gold>>,
) {
    for event in gold_collected_events.read() {
        for tile in gold_bars.iter() {
            if *tile == event.0.tile {
                // despawn the gold bar
                despawn_gold_bar(&mut commands, event.0.entity, &mut gold_amount);
            }
        }
    }
}

fn handle_gold_dropped(
    mut commands: Commands,
    mut gold_amount: ResMut<GoldAmount>,
    mut gold_dropped_events: EventReader<GoldBarDropped>,
    asset_server: Res<AssetServer>,
) {
    for event in gold_dropped_events.read() {
        // Spawn a new gold bar at the tile position
        let pos = IVec2::new(event.0.tile.x, event.0.tile.y);
        spawn_gold_bar(&mut commands, &asset_server, pos, &mut gold_amount);
    }
}


pub fn plugin(app: &mut App) {
    app.add_systems(Update, handle_gold_collected);
    app.add_systems(Update, handle_gold_dropped);
}

