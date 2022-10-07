use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(LdtkPlugin)
        .add_startup_system(setup)
        .insert_resource(LevelSelection::Index(0))
        .register_ldtk_entity::<MyBundle>("LdtkEntity")
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Hot reload https://bevy-cheatbook.github.io/assets/hot-reload.html
    asset_server.watch_for_changes().unwrap();

    commands.spawn_bundle(Camera2dBundle::default());

    commands.spawn_bundle(LdtkWorldBundle {
        ldtk_handle: asset_server.load("boatmap.ldtk"),
        ..Default::default()
    });
}
#[derive(Component, Default)]
struct Character {
    pub name: String,
}

#[derive(Bundle, LdtkEntity)]
pub struct MyBundle {
    pouet: Character,
    #[sprite_sheet_bundle]
    #[bundle]
    sprite_bundle: SpriteSheetBundle,
}
