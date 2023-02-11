use bevy::render::camera::ScalingMode;
use bevy::{
    log::{Level, LogPlugin},
    prelude::*,
};

mod components;
mod systems;

use components::player::PlayerBundle;
use systems::hex_map::HexGrid;
use systems::{camera_system, player_system};

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
        .run();
}

fn setup(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(camera_setup());
    let mut hex_grid = HexGrid::new(6, 6);
    hex_grid.generate(meshes, materials);

    for cell in hex_grid.cells {
        commands.spawn(cell);
    }
}

fn camera_setup() -> Camera3dBundle {
    Camera3dBundle {
        transform: Transform::from_xyz(0., 5., 20.).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    }
}
