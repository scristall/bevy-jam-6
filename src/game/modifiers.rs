use bevy::prelude::*;

#[derive(Component)]
pub struct GluePuddle;

#[derive(Component)]
pub struct Sticky {
    pub duration: f32,
}

pub fn glue_puddle_system(
    time: Res<Time>,
    mut commands: Commands,
    mut q_sticky_pirates: Query<(Entity, &mut Sticky)>,
) {
    for (entity, mut sticky) in q_sticky_pirates.iter_mut() {
        sticky.duration -= time.delta().as_secs_f32();
        if sticky.duration <= 0.0 {
            commands.entity(entity).remove::<Sticky>();
        }
    }
}

pub fn plugin(app: &mut App) {
    app.add_systems(Update, glue_puddle_system);
}
