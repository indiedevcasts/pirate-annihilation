use bevy::prelude::{debug, Input, KeyCode, Query, Res, With};
use heron::Velocity;

use crate::components::player::Player;

pub fn movement(input: Res<Input<KeyCode>>, mut query: Query<&mut Velocity, With<Player>>) {
    for mut velocity in query.iter_mut() {
        let right = if input.pressed(KeyCode::D) { 1. } else { 0. };
        let left = if input.pressed(KeyCode::Q) { 1. } else { 0. };

        velocity.linear.x = (right - left) * 200.;

        let up = if input.pressed(KeyCode::Z) { 1. } else { 0. };
        let down = if input.pressed(KeyCode::S) { 1. } else { 0. };

        debug!("movement : {left}, {right}, {up}, {down}");
        velocity.linear.y = (up - down) * 200.;
    }
}
