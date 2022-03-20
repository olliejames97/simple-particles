use crate::helpers::rand::_rand;
use bevy::prelude::*;

#[derive(Component, Clone)]
pub struct Particle {
    speed: f32,
    angle: f32,
    pub sprite: SpriteBundle,
}

impl Default for Particle {
    fn default() -> Self {
        Particle {
            speed: _rand(20.0) + 10.,
            angle: _rand(360.),
            sprite: SpriteBundle {
                transform: Transform {
                    scale: Vec3::new(4., 4., 4.),
                    ..Default::default()
                },
                sprite: Sprite {
                    color: Color::rgb(1., 0., 0.),
                    ..Default::default()
                },
                ..Default::default()
            },
        }
    }
}

impl Particle {
    pub fn set_position(&mut self, x: f32, y: f32) {
        self.sprite.transform.translation = Vec3::new(x, y, 0.);
    }
    fn get_position(&self) -> (f32, f32) {
        (
            self.sprite.transform.translation.x,
            self.sprite.transform.translation.y,
        )
    }
}

fn move_in_angle(transform: &mut Transform, angle: f32, speed: f32) {
    transform.translation.x += angle.sin() * speed;
    transform.translation.y += angle.cos() * speed;
}

pub fn particle_movement(mut query: Query<&Particle>) {
    for p in query.iter_mut() {
        let mut transform = p.sprite.transform;
        move_in_angle(&mut transform, p.angle, p.speed);
    }
}
