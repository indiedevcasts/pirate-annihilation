//! Hexagonal map module.
//!
//! Based on the excellent tutorial written by Jasper Flick.
//! See https://catlikecoding.com/unity/tutorials/hex-map

use bevy::{prelude::*, render::render_resource::PrimitiveTopology};

/// Hexagon's outer radius.
const OUTER_RADIUS: f32 = 10.;

/// Hexagon's inner radius.
/// The inner radius is equals to √3/2 times the outer radius.
/// In our case it's equals to 5 * √3.
const INNER_RADIUS: f32 = OUTER_RADIUS * 0.866_025_4;

/// Hexagon's inner diameter.
const INNER_DIAMETER: f32 = INNER_RADIUS * 2.;

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

#[derive(Clone, Component)]
pub struct HexCell {
    pub coordinates: (f32, f32),
    pub position: Vec3,
    pub color: Color,
    pub pbr_bundle: PbrBundle,
}

impl HexCell {
    pub fn new(
        coordinates: (f32, f32),
        position: Vec3,
        color: Color,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) -> Self {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        // The local position is the center of the hex cell.
        let vertices: Vec<Vec3> = vec![position, position + CORNERS[0], position + CORNERS[1]];
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);

        let mut triangles: Vec<u32> = Vec::new();

        let pbr_bundle = PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: OUTER_RADIUS })),
            material: materials.add(color.into()),
            transform: Transform::from_translation(position),
            ..default()
        };

        Self {
            coordinates,
            position,
            color,
            pbr_bundle,
        }
    }

    pub fn render() {
        todo!()
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
                let (x, y, z) = (x as f32, 0., z as f32);
                let mut position = Vec3::new(x, y, z);
                const OFFSET: f32 = 0.5;

                // The offset is applied every 2 lines, hence the mpodulo.
                position.x = (x + z % 2. * OFFSET) * (INNER_DIAMETER);

                // In Bevy the forward direction is -Z, so
                // we need to inverse the z position.
                position.z = -z * (OUTER_RADIUS * 1.5);

                let color = if x == 0. && z == 0. {
                    Color::ORANGE_RED
                } else if x == 1. && z == 0. {
                    Color::GREEN
                } else if x == 0. && z == 1. {
                    Color::DARK_GREEN
                } else if x % 2. == 0. {
                    Color::ALICE_BLUE
                } else {
                    Color::ORANGE
                };

                let cell = HexCell::new((x, z), position, color, &mut meshes, &mut materials);
                commands.spawn(cell.pbr_bundle.clone());

                self.cells.push(cell);
            }
        }
    }
}
