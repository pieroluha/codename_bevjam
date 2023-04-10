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
    #[asset(texture_atlas(tile_size_x = 16.0, tile_size_y = 16.0, columns = 2, rows = 1))]
    #[asset(path = "graphics/shroom_guy.png")]
    pub player: Handle<TextureAtlas>,
    #[asset(texture_atlas(tile_size_x = 16.0, tile_size_y = 16.0, columns = 2, rows = 1))]
    #[asset(path = "graphics/mastahpiece.png")]
    pub enemy: Handle<TextureAtlas>,
}
