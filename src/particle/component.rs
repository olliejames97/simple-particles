use crate::helpers::{math::progress, rand::_rand};
use bevy::prelude::*;
#[derive(Component, Clone)]
pub struct Particle {
    pub speed: f32,
    pub angle: f32,
    pub max_life_ms: f64,
    pub time_spawned: f64,
    pub size_start: f32,
    pub size_end: f32,
}

impl Default for Particle {
    fn default() -> Self {
        Particle {
            max_life_ms: (_rand(5.) as f64 + 40.) * 100., // todo: 0 for infinite
            speed: _rand(1.) + 0.2,
            angle: _rand(360.),
            time_spawned: 0.,
            size_start: 4.,
            size_end: 0.,
        }
    }
}


pub fn particle_sprite_default() -> SpriteBundle {
    let size = 4.;
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