use bevy::prelude::*;

use crate::game::game_state::GameState;

#[derive(Component)]
pub struct Oxygen(pub f32);

#[derive(Component)]
pub struct OxygenBar;

fn spawn_oxygen_bar(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    q_pirates: Query<(Entity, &Oxygen), Added<Oxygen>>,
) {
    for (e_pirate, oxygen) in q_pirates.iter() {
        let rect = Rectangle::new(100.0, 10.0);
        let color = Color::linear_rgba(1.0, 0.0, 0.0, 1.0);
        let mut transform = Transform::from_translation(Vec3::new(0.0, 70.0, 3.0));
        transform.scale.x = oxygen.0 / 100.0;
        commands.entity(e_pirate).with_child((
            OxygenBar,
            Mesh2d(meshes.add(rect)),
            MeshMaterial2d(materials.add(color)),
            transform,
        ));
    }
}

fn update_oxygen_bar(
    mut q_pirates: Query<(&Oxygen, &Children), Changed<Oxygen>>,
    mut q_bars: Query<&mut Transform, With<OxygenBar>>,
) {
    for (oxygen, children) in q_pirates.iter_mut() {
        for t_bar in children.iter() {
            let mut transform = q_bars.get_mut(t_bar).unwrap();
            transform.scale.x = oxygen.0 / 100.0;
        }
    }
}

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (spawn_oxygen_bar, update_oxygen_bar).run_if(in_state(GameState::WaveInProgress)),
    );
}
