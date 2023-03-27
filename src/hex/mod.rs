//! Hexagonal map module.
//!
//! Based on the excellent tutorials written respectively by
//! Jasper Flick and by Amit Patel:
//!
//! - https://catlikecoding.com/unity/tutorials/hex-map
//! - https://www.redblobgames.com/grids/hexagons

use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};

// --> √3 = 1.73205080757
// --> √3/2 = 0.86602540378
// --> √3/2 * 10 = 8,660254037844386
// --> 5√3 = 8,660254037844386

// red
// horiz = width = sqrt(3) * size
// horiz = width = 0.86602540378 * 10
// horiz = width = 8,660254037844386

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
const CORNERS: [[f32; 3]; 6] = [
    [0., 0., OUTER_RADIUS],
    [INNER_RADIUS, 0., 0.5 * OUTER_RADIUS],
    [INNER_RADIUS, 0., -0.5 * OUTER_RADIUS],
    [0., 0., -OUTER_RADIUS],
    [-INNER_RADIUS, 0., -0.5 * OUTER_RADIUS],
    [-INNER_RADIUS, 0., 0.5 * OUTER_RADIUS],
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

        let vertices: Vec<[f32; 3]> = vec![
            [0., 0., 0.], // the local center of the cell
            CORNERS[0],
            CORNERS[1],
            CORNERS[2],
            CORNERS[3],
            CORNERS[4],
            CORNERS[5],
        ];

        // Each index is expressed in a clockwise way so the
        // side facing the camera is rendered.
        let indices = Indices::U16(vec![
            0, 1, 2, // 6
            0, 2, 3, // 1
            0, 3, 4, // 3
            0, 4, 5, // 4
            0, 5, 6, // 5
            0, 6, 1, // 6
        ]);
        let normals = vec![[0., 1., 0.]; 7];
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
        mesh.set_indices(Some(indices));
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);

        let pbr_bundle = PbrBundle {
            mesh: meshes.add(mesh),
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

                let cell = HexCell::new((x, z), position, color, &mut meshes, &mut materials);
                commands.spawn(cell.pbr_bundle.clone());

                self.cells.push(cell);
            }
        }
    }
}
