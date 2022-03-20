use rand::Rng;
pub fn _rand(max: f32) -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen::<f32>() * max
}
