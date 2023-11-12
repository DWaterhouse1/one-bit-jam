use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::physics::{ColliderBundle, GroundDetection};

pub struct PlayerPlugin;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Player;

#[derive(Default, Bundle, LdtkEntity)]
pub struct PlayerBundle {
    pub player: Player,
    #[from_entity_instance]
    pub collider_bundle: ColliderBundle,
    #[worldly]
    pub worldly: Worldly,
    #[sprite_sheet_bundle("atlas/Lil_Gobbo.png", 32.0, 32.0, 4, 1, 0.0, 0.0, 3)]
    sprite_bundle: SpriteSheetBundle,
    pub ground_detection: GroundDetection,
}