use bevy::prelude::*;

use crate::game::events::PirateDeath;

pub fn plugin(app: &mut App) {
    app.add_systems(Update, on_pirate_death);
}

fn on_pirate_death(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut evr_pirate_death: EventReader<PirateDeath>,
) {
    for _ in evr_pirate_death.read() {
        commands.spawn(AudioPlayer::new(
            asset_server.load("audio/sound_effects/death.ogg"),
        ));
    }
}
