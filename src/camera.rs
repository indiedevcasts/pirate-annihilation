use crate::components::player::Player;
use bevy::prelude::*;
use bevy_ecs_ldtk::{LdtkLevel, LevelSelection};
use dolly::prelude::*;

const MOVEMENT_SPEED: f32 = 10.;
const HORIZONTAL_STEP: f32 = 0.2;
const VERTICAL_STEP: f32 = 1.;

#[derive(Component, Debug)]
pub struct CameraController {
    pub rig: CameraRig,
}

pub fn fit_inside_current_level(
    mut camera_query: Query<
        (
            &mut bevy::render::camera::OrthographicProjection,
            &mut Transform,
        ),
        Without<Player>,
    >,
    player_query: Query<&Transform, With<Player>>,
    level_query: Query<
        (&Transform, &Handle<LdtkLevel>),
        (Without<OrthographicProjection>, Without<Player>),
    >,
    level_selection: Res<LevelSelection>,
    ldtk_levels: Res<Assets<LdtkLevel>>,
    time: Res<Time>,
) {
    if let Ok(Transform {
        translation: player_translation,
        ..
    }) = player_query.get_single()
    {
        let player_translation = *player_translation;
        let (orthographic_projection, mut camera_transform) = camera_query.single_mut();
        for (_, level_handle) in level_query.iter() {
            if let Some(ldtk_level) = ldtk_levels.get(level_handle) {
                // The boundaries used to clamp the camera in the level
                let (x_boundary_distance, y_boundary_distance) = (
                    orthographic_projection.right * orthographic_projection.scale,
                    orthographic_projection.top * orthographic_projection.scale,
                );
                let level = &ldtk_level.level;
                if level_selection.is_match(&0, level) {
                    let x = (player_translation.x).clamp(
                        x_boundary_distance,
                        level.px_wid as f32 - x_boundary_distance,
                    );
                    let y = (player_translation.y).clamp(
                        y_boundary_distance,
                        level.px_hei as f32 - y_boundary_distance,
                    );

                    let direction = Vec3::new(x, y, camera_transform.translation.z);

                    let smooth_damp = magnet::core::vec3::smooth_damp(
                        camera_transform.translation,
                        direction,
                        Vec3::ZERO,
                        0.2,
                        f32::INFINITY,
                        time.delta_seconds(),
                    );
                    camera_transform.translation = smooth_damp;
                }
            }
        }
    }
}

pub fn move_free_camera(
    mut camera: Query<(&mut Transform, With<Camera>)>,
    mut camera_controller: Query<&mut CameraController>,
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
) {
    let (mut camera_transform, _) = match camera.get_single_mut() {
        Ok(transform) => transform,
        _ => return (),
    };

    let mut camera_controller = match camera_controller.get_single_mut() {
        Ok(controller) => controller,
        _ => return (),
    };

    let mut horizontal_move = 0.;
    let mut vertical_move = 0.;

    if input.pressed(KeyCode::Z) {
        vertical_move = -VERTICAL_STEP;
    }

    if input.pressed(KeyCode::S) {
        vertical_move = VERTICAL_STEP;
    }

    if input.pressed(KeyCode::D) {
        horizontal_move = HORIZONTAL_STEP;
    }

    if input.pressed(KeyCode::Q) {
        horizontal_move = -HORIZONTAL_STEP;
    }

    let move_vec = camera_controller.rig.final_transform.rotation
        * Vec3::new(horizontal_move, 0., vertical_move).clamp_length_max(1.0)
        * MOVEMENT_SPEED;

    debug!("camera_transform : {:?}", camera_transform);

    camera_controller
        .rig
        .driver_mut::<Position>()
        .translate(move_vec * time.delta_seconds() * 10.0);
    let rig_transform = camera_controller.rig.update(time.delta_seconds());

    camera_transform.translation = rig_transform.position;
    camera_transform.rotation = rig_transform.rotation;
}
