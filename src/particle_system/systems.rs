use bevy::prelude::*;
use crate::particle::component::{particle_sprite_default, Particle};

use super::component::ParticleSystem;

pub fn particle_spawn(time: Res<Time>, mut commands: Commands, query: Query<&ParticleSystem>) {
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
                    time_spawned: time.time_since_startup().as_millis() as f64,
                    ..Default::default()
                });
        }
    }
}
