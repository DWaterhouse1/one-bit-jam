use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use crate::physics::ColliderBundle;
use crate::{
    config::{
        PLAYER_SETTINGS,
        GAME_RULES,
    }, 
    game_rules::CoinState,
};
use crate::entity_state::*;
use std::collections::HashSet;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            spawn_ground_sensor.before(ground_detection),
            ground_detection.before(update_on_ground),
            update_on_ground.before(movement),
            movement,
        ));
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
    ), With<Player>>,
    q_coins: Query<&CoinState>,
) {
    let coins = match q_coins.get_single() {
        Ok(coins) => coins,
        Err(_) => return,
    };

    let loot_factor = coins.coins_total as f32 / GAME_RULES.starting_coins as f32;
    let speed_factor = 1.0 + ((PLAYER_SETTINGS.base_loot_factor - 1.0) * loot_factor);
    for (mut velocity, ground_detection) in &mut q_velocity {
        let right = if input.pressed(KeyCode::D) { 1.0 } else { 0.0 };
        let left = if input.pressed(KeyCode::A) { 1.0 } else { 0.0 };

        velocity.linvel.x = (right - left) * (PLAYER_SETTINGS.x_velocity * speed_factor);

        if input.just_pressed(KeyCode::Space) && ground_detection.on_ground {
            velocity.linvel.y = PLAYER_SETTINGS.jump_velocity * speed_factor;
        }
    }
}

pub fn update_movement_state(
    mut q_velocity: Query<(
        &Velocity,
        &GroundDetection,
        &mut EntityMovementState,
    ), With<Player>>,
) {
    for (velocity, ground_detection, mut movement_state) in &mut q_velocity {

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
        else {
            movement_state.ground_state = EntityGroundState::Airbourne;
        }
    }
}

pub fn spawn_ground_sensor(
    mut commands: Commands,
    detect_ground_for: Query<(Entity, &Collider), Added<GroundDetection>>,
) {
    for (entity, shape) in &detect_ground_for {
        if let Some(capsule) = shape.as_capsule() {
            let detector_shape = Collider::cuboid(capsule.radius() / 2.0, 2.);

            let sensor_translation = Vec3::new(0., -capsule.height(), 0.);

            commands.entity(entity).with_children(|builder| {
                builder
                    .spawn_empty()
                    .insert(ActiveEvents::COLLISION_EVENTS)
                    .insert(detector_shape)
                    .insert(Sensor)
                    .insert(Transform::from_translation(sensor_translation))
                    .insert(GlobalTransform::default())
                    .insert(GroundSensor {
                        ground_detection_entity: entity,
                        intersecting_ground_entities: HashSet::new(),
                    });
            });
        }
    }
}


#[derive(Clone, Default, Component)]
pub struct GroundDetection {
    pub on_ground: bool,
}

#[derive(Component)]
pub struct GroundSensor {
    pub ground_detection_entity: Entity,
    pub intersecting_ground_entities: HashSet<Entity>,
}


pub fn update_on_ground(
    mut ground_detectors: Query<&mut GroundDetection>,
    ground_sensors: Query<&GroundSensor, Changed<GroundSensor>>,
) {
    for sensor in &ground_sensors {
        if let Ok(mut ground_detection) = ground_detectors.get_mut(sensor.ground_detection_entity) {
            ground_detection.on_ground = !sensor.intersecting_ground_entities.is_empty();
        }
    }
}

pub fn ground_detection(
    mut ground_sensors: Query<&mut GroundSensor>,
    mut collisions: EventReader<CollisionEvent>,
    collidables: Query<With<Collider>, Without<Sensor>>,
) {
    for collision_event in collisions.iter() {
        match collision_event {
            CollisionEvent::Started(e1, e2, _) => {
                if collidables.contains(*e1) {
                    if let Ok(mut sensor) = ground_sensors.get_mut(*e2) {
                        sensor.intersecting_ground_entities.insert(*e1);
                    }
                } else if collidables.contains(*e2) {
                    if let Ok(mut sensor) = ground_sensors.get_mut(*e1) {
                        sensor.intersecting_ground_entities.insert(*e2);
                    }
                }
            }
            CollisionEvent::Stopped(e1, e2, _) => {
                if collidables.contains(*e1) {
                    if let Ok(mut sensor) = ground_sensors.get_mut(*e2) {
                        sensor.intersecting_ground_entities.remove(e1);
                    }
                } else if collidables.contains(*e2) {
                    if let Ok(mut sensor) = ground_sensors.get_mut(*e1) {
                        sensor.intersecting_ground_entities.remove(e2);
                    }
                }
            }
        }
    }
}