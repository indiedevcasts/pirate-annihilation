use bevy::prelude::{Bundle, Component, SpriteBundle};
use bevy_ecs_ldtk::{EntityInstance, LdtkEntity, Worldly};
use heron::Velocity;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Player;

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct PlayerBundle {
    #[sprite_bundle("player.png")]
    #[bundle]
    pub sprite_bundle: SpriteBundle,
    pub player: Player,
    #[worldly]
    pub worldly: Worldly,
    // The whole EntityInstance can be stored directly as an EntityInstance component
    #[from_entity_instance]
    entity_instance: EntityInstance,

    velocity: Velocity,
}
