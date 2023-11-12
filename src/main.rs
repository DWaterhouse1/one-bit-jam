use bevy::prelude::*;
use bevy::window::PresentMode;

use one_bit_jam::camera::CameraManagerPlugin;
use one_bit_jam::levels::update_level_selection;
use one_bit_jam::config::{
    WINDOW_SETTINGS,
    LDTK_PLAYER_LEVEL,
    LDTK_INT_CELL_VALUES,
};
use one_bit_jam::physics::PhysicsPluginGroup;
use one_bit_jam::player::PlayerBundle;
use one_bit_jam::physics::WallBundle;
use one_bit_jam::game_rules::GameRulesPlugin;

use bevy_ecs_ldtk::prelude::*;

fn main() {
    // When building for WASM, print panics to the browser console
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    App::new()
        .insert_resource(Msaa::Off)
        .insert_resource(ClearColor(Color::rgb(52.0, 46.0, 44.0)))
        .insert_resource(LdtkSettings {
            level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                load_level_neighbors: true,
            },
            set_clear_color: SetClearColor::FromLevelBackground,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: WINDOW_SETTINGS.title.into(),
                resolution: (WINDOW_SETTINGS.width, WINDOW_SETTINGS.height).into(),
                present_mode: PresentMode::AutoVsync,
                fit_canvas_to_parent: true,
                prevent_default_event_handling: false,
                canvas: Some("#bevy".to_owned()),
                ..default()
            }),
            ..default()
        })
        .set(ImagePlugin::default_nearest()))
        .add_plugins(LdtkPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, update_level_selection)
        .insert_resource(LevelSelection::Uid(LDTK_PLAYER_LEVEL))
        .register_ldtk_entity::<PlayerBundle>("Player")
        .register_ldtk_int_cell::<WallBundle>(LDTK_INT_CELL_VALUES.walls)
        .add_plugins(PhysicsPluginGroup)
        .add_plugins(CameraManagerPlugin)
        .add_plugins(GameRulesPlugin)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("test.ldtk"),
        ..Default::default()
    });
}
