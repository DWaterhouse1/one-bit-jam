use bevy::prelude::Component;

#[derive(Default)]
pub enum EntityDirection {
    Left,
    #[default]
    Right,
    Up,
    Down,
}

#[derive(Default)]
pub enum EntityMovement {
    #[default]
    Still,
    Walking,
    Dashing,
}

#[derive(Default)]
pub enum EntityGroundState {
    #[default]
    Grounded,
    Jumping,
    Airbourne,
}

#[derive(Component, Default)]
pub struct EntityMovementState {
    pub direction: EntityDirection,
    pub ground_state: EntityGroundState,
    pub movement: EntityMovement,
}
