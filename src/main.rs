use bevy::prelude::*;
use bevy::window::PresentMode;

mod config;

use config::WINDOW_SETTINGS;
use bevy_ecs_ldtk::prelude::*;

fn main() {
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
        .insert_resource(LevelSelection::Index(1))
        // .register_ldtk_entity::<MyBundle>("MyEntityIdentifier")
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("test.ldtk"),
        ..Default::default()
    });
}

// #[derive(Bundle, LdtkEntity)]
// pub struct MyBundle {
//     a: ComponentA,
//     b: ComponentB,
//     #[sprite_sheet_bundle]
//     sprite_bundle: SpriteSheetBundle,
// }
