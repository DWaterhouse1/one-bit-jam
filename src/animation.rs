use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

// use benimator::*;

pub struct SpriteManagerPlugin;
impl Plugin for SpriteManagerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, load_sprite_sheets);
    }
}

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct AnimationBundle {
    pub sprite_sheet_bundle: SpriteSheetBundle,
}

#[derive(Clone, Default, Component, LdtkEntity)]
pub struct AnimationInfo {
    pub sprite_sheet_path: String,
}

impl From<&EntityInstance> for AnimationInfo {
    fn from(entity_instance: &EntityInstance) -> AnimationInfo {
        println!("fomr entity instance for anim info");
        match entity_instance.identifier.as_ref() {
            "Player" => AnimationInfo {
                sprite_sheet_path: "atlas/Lil_Gobbo.png".to_string(),
            },
            _ => AnimationInfo::default(),
        }
    }
}

fn load_sprite_sheets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    q_anim: Query<(Entity, &AnimationInfo), Added<AnimationInfo>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // should always have a result due to Added, panic otherwise
    let (entity, anim) = match q_anim.get_single() {
        Ok(val) => val,
        Err(_) => {
            return;
        },
    };

    let texture_handle = asset_server.load(anim.sprite_sheet_path.to_string());
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 4, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.entity(entity).insert(SpriteSheetBundle {
        texture_atlas: texture_atlas_handle,
        ..Default::default()
    });
}
