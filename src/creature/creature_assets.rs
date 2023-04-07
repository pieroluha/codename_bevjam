use crate::GameState;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

pub struct CreatureAssetsPlugin;
impl Plugin for CreatureAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_collection_to_loading_state::<_, CreatureAssets>(GameState::Startup);
    }
}

#[derive(AssetCollection, Resource)]
pub struct CreatureAssets {
    #[asset(path = "graphics/monalisa.png")]
    pub player: Handle<Image>,
    #[asset(path = "graphics/frog_crab.png")]
    pub attacker: Handle<Image>,
    #[asset(path = "graphics/bible_accurate_angel.png")]
    pub defender: Handle<Image>,
}
