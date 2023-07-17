use bevy::{
    asset::ChangeWatcher,
    log::{Level, LogPlugin},
    prelude::*,
};
use dolly::prelude::*;
use std::{f32::consts::PI, time::Duration};

mod camera;
mod components;
mod core;
mod hex;
mod systems;

use camera::CameraController;
use hex::HexGrid;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(LogPlugin {
                    filter: "info,wgpu_core=warn,wgpu_hal=error,magnet=debug".into(),
                    level: Level::INFO,
                })
                .set(AssetPlugin {
                    watch_for_changes: ChangeWatcher::with_delay(Duration::from_millis(500)),
                    ..default()
                }),
        )
        .add_systems(Startup, setup)
        .add_systems(Update, camera::move_free_camera)
        .run();
}

fn setup(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(light_setup());
    commands.spawn(camera_setup());
    commands.spawn(camera_controller_setup());
    HexGrid::new(6, 6).render(&mut commands, meshes, materials);
}

fn light_setup() -> DirectionalLightBundle {
    DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            // π = 180° (in radians). π/4 = 45°.
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        ..default()
    }
}

fn camera_controller_setup() -> CameraController {
    let rig = CameraRig::builder()
        .with(Position::new(Vec3::new(50., 70., 80.)))
        .with(YawPitch::new().pitch_degrees(-35.))
        .with(Smooth::new_position_rotation(1.0, 1.0))
        .build();

    CameraController { rig }
}

fn camera_setup() -> Camera3dBundle {
    Camera3dBundle::default()
}
