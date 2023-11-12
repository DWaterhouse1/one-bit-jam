use std::time::Duration;
use crate::player::Player;
use bevy::prelude::*;
use bevy::time::Stopwatch;

use crate::config::{
    CAMERA_SETTINGS,
    CONSTANTS,
};

pub struct CameraManagerPlugin;
fn setup_camera_manager(mut commands: Commands) {
    commands.spawn(CameraManager {
        time: Stopwatch::new(),
    });
}

impl Plugin for CameraManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup,setup_camera_manager);
        app.add_systems(Update, (
            update_camera,
            init_camera,
            check_new_player,
        ));
        app.add_event::<InitCameraEvent>();
    }
}

#[derive(Component)]
pub struct CameraManager {
    time: Stopwatch,
}

pub fn check_new_player(
    q_player: Query<&Player, Added<Player>>,
    mut ev_init: EventWriter<InitCameraEvent>,
) {
    let _ = match q_player.get_single() {
        Ok(_) => {},
        Err(_) => return,
    };
    ev_init.send(InitCameraEvent)
}

// TODO add magic numbers to config?
// might want to be able to change camera speed based
// add rates as to CameraManager?
pub fn update_camera(
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
    mut q_camera_manager: Query<&mut CameraManager>,
    mut q_camera_transform: Query<&mut Transform, (With<Camera>, Without<Player>)>,
    q_player_transform: Query<&Transform, (With<Player>, Without<Camera>)>,
) {
    let mut camera_manager = match q_camera_manager.get_single_mut() {
        Ok(camera_manager) => camera_manager,
        Err(_) => return,
    };
    let mut camera_transform = match q_camera_transform.get_single_mut() {
        Ok(camera_transform) => camera_transform,
        Err(_) => return,
    };
    let player_transform = match q_player_transform.get_single() {
        Ok(player_transform) => player_transform,
        Err(_) => return,
    };

    // progress timer
    camera_manager.time.tick(time.delta());

    // add camera delta based on user input
    let mut delta_transform = Vec3::new(0.0, 0.0, 0.0);
    if input.pressed(KeyCode::Right) {
        delta_transform.x += CAMERA_SETTINGS.look_distance;
    };
    if input.pressed(KeyCode::Left) {
        delta_transform.x -= CAMERA_SETTINGS.look_distance;
    };
    if input.pressed(KeyCode::Up) {
        delta_transform.y += CAMERA_SETTINGS.look_distance;
    };
    if input.pressed(KeyCode::Down) {
        delta_transform.y -= CAMERA_SETTINGS.look_distance;
    };

    let target_rate = 
        CAMERA_SETTINGS.target_rate * (
            time.delta().as_nanos() as f32 /
            CONSTANTS.nano_per_mili as f32
        )
    ;

    // pan camera to target location
    let delta_target = camera_transform.translation - 
                            (player_transform.translation + 
                             delta_transform);
    let delta_distance = camera_transform.translation - 
                               player_transform.translation;
    if delta_transform != Vec3::new(0.0, 0.0, 0.0)
    {
        camera_transform.translation -= (delta_target) * target_rate;

        // stop camera tracking for timout period
        camera_manager.time.reset()
    }

    // start tracking player after elapsed time
    if camera_manager.time.elapsed_secs() > CAMERA_SETTINGS.snap_time {
        camera_transform.translation -= delta_distance * target_rate;
    };
}

#[derive (Event)]
pub struct InitCameraEvent;

pub fn init_camera(
    mut q_camera_manager: Query<&mut CameraManager>,
    mut q_camera_transform: Query<&mut Transform, (With<Camera>, Without<Player>)>,
    q_player_transform: Query<&Transform, (With<Player>, Without<Camera>)>,
    mut ev_init: EventReader<InitCameraEvent>,
) {
    for _ in ev_init.iter() {

        let mut camera_manager = match q_camera_manager.get_single_mut() {
            Ok(camera_manager) => camera_manager,
            Err(_) => return,
        };
    
        let mut camera_transform = match q_camera_transform.get_single_mut() {
            Ok(camera_transform) => camera_transform,
            Err(_) => return,
        };
    
        let player_transform = match q_player_transform.get_single() {
            Ok(player_transform) => player_transform,
            Err(_) => return,
        };
    
        // elapse timer to start player tracking
        // start the timer at any point after the snap time
        camera_manager.time.set_elapsed(Duration::new((CAMERA_SETTINGS.snap_time as u64) + 1, 0));
        camera_transform.translation = player_transform.translation;

        camera_transform.scale = Vec3::new(0.35, 0.35, 0.35);
    }

    ev_init.clear();
}