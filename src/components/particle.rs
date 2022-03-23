use crate::helpers::rand::_rand;
use bevy::prelude::*;

#[derive(Component, Clone)]
pub struct Particle {
    pub speed: f32,
    pub angle: f32,
    pub max_life: f64,
    pub time_spawned: f64,
}

impl Default for Particle {
    fn default() -> Self {
        Particle {
            max_life: _rand(5.) as f64 + 1., // 0 for infinite
            speed: _rand(1.) + 0.2,
            angle: _rand(360.),
            time_spawned: 0.,
        }
    }
}

pub fn particle_sprite_default() -> SpriteBundle {
    let size = _rand(4.) + 2.;
    SpriteBundle {
        transform: Transform {
            scale: Vec3::new(size, size, size),
            ..Default::default()
        },
        sprite: Sprite {
            color: Color::rgb(_rand(1.), _rand(1.), _rand(1.)),
            ..Default::default()
        },
        ..Default::default()
    }
}

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
        let is_past_time = particle.max_life + particle.time_spawned < time.seconds_since_startup();

        if is_past_time {
            commands.entity(entity).despawn();
        }
    }
}
