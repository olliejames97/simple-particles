/**
 * Todo:
 * Refactor files, maybe seperate files
 *  - - 1 for components component spawner helpers, defautls etc.
 *  - 1 for related systems
 *  - eg.
 *  - -/particle/component.rs + /particle/systems.rs
 * Find out how plugins work
 * Replace size_start size_end stuff with the functionality to automate any property on particle
 * fn automate(particle.size,  end_val, particle.life_progress()) somethin like that
 */
use crate::components::{
    particle::{particle_killer, particle_movement, particle_sizer},
    particle_system::particle_spawn,
};
use bevy::{
    core::FixedTimestep,
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};
use components::{particle::Particle, particle_system::ParticleSystem};

mod components;
mod helpers;

fn main() {
    println!("Hello, world!");
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(60. * 2. / 60.))
                .with_system(particle_spawn),
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1.0 / 60.))
                .with_system(particle_movement),
        )
        .add_system(particle_killer)
        .add_system(particle_sizer)
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
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
