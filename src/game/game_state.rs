use bevy::prelude::*;

#[derive(Resource, Debug, Default, Clone, Copy, PartialEq, Eq, Hash, States)]
pub enum GameState {
    #[default]
    Tutorial,
    Building,
    WaveInProgress,
    Prize,

    #[allow(unused)]
    GameOver,
}
