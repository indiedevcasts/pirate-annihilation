use bevy::prelude::*;

// https://github.com/clynamen/bevy_terrain/blob/0.0.1/src/gizmo.rs
pub fn setup(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    transform: Transform,
) {
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Icosphere {
                radius: 0.1,
                subdivisions: 10,
            })),
            material: materials.add(Color::rgb(0.0, 0.0, 0.0).into()),
            transform: transform,
            ..Default::default()
        })
        .with_children(|builder| {
            builder.spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 0.1 })),
                material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
                transform: Transform::from_translation(Vec3::new(1.0, 0.0, 0.0)),
                ..Default::default()
            });
            builder.spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 0.1 })),
                material: materials.add(Color::rgb(0.0, 1.0, 0.0).into()),
                transform: Transform::from_translation(Vec3::new(0.0, 1.0, 0.0)),
                ..Default::default()
            });
            builder.spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 0.1 })),
                material: materials.add(Color::rgb(0.0, 0.0, 1.0).into()),
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
                ..Default::default()
            });
        });
}
