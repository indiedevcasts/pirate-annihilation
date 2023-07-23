use super::CORNERS;
use crate::materials::BiomeMaterial;
use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};

#[derive(Component)]
pub struct HexCell {
    pub coordinates: (f32, f32),
}

#[derive(Clone, Component)]
pub struct HexCellMesh {
    pub transform: Transform,
    pub color: Color,
    pub mesh_bundle: MaterialMeshBundle<BiomeMaterial>,
}

impl HexCellMesh {
    pub fn new(
        transform: Transform,
        color: Color,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<BiomeMaterial>>,
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

        let mesh_bundle = MaterialMeshBundle {
            mesh: meshes.add(mesh),
            material: materials.add(BiomeMaterial {
                color,
                alpha_mode: AlphaMode::Blend,
            }),
            transform,
            ..default()
        };

        Self {
            transform,
            color,
            mesh_bundle,
        }
    }
}
