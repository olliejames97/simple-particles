use super::particle::Particle;
use bevy::{
    math::Vec3,
    prelude::{Commands, Component, Query, Transform},
};
#[derive(Component)]
pub struct ParticleSystem {
    particle: Particle,
    transform: Transform,
    spawn_rate: i64,
}

impl Default for ParticleSystem {
    fn default() -> Self {
        ParticleSystem {
            particle: Particle::default(),
            transform: Transform {
                translation: Vec3::new(0., 0., 0.),
                ..Default::default()
            },
            spawn_rate: 3,
        }
    }
}

pub fn particle_spawn(mut commands: Commands, query: Query<&ParticleSystem>) {
    for spawner in query.iter() {
        for _ in 1..spawner.spawn_rate {
            let mut clone = spawner.particle.clone();
            clone.set_position(
                spawner.transform.translation.x,
                spawner.transform.translation.y,
            );
            commands.spawn_bundle(clone.sprite).insert();
        }
    }
}

// add particles to scene with .spawnBundle(Sprite {}) instead
// which should take take properties from the particle component, but the sprite should
// be added on spawn... not when setting up the struct (maybe idk ?)
