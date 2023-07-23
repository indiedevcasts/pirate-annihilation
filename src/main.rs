use bevy::{
    asset::ChangeWatcher,
    log::{Level, LogPlugin},
    prelude::*,
};
use characters::traveler::{self, Traveler};
use dolly::prelude::*;
use materials::BiomeMaterial;
use std::{f32::consts::PI, time::Duration};

mod camera;
mod characters;
mod core;
mod hex;
mod materials;

use camera::CameraController;
use hex::HexGrid;

fn main() {
    let default_plugins = DefaultPlugins
        .set(LogPlugin {
            filter: "info,wgpu_core=warn,wgpu_hal=error,magnet=debug".into(),
            level: Level::INFO,
        })
        .set(AssetPlugin {
            watch_for_changes: ChangeWatcher::with_delay(Duration::from_millis(500)),
            ..default()
        });

    App::new()
        .add_plugins((default_plugins, MaterialPlugin::<BiomeMaterial>::default()))
        .add_systems(Startup, setup)
        .add_systems(Update, (camera::move_free_camera, traveler::travel))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    biome_materials: ResMut<Assets<BiomeMaterial>>,
    mut std_materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(light_setup());
    commands.spawn(camera_setup());
    commands.spawn(camera_controller_setup());
    HexGrid::new(6, 6).render(&mut commands, &mut meshes, biome_materials);
    commands
        .spawn(traveler_setup(&mut meshes, &mut std_materials))
        .insert(Traveler::default());
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

fn traveler_setup(
    meshes: &mut ResMut<Assets<Mesh>>,
    std_materials: &mut ResMut<Assets<StandardMaterial>>,
) -> PbrBundle {
    PbrBundle {
        mesh: meshes.add(shape::Cube::default().into()),
        material: std_materials.add(Color::BLACK.into()),
        ..default()
    }
}

fn camera_setup() -> Camera3dBundle {
    Camera3dBundle::default()
}
