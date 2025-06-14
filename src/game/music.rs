use bevy::{audio::Volume, prelude::*};
use rand::prelude::*;

use crate::game::events::{WaveComplete, WaveStarted};

#[derive(Component)]
pub struct Music;

fn set_volume(mut global_volume: ResMut<GlobalVolume>) {
    global_volume.volume = Volume::Linear(0.3);
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

const MUSIC_LIST: [&str; 3] = [
    "audio/music/wellerman.ogg",
    "audio/music/wellerman2.ogg",
    "audio/music/wellerman3.ogg",
];

fn wave_start_music(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut evr_wave_started: EventReader<WaveStarted>,
) {
    for _ in evr_wave_started.read() {
        let mut rng = rand::thread_rng();
        let music_idx = rng.gen_range(0..MUSIC_LIST.len());
        commands.spawn((
            Music,
            AudioPlayer::new(asset_server.load(MUSIC_LIST[music_idx])),
            PlaybackSettings {
                mode: bevy::audio::PlaybackMode::Loop,
                volume: Volume::Linear(0.6),
                ..default()
            },
        ));
    }
}

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, set_volume);
    app.add_systems(Update, (wave_start_music, wave_end_music));
}
