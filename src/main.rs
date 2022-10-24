use bevy::render::camera::ScalingMode;
use bevy::{log::LogSettings, prelude::*};
use bevy_ecs_ldtk::prelude::*;

mod components;
mod systems;

use components::player::PlayerBundle;
use systems::{camera_system, player_system};

fn main() {
    App::new()
        .insert_resource(LogSettings {
            filter: "info,wgpu_core=warn,wgpu_hal=error,magnet=debug".into(),
            level: bevy::log::Level::INFO,
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(LdtkPlugin)
        .add_startup_system(setup)
        .add_system(camera_system::fit_inside_current_level)
        .add_system(player_system::movement)
        .insert_resource(LevelSelection::Index(0))
        .register_ldtk_entity::<PlayerBundle>("Player")
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    asset_server.watch_for_changes().unwrap();
    commands.spawn_bundle(camera_setup());
    commands.spawn_bundle(LdtkWorldBundle {
        ldtk_handle: asset_server.load("boatmap.ldtk"),
        ..Default::default()
    });
}

fn camera_setup() -> Camera2dBundle {
    let mut camera = Camera2dBundle::default();
    camera.projection.scaling_mode = ScalingMode::WindowSize;
    camera.projection.scale *= 1.5;

    camera
}
