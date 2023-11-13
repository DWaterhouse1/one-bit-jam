use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use crate::animation::Animation;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct TestEntity;

#[derive(Default, Bundle, LdtkEntity)]
pub struct TestEntityBundle {
    pub test_entity: TestEntity,
    #[sprite_sheet_bundle("atlas/Lil_Gobbo.png", 32.0, 32.0, 4, 1, 0.0, 0.0, 3)]
    sprite_bundle: SpriteSheetBundle,
    pub animation: Animation,
}
