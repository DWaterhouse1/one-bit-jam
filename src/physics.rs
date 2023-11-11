use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_rapier2d::dynamics::RigidBody;
// use bevy_ecs_ldtk::prelude::*;

use bevy::app::PluginGroupBuilder;

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
            .add_systems(Startup, setup_physics);
    }
}

fn setup_physics(mut commands: Commands) {
    /* Create the ground. */
    commands
        .spawn(Collider::cuboid(500.0, 50.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -100.0, 0.0)));

    /* Create the bouncing ball. */
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(50.0))
        .insert(Restitution::coefficient(0.7))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 400.0, 0.0)));
}