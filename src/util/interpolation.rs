pub fn lerp(a: f32, b: f32, rate: f32) -> f32 {
    a * (1. - rate) + b * rate
}
