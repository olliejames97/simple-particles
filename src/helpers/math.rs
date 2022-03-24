pub fn progress(start: f32, end: f32, progress: f32) -> f32 {
    let diff = end - start;
    diff * progress + start
}
