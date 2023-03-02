//! Hexagonal map module.
//!
//! Based on the excellent tutorial written by Jasper Flick.
//! See https://catlikecoding.com/unity/tutorials/hex-map.

use bevy::prelude::*;

/// Hexagon's outer radius.
const OUTER_RADIUS: f32 = 10.;

/// Hexagon's inner radius.
/// The inner radius is equals to √3/2 times the outer radius.
/// In our case it's equals to 5 * √3.
const INNER_RADIUS: f32 = OUTER_RADIUS * 0.866_025_4;

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

#[derive(Clone)]
pub struct HexCell {
    pub position: Vec3,
    pub color: Color,
    pub pbr_bundle: PbrBundle,
}

impl HexCell {
    pub fn new(
        position: Vec3,
        color: Color,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) -> Self {
        let pbr_bundle = PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: OUTER_RADIUS })),
            material: materials.add(color.into()),
            transform: Transform::from_translation(position),
            ..default()
        };

        Self {
            position,
            color,
            pbr_bundle,
        }
    }
}

#[derive(Default)]
pub struct HexGrid {
    pub width: u8,
    pub height: u8,
    pub cells: Vec<HexCell>,
}

impl HexGrid {
    pub fn new(width: u8, height: u8) -> Self {
        Self {
            width,
            height,
            ..default()
        }
    }

    pub fn render(
        &mut self,
        commands: &mut Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
        for z in 0..self.height {
            for x in 0..self.width {
                let position = Vec3::new(x as f32 * OUTER_RADIUS, 0., z as f32 * OUTER_RADIUS);
                let color = if x == 0 && z == 0 {
                    Color::ORANGE_RED
                } else if x % 2 == 0 {
                    Color::ALICE_BLUE
                } else {
                    Color::ORANGE
                };

                let cell = HexCell::new(position, color, &mut meshes, &mut materials);
                commands.spawn(cell.pbr_bundle.clone());

                self.cells.push(cell);
            }
        }
    }
}
