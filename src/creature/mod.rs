use bevy::{
    app::PluginGroupBuilder,
    prelude::*,
};

mod creature_anim;
mod creature_assets;
mod creature_bundle;
// mod creature_shader;

pub const CRE_SIZE: f32 = 16.0;

pub mod stats {
    pub use super::creature_bundle::{
        Health, MagicalDamage, Mana, ManaRegen, PhysicalDamage, Regen, Speed,
    };
}

pub use creature_assets::CreatureAssets;
pub use creature_bundle::{CreatureBundle, EnemyBundle};
// pub use creature_shader::{CreatureMaterial, CreatureMaterialHandle, CreatureMesh};

pub struct CreaturePlugins;
impl PluginGroup for CreaturePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(creature_assets::CreatureAssetsPlugin)
            .add(creature_anim::CreatureAnimPlugin)
    }
}

// #[derive(PartialEq, Eq)]
// pub enum CreatureFaction {
//     Player,
//     Enemy,
// }
