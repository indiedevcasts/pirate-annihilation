use bevy::prelude::Component;

pub mod traveler;

#[derive(Component)]
pub struct MovementSpeed(f32);
