use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use crate::physics::{ColliderBundle, GroundDetection};
use crate::{config::{
    PLAYER_SETTINGS,
    GAME_RULES,
}, game_rules::CoinState};
use crate::entity_state::*;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, movement);
    }
}



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
    pub movement_state: EntityMovementState,
}

pub fn movement(
    input: Res<Input<KeyCode>>,
    // time: Res<Time>,
    mut q_velocity: Query<(
        &mut Velocity, 
        &GroundDetection, 
        &mut EntityMovementState, 
    ), With<Player>>,
    q_coins: Query<&CoinState>,
) {
    let coins = match q_coins.get_single() {
        Ok(coins) => coins,
        Err(_) => return,
    };

    let loot_factor = coins.coins_total as f32 / GAME_RULES.starting_coins as f32;
    let speed_factor = 1.0 + ((PLAYER_SETTINGS.base_loot_factor - 1.0) * loot_factor);
    for (mut velocity, ground_detection, mut movement_state) in &mut q_velocity {
        let right = if input.pressed(KeyCode::D) { 1.0 } else { 0.0 };
        let left = if input.pressed(KeyCode::A) { 1.0 } else { 0.0 };

        velocity.linvel.x = (right - left) * (PLAYER_SETTINGS.x_velocity * speed_factor);

        if input.just_pressed(KeyCode::Space) && ground_detection.on_ground {
            velocity.linvel.y = PLAYER_SETTINGS.jump_velocity * speed_factor;
            movement_state.ground_state = EntityGroundState::Jumping;
        }

        if velocity.linvel.x > 0.01 {
            movement_state.direction = EntityDirection::Right;
            movement_state.movement = EntityMovement::Walking;
        } 
        else if velocity.linvel.x < -0.01 {
            movement_state.direction = EntityDirection::Left;
            movement_state.movement = EntityMovement::Walking;
        } 
        else {
            movement_state.movement = EntityMovement::Still;
        }

        if ground_detection.on_ground {
            movement_state.ground_state = EntityGroundState::Grounded;
            
        } 
        else if velocity.linvel.y < -0.2 {
            movement_state.ground_state = EntityGroundState::Airbourne;
        }
    }
}