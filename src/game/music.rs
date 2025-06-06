use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, setup_music);
}

fn setup_music(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(AudioPlayer::new(
        asset_server.load("audio/music/wellerman.ogg"),
    ));
}
