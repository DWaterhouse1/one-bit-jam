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
        app.add_systems(Startup,setup_camera_manager);
        app.add_systems(PostStartup, init_camera);
        app.add_systems(Update, update_camera);
    }
}

#[derive(Component)]
pub struct CameraManager {
    time: Stopwatch,
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
        delta_transform.x += 32.0;
    };
    if input.pressed(KeyCode::Left) {
        delta_transform.x -= 32.0;
    };
    if input.pressed(KeyCode::Up) {
        delta_transform.y += 32.0;
    };
    if input.pressed(KeyCode::Down) {
        delta_transform.y -= 32.0;
    };

    // pan camera to target location
    let delta_target = camera_transform.translation - 
                            (player_transform.translation + 
                             delta_transform);
    let delta_distance = camera_transform.translation - 
                               player_transform.translation;
    if delta_transform != Vec3::new(0.0, 0.0, 0.0)
    {
        camera_transform.translation -= (delta_target) * 0.025;

        // stop camera tracking for timout period
        camera_manager.time.reset()
    }

    // start tracking player after elapsed time
    if camera_manager.time.elapsed_secs() > 0.5 {
        camera_transform.translation -= delta_distance * 0.05;
    };
}

pub fn init_camera(
    mut q_camera_manager: Query<&mut CameraManager>,
    mut q_camera_transform: Query<&mut Transform, (With<Camera>, Without<Player>)>,
    q_player_transform: Query<&Transform, (With<Player>, Without<Camera>)>,
)
{
    println!("init_camera");

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
    camera_manager.time.set_elapsed(Duration::new(100, 0));
    camera_transform.translation = player_transform.translation;
}