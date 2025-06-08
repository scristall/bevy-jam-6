use bevy::{audio::Volume, prelude::*};

use crate::game::events::*;

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, wind_ocean_noise);
    app.add_systems(Update, on_pirate_death);
    app.add_systems(Update, on_gold_pickup);
    app.add_systems(Update, on_gold_drop);
    app.add_systems(Update, on_prize_collect);
    app.add_systems(Update, on_wave_start);
    app.add_systems(Update, on_play_click);
    app.add_systems(Update, on_play_long_click);
    app.add_systems(Update, on_chain_placed);
    app.add_systems(Update, on_chain_finished);
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

fn on_gold_pickup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut evr_gold_pickup: EventReader<GoldBarCollected>,
) {
    for _ in evr_gold_pickup.read() {
        commands.spawn(AudioPlayer::new(
            asset_server.load("audio/sound_effects/gold_pickup.ogg"),
        ));
    }
}

fn on_gold_drop(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut evr_gold_drop: EventReader<GoldBarDropped>,
) {
    for _ in evr_gold_drop.read() {
        commands.spawn(AudioPlayer::new(
            asset_server.load("audio/sound_effects/gold_drop.ogg"),
        ));
    }
}

fn on_wave_start(
    mut evr_wave_started: EventReader<WaveStarted>,
    mut evw: EventWriter<PlayLongClickSFX>,
) {
    for _ in evr_wave_started.read() {
        evw.write(PlayLongClickSFX);
    }
}

fn on_play_click(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut evr: EventReader<PlayClickSFX>,
) {
    for _ in evr.read() {
        commands.spawn(AudioPlayer::new(
            asset_server.load("audio/sound_effects/click_reverb_hover.ogg"),
        ));
    }
}

fn on_play_long_click(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut evr: EventReader<PlayLongClickSFX>,
) {
    for _ in evr.read() {
        commands.spawn(AudioPlayer::new(
            asset_server.load("audio/sound_effects/click_reverb.ogg"),
        ));
    }
}

fn on_prize_collect(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut evr_prize: EventReader<PrizeCollected>,
) {
    for _ in evr_prize.read() {
        commands.spawn(AudioPlayer::new(
            asset_server.load("audio/sound_effects/power_up_jingle.ogg"),
        ));
    }
}

fn on_chain_placed(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut evr: EventReader<ChainPlaced>,
) {
    for _ in evr.read() {
        commands.spawn(AudioPlayer::new(
            asset_server.load("audio/sound_effects/chain_placed.ogg"),
        ));
    }
}

fn on_chain_finished(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut evr: EventReader<ChainFinished>,
) {
    for _ in evr.read() {
        commands.spawn(AudioPlayer::new(
            asset_server.load("audio/sound_effects/chain_finished.ogg"),
        ));
    }
}

fn wind_ocean_noise(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        AudioPlayer::new(asset_server.load("audio/sound_effects/ocean_wind_noise.ogg")),
        PlaybackSettings {
            mode: bevy::audio::PlaybackMode::Loop,
            volume: Volume::Linear(0.5),
            ..default()
        },
    ));
}
