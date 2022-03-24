use crate::helpers::rand::_rand;
use bevy::prelude::*;
#[derive(Clone)]
struct FloatOverTime(f32, f32);
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

pub fn particle_sizer(time: Res<Time>, mut query: Query<(&Particle, &mut Transform)>) {
    query.for_each_mut(|(particle, mut transform)| {
        let death_point = particle.time_spawned + particle.max_life_ms;
        let tss = time.time_since_startup().as_millis() as f64;
        let life_lived = 1. - ((death_point - tss) / particle.max_life_ms) as f32;
        // time spawned = 100
        // max_life = 5
        // death_point = time spawned + max_life = 105
        // time since startup = 102.5
        // percent life =  death point - time since / max_life
        // 2.5 * 2 = 5
        //
        // start_size = 0
        // max_size = 15
        // life_lived = 0.5
        // current_size = 30
        // start = 50
        //  end = 105
        // percent = 0.5
        // diff = end - start = 55
        //  55 * 0.5 + 50 = 22.5 + 50 =  diff *
        // current = x
        let start_size = particle.size_start;
        let max_size = particle.size_end;
        let diff = max_size - start_size;
        let size = diff * life_lived + start_size;
        transform.scale = Vec3::new(size, size, size);
    })
}
