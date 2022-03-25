
use bevy::prelude::*;

use crate::helpers::math::progress;

use super::component::Particle;

fn move_in_angle(mut transform: Mut<Transform>, angle: f32, speed: f32) {
    transform.translation.x = transform.translation.x + (angle.sin() * speed);
    transform.translation.y = transform.translation.y + (angle.cos() * speed);
}

pub fn particle_movement(mut query: Query<(&mut Transform, &Particle)>) {
    for (t, p) in query.iter_mut() {
        move_in_angle(t, p.angle, p.speed);
    }
}

pub fn particle_killer(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &Particle)>,
) {
    for (entity, particle) in query.iter_mut() {
        let is_past_time = particle.max_life_ms + particle.time_spawned
            < time.time_since_startup().as_millis() as f64;

        if is_past_time {
            commands.entity(entity).despawn();
        }
    }
}

// Returns progress of natural  life from 0 to 1
fn get_life_progress(time: &Res<Time>, particle: &Particle) -> f32{
    let death_point = particle.time_spawned + particle.max_life_ms;
    let tss = time.time_since_startup().as_millis() as f64;
    let life_lived = 1. - ((death_point - tss) / particle.max_life_ms) as f32;
    return life_lived
}

pub fn particle_sizer(time: Res<Time>, mut query: Query<(&Particle, &mut Transform)>) {
    query.for_each_mut(|(particle, mut transform)| {
        let size = progress(particle.size_start, particle.size_end, get_life_progress(&time, particle));
        transform.scale = Vec3::new(size, size, size);
    })
}