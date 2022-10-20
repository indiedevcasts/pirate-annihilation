use crate::components::player::Player;
use bevy::prelude::*;

const MOVEMENT_SPEED: f32 = 500.;
const MAX_DEGREES: f32 = 360.;
const ROTATION_STEP: f32 = 0.5;
const FORWARD_STEP: f32 = 1.;

pub fn movement(
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    for mut transform in query.iter_mut() {
        let mut rotation_input = 0.;
        let mut forward_input = 0.;

        if input.pressed(KeyCode::Z) {
            forward_input = -FORWARD_STEP;

            if input.pressed(KeyCode::D) {
                rotation_input -= ROTATION_STEP;
            }
            if input.pressed(KeyCode::Q) {
                rotation_input += ROTATION_STEP;
            }
        }

        // Rotation
        transform.rotate_z(rotation_input * f32::to_radians(MAX_DEGREES) * time.delta_seconds());

        // Movement
        let direction = transform.rotation * Vec3::Y * forward_input;
        transform.translation += direction * MOVEMENT_SPEED * time.delta_seconds();
    }
}
