use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};

use super::CORNERS;

#[derive(Clone, Component)]
pub struct HexCell {
    pub coordinates: (f32, f32),
    pub transform: Transform,
    pub color: Color,
    pub pbr_bundle: PbrBundle,
}

impl HexCell {
    pub fn new(
        coordinates: (f32, f32),
        transform: Transform,
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
            transform,
            ..default()
        };

        Self {
            coordinates,
            transform,
            color,
            pbr_bundle,
        }
    }
}
