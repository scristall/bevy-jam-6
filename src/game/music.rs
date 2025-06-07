use bevy::{audio::Volume, prelude::*};

use crate::game::events::{WaveComplete, WaveStarted};

#[derive(Component)]
pub struct Music;

fn set_volume(mut global_volume: ResMut<GlobalVolume>) {
    global_volume.volume = Volume::Linear(0.1);
}

fn wave_end_music(
    mut commands: Commands,
    q_music: Query<Entity, With<Music>>,
    mut evr_wave_complete: EventReader<WaveComplete>,
) {
    for _ in evr_wave_complete.read() {
        for entity in q_music.iter() {
            commands.entity(entity).despawn();
        }
    }
}

fn wave_start_music(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut evr_wave_started: EventReader<WaveStarted>,
) {
    for _ in evr_wave_started.read() {
        commands.spawn((
            Music,
            AudioPlayer::new(asset_server.load("audio/music/wellerman.ogg")),
            PlaybackSettings::LOOP,
        ));
    }
}

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, set_volume);
    app.add_systems(Update, (wave_start_music, wave_end_music));
}
