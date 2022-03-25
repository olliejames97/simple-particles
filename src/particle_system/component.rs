use bevy::prelude::*;

#[derive(Component)]
pub struct ParticleSystem {
    pub transform: Transform,
    pub spawn_rate: i64,
    
}



impl Default for ParticleSystem {
    fn default() -> Self {
        ParticleSystem {
            transform: Transform {
                translation: Vec3::new(0., 0., 0.),
                ..Default::default()
            },
            spawn_rate: 1000,
        }
    }
}
