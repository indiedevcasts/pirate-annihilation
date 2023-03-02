use bevy::{input::mouse::MouseMotion, prelude::*};
use dolly::prelude::*;

const MOVEMENT_SPEED: f32 = 10.;
const HORIZONTAL_STEP: f32 = 1.;
const VERTICAL_STEP: f32 = 1.;

#[derive(Component, Debug)]
pub struct CameraController {
    pub rig: CameraRig,
}

pub fn move_free_camera(
    mut camera: Query<(&mut Transform, With<Camera>)>,
    mut camera_controller: Query<&mut CameraController>,
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
    mouse_input: Res<Input<MouseButton>>,
    mut mouse_motion: EventReader<MouseMotion>,
) {
    let (mut camera_transform, _) = match camera.get_single_mut() {
        Ok(transform) => transform,
        _ => return,
    };

    let mut camera_controller = match camera_controller.get_single_mut() {
        Ok(controller) => controller,
        _ => return,
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

    // camera rotation
    if mouse_input.pressed(MouseButton::Right) {
        for mouse_event in mouse_motion.iter() {
            camera_controller
                .rig
                .driver_mut::<YawPitch>()
                .rotate_yaw_pitch(-0.3 * mouse_event.delta.x, -0.3 * mouse_event.delta.y);
        }
    }

    // camera movement
    camera_controller
        .rig
        .driver_mut::<Position>()
        .translate(move_vec * time.delta_seconds() * 10.0);

    // run all the camera rig drivers and retrieve the final transform
    let rig_transform = camera_controller.rig.update(time.delta_seconds());

    // apply the camera rig info to the bevy camera
    camera_transform.translation = rig_transform.position;
    camera_transform.rotation = rig_transform.rotation;
}
