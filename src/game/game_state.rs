use bevy::prelude::*;

#[derive(Resource, Debug, Default, Clone, Copy, PartialEq, Eq, Hash, States)]
pub enum GameState {
    #[default]
    TitleScreen,
    Tutorial,
    Building,
    WaveInProgress,
    Prize,
    Modifier,

    #[allow(unused)]
    GameOver,
}
