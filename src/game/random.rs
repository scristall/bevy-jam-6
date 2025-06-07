use bevy::prelude::*;

use rand_chacha::{ChaCha8Rng, rand_core::SeedableRng};

#[derive(Resource)]
pub struct RandomSource(pub ChaCha8Rng);

fn setup(mut commands: Commands) {
    let seeded_rng = ChaCha8Rng::seed_from_u64(198234592347712);
    commands.insert_resource(RandomSource(seeded_rng));
}

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, setup);
}
