use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

pub struct SoundsPlugin;
impl Plugin for SoundsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(AudioPlugin)
            .add_startup_system(play_bopper);
    }
}

fn play_bopper(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    audio
        .play(asset_server.load("sound/thanks_bosca_ceoil.wav"))
        .looped();
}
