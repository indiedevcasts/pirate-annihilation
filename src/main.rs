use bevy::render::camera::{CameraProjection, ScalingMode};
use bevy::{
    log::{Level, LogPlugin},
    prelude::*,
};
use dolly::prelude::*;

mod camera;
mod components;
mod systems;

use camera::CameraController;
use systems::hex_map::HexGrid;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(LogPlugin {
                    filter: "info,wgpu_core=warn,wgpu_hal=error,magnet=debug".into(),
                    level: Level::INFO,
                })
                .set(AssetPlugin {
                    watch_for_changes: true,
                    ..default()
                }),
        )
        .add_startup_system(setup)
        .add_system(camera::move_free_camera)
        .run();
}

fn setup(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(camera_setup());
    commands.spawn(camera_controller_setup());
    let mut hex_grid = HexGrid::new(6, 6);
    hex_grid.generate(meshes, materials);

    for cell in hex_grid.cells {
        commands.spawn(cell);
    }
}

fn camera_controller_setup() -> CameraController {
    let rig = CameraRig::builder()
        .with(Position::new(Vec3::Y))
        .with(YawPitch::new())
        .with(Smooth::new_position_rotation(1.0, 1.0))
        .build();

    CameraController { rig }
}

fn camera_setup() -> Camera3dBundle {
    Camera3dBundle {
        transform: Transform::from_xyz(0., 5., 20.).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    }
}
