use super::particle::{particle_sprite_default, Particle};
use bevy::{
    math::Vec3,
    prelude::{Commands, Component, Query, Transform},
    sprite::SpriteBundle,
};
#[derive(Component)]
pub struct ParticleSystem {
    transform: Transform,
    spawn_rate: i64,
}

impl Default for ParticleSystem {
    fn default() -> Self {
        ParticleSystem {
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
            commands
                .spawn_bundle(SpriteBundle {
                    transform: Transform {
                        translation: spawner.transform.translation,
                        ..particle_sprite_default().transform
                    },
                    ..particle_sprite_default()
                })
                .insert(Particle {
                    ..Default::default()
                });
        }
    }
}
