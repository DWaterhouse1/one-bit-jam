use bevy::prelude::*;
use bevy::time::Stopwatch;
use crate::player::Player;


pub struct CameraManagerPlugin;
fn setup_camera_manager(mut commands: Commands)
{
    commands.spawn(CameraManager{
        time : Stopwatch::new()
    });
}

impl Plugin for CameraManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera_manager);
    }
}

#[derive(Component)]
struct CameraManager {
    pub 
    time: Stopwatch,
}

pub fn update_camera(
    // input: Res<Input<KeyCode>>,
    mut q_camera: Query<&mut Transform, (With<Camera>, Without<Player>)>,
    q_player: Query<&Transform,  (With<Player>, Without<Camera>)>,
) {
    let mut camera_transform = match q_camera.get_single_mut() {
        Ok(camera_transform) => camera_transform,
        Err(_) => return,
    };

    let player_transform = match q_player.get_single() {
        Ok(player_transform) => player_transform,
        Err(_) => return,
    };

    // if input.pressed(KeyCode::Right) {
    //     camera_transform.translation += Vec3::new(2.0, 0.0, 0.0);
    // };
    // if input.pressed(KeyCode::Left) {
    //     camera_transform.translation += Vec3::new(-2.0, 0.0, 0.0);
    // };
    // if input.pressed(KeyCode::Up) {
    //     camera_transform.translation += Vec3::new(0.0, 2.0, 0.0);
    // };
    // if input.pressed(KeyCode::Down) {
    //     camera_transform.translation += Vec3::new(0.0, -2.0, 0.0);
    // };

    camera_transform.translation = player_transform.translation;
}