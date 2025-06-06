use bevy::prelude::*;


#[derive(Component)]
struct GoldBarText;

#[derive(Resource, Default)]
struct GoldAmount {
    value: i32,
}

fn spawn_gold_bar_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        // Accepts a `String` or any type that converts into a `String`, such as `&str`
        Text2d::new("Gold: 0"),
        TextFont {
            // This font is loaded and will be used instead of the default font.
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: 55.0,
            ..default()
        },
        TextShadow::default(),
        Transform::from_xyz(850.0, 330.0, 5.0),
        TextColor(Color::linear_rgba(0.0, 0.0, 0.0, 1.0)),
    ));
}


pub struct GoldBarTextPlugin;

impl Plugin for GoldBarTextPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GoldAmount>()
            .add_systems(Startup, spawn_gold_bar_text);
    }
}

