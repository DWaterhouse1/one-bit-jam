use std::time::Duration;
use bevy::prelude::*;

pub struct SpriteManagerPlugin;
impl Plugin for SpriteManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, animate_sprites);
    }
}

#[derive(Default, Component)]
pub struct Animation {
    pub animation_timer: AnimationTimer,
    pub animation_meta: AnimationMetadata,
    pub animation_keys: AnimationKeys,
}

#[derive(Default, Component)]
pub struct AnimationTimer(pub Timer);

#[derive(Default, Component)]
pub struct AnimationMetadata {
    pub current: usize,
    pub looping: bool,
}

#[derive(Default, Component)]
pub struct AnimationKeys {
    pub keys: Vec<(usize, u32)>,
}

pub fn animate_sprites(
    time: Res<Time>,
    mut q_animation: Query<(
        &mut TextureAtlasSprite, 
        &mut AnimationTimer,
        &mut AnimationMetadata,
        & AnimationKeys,
    )>,
) {
    for (mut tas, 
        mut timer, 
        mut anim_metadata, 
        anim_keys
    ) in &mut q_animation {
        timer.0.tick(time.delta());
        if timer.0.just_finished() {
            anim_metadata.current += 1;

            if anim_metadata.current <= anim_keys.keys.len() {
                tas.index = anim_keys.keys[anim_metadata.current].0;

            } else if anim_metadata.looping {
                anim_metadata.current = 0;
                tas.index = anim_metadata.current;
            } else {
                return
            }

            println!("setting timer to {:?} ns", anim_keys.keys[anim_metadata.current].1);
            timer.0.set_duration(Duration::new(0, anim_keys.keys[anim_metadata.current].1));
        }
    }
}

#[derive(Default, Resource)]
pub struct AnimationResource {

}
