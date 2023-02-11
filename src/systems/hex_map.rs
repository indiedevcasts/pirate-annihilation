//! Hexagonal map module.
//!
//! Based on the excellent tutorial written by Jasper Flick.
//! See https://catlikecoding.com/unity/tutorials/hex-map.

use bevy::{prelude::*, render::render_resource::PrimitiveTopology};
use std::sync::Mutex;

/// Hexagon's outer radius.
const OUTER_RADIUS: f32 = 10.;

/// Hexagon's inner radius.
/// The inner radius is equals to √3/2 times the outer radius.
/// In our case it's equals to 5 * √3.
const INNER_RADIUS: f32 = OUTER_RADIUS * 0.866025404;

/// Hexagon's corners.
/// Orientation is pointy side up ⬡.
const CORNERS: [Vec3; 6] = [
    Vec3::new(0., 0., OUTER_RADIUS),
    Vec3::new(INNER_RADIUS, 0., 0.5 * OUTER_RADIUS),
    Vec3::new(INNER_RADIUS, 0., -0.5 * OUTER_RADIUS),
    Vec3::new(0., 0., -OUTER_RADIUS),
    Vec3::new(-INNER_RADIUS, 0., -0.5 * OUTER_RADIUS),
    Vec3::new(-INNER_RADIUS, 0., 0.5 * OUTER_RADIUS),
];

const PLANE_SIZE: f32 = 10.;

pub struct HexCell;

#[derive(Default)]
pub struct HexGrid {
    pub width: u8,
    pub height: u8,
    pub cells: Vec<PbrBundle>,
}

impl HexGrid {
    pub fn new(width: u8, height: u8) -> Self {
        Self {
            width,
            height,
            ..default()
        }
    }
    pub fn generate(
        &mut self,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
        for z in 0..self.height {
            for x in 0..self.width {
                let position = Vec3::new(x as f32 * PLANE_SIZE, 0., z as f32 * PLANE_SIZE);
                let color = if x == 0 && z == 0 {
                    Color::ORANGE_RED
                } else if x % 2 == 0 {
                    Color::ALICE_BLUE
                } else {
                    Color::ORANGE
                };

                let plane = PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Plane { size: PLANE_SIZE })),
                    material: materials.add(color.into()),
                    transform: Transform::from_translation(position),
                    ..default()
                };

                self.cells.push(plane)
            }
        }
    }
}
