use crate::helpers::rand::_rand;
use bevy::prelude::*;

#[derive(Component, Clone)]
pub struct Particle {
    pub speed: f32,
    pub angle: f32,
    pub sprite: SpriteBundle,
}

impl Default for Particle {
    fn default() -> Self {
        Particle {
            speed: _rand(5.) + 0.2,
            angle: _rand(360.),
            sprite: SpriteBundle {
                transform: Transform {
                    scale: Vec3::new(10., 4., 4.),
                    ..Default::default()
                },
                sprite: Sprite {
                    color: Color::rgb(_rand(1.), _rand(1.), _rand(1.)),
                    ..Default::default()
                },
                ..Default::default()
            },
        }
    }
}

pub fn particle_sprite_default() -> SpriteBundle {
    SpriteBundle {
        transform: Transform {
            scale: Vec3::new(10., 4., 4.),
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
