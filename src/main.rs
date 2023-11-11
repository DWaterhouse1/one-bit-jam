use bevy::prelude::*;
use bevy::window::PresentMode;

use one_bit_jam::config::WINDOW_SETTINGS;
use one_bit_jam::physics::PhysicsPluginGroup;
use one_bit_jam::player::PlayerBundle;
use one_bit_jam::physics::GroundBundle;
use bevy_ecs_ldtk::prelude::*;

fn main() {
    // When building for WASM, print panics to the browser console
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();
    
    App::new()
        .insert_resource(Msaa::Off)
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
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
        }))
        .add_plugins(LdtkPlugin)
        .add_systems(Startup, setup)
        .insert_resource(LevelSelection::Index(0))
        .register_ldtk_entity::<TestEntityBundle>("Entity")
        .register_ldtk_entity::<TestPlayerEntityBundle>("TestPlayer")
        .register_ldtk_entity::<PlayerBundle>("Player")
        .register_ldtk_int_cell::<GroundBundle>(1)
        .add_plugins(PhysicsPluginGroup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("test.ldtk"),
        ..Default::default()
    });
}

#[derive(Bundle, LdtkEntity)]
pub struct TestEntityBundle {
    #[sprite_bundle("atlas/test_ent.png")]
    sprite_bundle: SpriteBundle,
}

#[derive(Bundle, LdtkEntity)]
pub struct TestPlayerEntityBundle {
    #[sprite_sheet_bundle("atlas/test_player_ent.png", 16.0, 16.0, 7, 1, 0.0, 0.0, 0)]
    sprite_bundle: SpriteSheetBundle,
}
