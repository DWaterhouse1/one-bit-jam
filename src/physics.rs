use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_rapier2d::dynamics::RigidBody;
use bevy_ecs_ldtk::prelude::*;
use bevy::app::PluginGroupBuilder;

use crate::config::{
    INT_GRID_SETTINGS,
    PHYSICS_SETTINGS,
};

use crate::player::Player;

pub struct PhysicsPluginGroup;

struct PhysicsPlugin;

impl PluginGroup for PhysicsPluginGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
            .add(RapierDebugRenderPlugin::default())
            .add(PhysicsPlugin)
    }
}

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_physics)
            .add_systems(Update, movement)
            .insert_resource(RapierConfiguration {
                gravity: Vec2::new(0.0, PHYSICS_SETTINGS.gravity),
                ..Default::default()
            });
    }
}

#[derive(Clone, Default, Bundle, LdtkIntCell)]
pub struct ColliderBundle {
    pub collider: Collider,
    pub rigid_body: RigidBody,
    pub velocity: Velocity,
    pub rotation_constraints: LockedAxes,
    pub friction: Friction,
    pub restitution: Restitution,
    pub mass_properties: ColliderMassProperties,
    pub force: ExternalForce,
}

#[derive(Clone, Default, Bundle, LdtkIntCell)]
pub struct GroundBundle {
    pub collider: Collider,
}


impl From<EntityInstance> for ColliderBundle {
    fn from(
        entity_instance: EntityInstance,
    ) -> ColliderBundle {
        match entity_instance.identifier.as_ref() {
            "Player" => ColliderBundle {
                collider: Collider::capsule_y(12.0, 12.0),
                rigid_body: RigidBody::KinematicPositionBased,
                rotation_constraints: LockedAxes::ROTATION_LOCKED,
                ..Default::default()
            },
            _ => ColliderBundle::default(),
        }
    }
}

impl From<IntGridCell> for GroundBundle {
    fn from(int_grid_cell: IntGridCell) -> GroundBundle {
        if int_grid_cell.value == INT_GRID_SETTINGS.ground {
            GroundBundle {
                collider: Collider::cuboid(200.0, 200.0),
            }
        } else {
            GroundBundle::default()
        }
    }
}

pub fn movement(
    input: Res<Input<KeyCode>>,
    mut q_velocity: Query<&mut Velocity, With<Player>>
) {
    for mut velocity in &mut q_velocity {
        let right = if input.pressed(KeyCode::D) { 1. } else { 0. };
        let left = if input.pressed(KeyCode::A) { 1. } else { 0. };

        velocity.linvel.x = (right - left) * 200.0;

        if input.just_pressed(KeyCode::Space) {
            velocity.linvel.y = 500.0;
        }
    }
}


fn setup_physics(mut commands: Commands) {
    /* Create the ground. */
    commands
        .spawn(Collider::cuboid(500.0, 50.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -100.0, 0.0)));
}