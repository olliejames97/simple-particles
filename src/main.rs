use crate::components::{
    particle::{particle_killer, particle_movement},
    particle_system::particle_spawn,
};
use bevy::{core::FixedTimestep, prelude::*};
use components::{particle::Particle, particle_system::ParticleSystem};

mod components;
mod helpers;

fn main() {
    println!("Hello, world!");
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(particle_spawn)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1.0 / 60.))
                .with_system(particle_movement),
        )
        .add_system(particle_killer)
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
    commands.spawn_bundle(SpriteBundle {
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            scale: Vec3::new(20., 20., 0.),
            ..Default::default()
        },
        sprite: Sprite {
            color: Color::rgb(0., 0., 0.0),
            ..Default::default()
        },
        ..Default::default()
    });
    commands.spawn().insert(ParticleSystem::default());
    commands.spawn().insert(Particle::default());
}
