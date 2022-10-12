use bevy::prelude::{Input, KeyCode, Query, Res, Transform, Vec3, With};

use crate::components::player::Player;

const TIME_STEP: f32 = 1.0 / 60.0;

pub fn movement(input: Res<Input<KeyCode>>, mut query: Query<&mut Transform, With<Player>>) {
    for mut transform in query.iter_mut() {
        let mut rotation_factor = 0.0;
        let mut movement_factor = 0.0;
        if input.pressed(KeyCode::Z) {
            movement_factor -= 1.0;
            if input.pressed(KeyCode::D) {
                rotation_factor -= 0.5;
            }
            if input.pressed(KeyCode::Q) {
                rotation_factor += 0.5;
            }
        }

        transform.rotate_z(rotation_factor * f32::to_radians(360.0) * TIME_STEP);

        // get the ship's forward vector by applying the current rotation to the ships initial facing vector
        let movement_direction = transform.rotation * Vec3::Y;

        // get the distance the ship will move based on direction, the ship's movement speed and delta time
        let movement_distance = movement_factor * 500.0 * TIME_STEP;

        let translation_delta = movement_direction * movement_distance;

        transform.translation += translation_delta;
    }
}
