use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use crate::physics::{ColliderBundle, GroundDetection};
use crate::animation::AnimationInfo;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Player;

#[derive(Default, Bundle, LdtkEntity)]
pub struct PlayerBundle {
    pub player: Player,
    #[from_entity_instance]
    pub collider_bundle: ColliderBundle,
    #[worldly]
    pub worldly: Worldly,
    #[from_entity_instance]
    pub animation_bundle: AnimationInfo,
    pub ground_detection: GroundDetection,
}