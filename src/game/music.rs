use bevy::{audio::Volume, prelude::*};

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, setup_music);
}

fn setup_music(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut global_volume: ResMut<GlobalVolume>,
) {
    global_volume.volume = Volume::Linear(0.1);
    commands.spawn((
        AudioPlayer::new(asset_server.load("audio/music/wellerman.ogg")),
        PlaybackSettings::LOOP,
    ));
}
