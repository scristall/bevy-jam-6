use bevy::prelude::*;

#[derive(Component)]
pub struct Background;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Background,
        Sprite {
            image: asset_server.load("images/background.png"),
            ..default()
        },
    ));
}

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, setup);
}
