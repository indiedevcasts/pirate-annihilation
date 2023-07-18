use crate::materials::BiomeMaterial;

use super::{HexCell, INNER_DIAMETER, OUTER_RADIUS};
use bevy::prelude::*;

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
        mut materials: ResMut<Assets<BiomeMaterial>>,
    ) {
        for z in 0..self.height {
            for x in 0..self.width {
                let (x, y, z) = (x as f32, 0., z as f32);
                let mut position = Vec3::new(x, y, z);
                const OFFSET: f32 = 0.5;

                // The offset is applied every 2 lines, hence the mpodulo.
                position.x = (x + z % 2. * OFFSET) * (INNER_DIAMETER);

                // In Bevy the forward direction is -Z, so
                // we need to inverse the z position.
                position.z = -z * (OUTER_RADIUS * 1.5);

                // starting position
                let color = if x == 0. && z == 0. {
                    Color::ORANGE_RED
                // every even lines (considering camera facing z)
                } else if z % 2. == 0. {
                    Color::BLUE
                // every odd lines (considering camera facing z)
                } else {
                    Color::DARK_GREEN
                };

                let cell = HexCell::new(
                    (x, z),
                    Transform::from_translation(position),
                    color,
                    &mut meshes,
                    &mut materials,
                );
                commands.spawn(cell.mesh_bundle.clone());

                self.cells.push(cell);
            }
        }
    }
}
