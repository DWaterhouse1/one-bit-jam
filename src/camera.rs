use std::time::Duration;

use crate::player::Player;
use bevy::prelude::*;
use bevy::time::Stopwatch;

pub struct CameraManagerPlugin;
fn setup_camera_manager(mut commands: Commands) {
    commands.spawn(CameraManager {
        time: Stopwatch::new(),
    });
}

impl Plugin for CameraManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera_manager);
    }
}

#[derive(Component)]
pub struct CameraManager {
    time: Stopwatch,
}

// TODO add pan speeds, rates, and distance etc... to config?
pub fn update_camera(
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
    mut q_camera_transform: Query<&mut Transform, (With<Camera>, Without<Player>)>,
    mut q_camera_manager: Query<&mut CameraManager>,
    q_player_transform: Query<&Transform, (With<Player>, Without<Camera>)>,
) {
    let mut camera_transform = match q_camera_transform.get_single_mut() {
        Ok(camera_transform) => camera_transform,
        Err(_) => return,
    };

    let mut camera_manager = match q_camera_manager.get_single_mut() {
        Ok(camera_manager) => camera_manager,
        Err(_) => return,
    };

    let player_transform = match q_player_transform.get_single() {
        Ok(player_transform) => player_transform,
        Err(_) => return,
    };
    camera_manager.time.tick(time.delta());

    let mut delta_transform = Vec3::new(0.0, 0.0, 0.0);
    if input.pressed(KeyCode::Right) {
        delta_transform.x += 1.0;
        camera_manager.time.reset();
    };
    if input.pressed(KeyCode::Left) {
        delta_transform.x -= 1.0;
        camera_manager.time.reset()
    };
    if input.pressed(KeyCode::Up) {
        delta_transform.y += 1.0;
        camera_manager.time.reset()
    };
    if input.pressed(KeyCode::Down) {
        delta_transform.y -= 1.0;
        camera_manager.time.reset()
    };

    let delta_distance = camera_transform.translation - player_transform.translation;
    let new_pos = camera_transform.translation + delta_transform;
    if (new_pos - player_transform.translation).length() < 32.0 {
        camera_transform.translation += delta_transform * (1.0 - (delta_distance.length() / 32.0));
    }

    if camera_manager.time.elapsed_secs() > 1.0 {
        camera_transform.translation -= delta_distance * 0.025;
    };
}
