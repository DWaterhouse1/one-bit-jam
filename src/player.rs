use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::physics::ColliderBundle;

pub struct PlayerPlugin;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Player;

#[derive(Default, Bundle, LdtkEntity)]
pub struct PlayerBundle {
    pub player: Player,
    pub collider_bundle: ColliderBundle,
    #[worldly]
    pub worldly: Worldly,
    #[sprite_sheet_bundle("atlas/test_player_ent.png", 16.0, 16.0, 7, 1, 0.0, 0.0, 0)]
    sprite_bundle: SpriteSheetBundle,
}