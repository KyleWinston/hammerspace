use bevy::prelude::*;

use bevy_third_person_camera::{
    camera::{CameraGamepadSettings, Offset, Zoom},
};

use crate::{controller::resources::Controller, HammerState};

use super::{
    components::{PanOrbitCamera, Setpiece},
    PANNING_KEYS,
};

pub fn spawn_cameras(mut commands: Commands) {
    let gamepad = Gamepad::new(0);
    commands.insert_resource(Controller(gamepad));
    commands.spawn((Camera3dBundle {
        camera: Camera {
            is_active: false,
            ..default()
        },
        ..default()
    },));

    let radius = 20.0;
    let orbit: f32 = 0.0;
    commands.spawn((
        Camera3dBundle {
            camera: Camera {
                is_active: true,
                ..default()
            },
            transform: Transform::from_xyz(orbit.cos() * radius, 10.0, orbit.sin() * radius)
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        PanOrbitCamera {
            radius,
            orbit,
            ..default()
        },
    ));
}

pub fn switch_to_editor_view(
    mut edit_cam: Query<&mut Camera, With<PanOrbitCamera>>,
    mut game_cam: Query<&mut Camera, Without<PanOrbitCamera>>,
) {
    if let Ok(mut edit_cam) = edit_cam.get_single_mut() {
        edit_cam.is_active = true;
    }
    if let Ok(mut game_cam) = game_cam.get_single_mut() {
        game_cam.is_active = false;
    }
}

pub fn switch_to_game_view(
    mut edit_cam: Query<&mut Camera, With<PanOrbitCamera>>,
    mut game_cam: Query<&mut Camera, Without<PanOrbitCamera>>,
) {
    if let Ok(mut edit_cam) = edit_cam.get_single_mut() {
        edit_cam.is_active = false;
    }
    if let Ok(mut game_cam) = game_cam.get_single_mut() {
        game_cam.is_active = true;
    }
}

pub fn fly_cam(
    mut edit_cam: Query<(&mut Transform, &GlobalTransform, &mut PanOrbitCamera)>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if let Ok((mut transform, global, mut pan_orbit)) = edit_cam.get_single_mut() {
        if keys.any_pressed(PANNING_KEYS) {
            pan_orbit.panning = true;
        } else {
            pan_orbit.panning = false;
        }
        let mut direction = Vec3::ZERO;
        if keys.pressed(PANNING_KEYS[0]) {
            direction += global.left();
        }
        if keys.pressed(PANNING_KEYS[1]) {
            direction += global.right();
        }
        if keys.pressed(PANNING_KEYS[2]) {
            direction += global.forward();
        }
        if keys.pressed(PANNING_KEYS[3]) {
            direction += global.back();
        }
        if keys.pressed(PANNING_KEYS[4]) {
            direction += global.up();
        }
        if keys.pressed(PANNING_KEYS[5]) {
            direction += global.down();
        }
        direction = direction.normalize_or_zero();
        transform.translation += direction;
        pan_orbit.focus += direction;
    }
}

pub fn orbit_cam(
    mut edit_cam: Query<(&mut Transform, &mut PanOrbitCamera)>,
    keys: Res<ButtonInput<KeyCode>>,
    alkyd_setpiece: Query<&Transform, (With<Setpiece>, Without<PanOrbitCamera>)>,
    state: Res<State<HammerState>>,
) {
    if let Ok((mut transform, mut pan_orbit)) = edit_cam.get_single_mut() {
        if *state == HammerState::Showcase {
            if let Ok(set_transform) = alkyd_setpiece.get_single() {
                transform.look_at(set_transform.translation, Vec3::Y);
            }
        }
        if !pan_orbit.panning {
            if keys.any_pressed([KeyCode::ArrowLeft, KeyCode::ArrowRight]) {
                if keys.pressed(KeyCode::ArrowLeft) {
                    pan_orbit.orbit -= 0.17;
                }
                if keys.pressed(KeyCode::ArrowRight) {
                    pan_orbit.orbit += 0.17;
                }

                transform.translation.x =
                    pan_orbit.orbit.cos() * pan_orbit.radius + pan_orbit.focus.x;
                transform.translation.z =
                    pan_orbit.orbit.sin() * pan_orbit.radius + pan_orbit.focus.z;
                transform.look_at(pan_orbit.focus, Vec3::Y);
            }
            if keys.any_pressed([KeyCode::ArrowUp, KeyCode::ArrowDown]) {
                if keys.pressed(KeyCode::ArrowUp) {
                    transform.translation.y += 1.0;
                }
                if keys.pressed(KeyCode::ArrowDown) {
                    transform.translation.y -= 1.0;
                }
                if *state != HammerState::Showcase {
                    transform.look_at(pan_orbit.focus, Vec3::Y);
                }
            }
        }
    }
}
