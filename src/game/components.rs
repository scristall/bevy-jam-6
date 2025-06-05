use bevy::prelude::*;

// Pirate components
#[derive(Component)]
pub struct Pirate;

#[derive(Component)]
pub struct Oxygen(pub f32);

#[derive(Component)]
pub struct MovementSpeed(pub f32);

#[derive(Component)]
pub struct Path(pub Vec<Vec2>);

#[derive(Component)]
pub struct CurrentTarget(pub Vec2);

// Tile components
#[derive(Component)]
pub struct Tile;

#[derive(Component)]
pub struct Position(pub IVec2);

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub enum TileType {
    Empty,
    Chain,
    Wall,
    Gold,
}

#[derive(Component)]
pub struct TileProperties {
    pub oxygen_drain: f32,
    pub walkable: bool,
}

// Spawner components
#[derive(Component)]
pub struct Spawner;

#[derive(Component)]
pub struct SpawnTimer(pub Timer);

// Gold components
#[derive(Component)]
pub struct Gold;

#[derive(Component)]
pub struct TreasureCount(pub u32);

// UI components
#[derive(Component)]
pub struct OxygenBar;

#[derive(Component)]
pub struct WaveText;

#[derive(Component)]
pub struct GameOverUI;

// Game state
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    Playing,
    GameOver,
    Paused,
}

// Resources
#[derive(Resource)]
pub struct GameConfig {
    pub grid_size: IVec2,
    pub spawn_interval: f32,
    pub initial_oxygen: f32,
    pub base_movement_speed: f32,
    pub game_state: GameState,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            grid_size: IVec2::new(20, 15),
            spawn_interval: 5.0,
            initial_oxygen: 100.0,
            base_movement_speed: 2.0,
            game_state: GameState::Playing,
        }
    }
}

#[derive(Resource)]
pub struct WaveState {
    pub current_wave: u32,
    pub pirates_per_wave: u32,
    pub pirates_spawned: u32,
}

impl Default for WaveState {
    fn default() -> Self {
        Self {
            current_wave: 1,
            pirates_per_wave: 5,
            pirates_spawned: 0,
        }
    }
} 