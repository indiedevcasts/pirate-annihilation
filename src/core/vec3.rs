use bevy::math::Vec3;

/// Update a vector `Vec3` towards a target over time (`delta_time`).
/// The smoothness is achieved with a spring-damper like function.
///
/// Algorithm based on Game Programming Gems vol.4,
/// chapter 1.10 "Critically Damped Ease-In/Ease-Out Smoothing".
pub fn smooth_damp(
    from: Vec3,
    to: Vec3,
    mut velocity: Vec3,
    mut smoothness: f32,
    max_speed: f32,
    delta_time: f32,
) -> Vec3 {
    // The desired smoothness clamped to the minimum value.
    smoothness = f32::max(0.0001, smoothness);
    // Corresponds to the spring's natural frequency
    let omega = 2. / smoothness;

    let x = omega * delta_time;
    // Approximation
    let exp = 1. / (1. + x + 0.48 * x * x + 0.235 * x * x * x);

    let mut distance_x = from.x - to.x;
    let mut distance_y = from.y - to.y;
    let mut distance_z = from.z - to.z;

    let max_distance = max_speed * smoothness;
    distance_x = f32::min(f32::max(-max_distance, distance_x), max_distance);
    distance_y = f32::min(f32::max(-max_distance, distance_y), max_distance);
    distance_z = f32::min(f32::max(-max_distance, distance_z), max_distance);

    let temp_x = (velocity.x + omega * distance_x) * delta_time;
    let temp_y = (velocity.y + omega * distance_y) * delta_time;
    let temp_z = (velocity.z + omega * distance_z) * delta_time;

    velocity.x = (velocity.x - omega * temp_x) * exp;
    velocity.y = (velocity.y - omega * temp_y) * exp;
    velocity.z = (velocity.z - omega * temp_z) * exp;

    let x = to.x + (distance_x + temp_x) * exp;
    let y = to.y + (distance_y + temp_y) * exp;
    let z = to.z + (distance_z + temp_z) * exp;

    Vec3::new(x, y, z)
}
