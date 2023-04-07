// use crate::GameState;
use super::CreatureFaction;
use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{Material2d, Material2dPlugin},
};

pub struct CreatureShaderPlugin;
impl Plugin for CreatureShaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(Material2dPlugin::<CreatureMaterial>::default());
    }
}

#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "b19f78b6-c9e5-43cf-9600-52b5a5ef09ce"]
pub struct CreatureMaterial {
    #[uniform(0)]
    colors: [Color; 5],
    #[uniform(0)]
    idx: i32,
    #[texture(1)]
    #[sampler(2)]
    base: Handle<Image>,
}

impl Material2d for CreatureMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/creature_shader.wgsl".into()
    }
}

impl CreatureMaterial {
    pub fn new(cre_type: CreatureFaction, img_handle: Handle<Image>) -> Self {
        let player_colors = [
            Color::hex("#9a0e0e").unwrap(),
            Color::hex("#a62219").unwrap(),
            Color::hex("#b33c24").unwrap(),
            Color::hex("#bd4926").unwrap(),
            Color::hex("#cc6633").unwrap(),
        ];

        let enemy_colors = [
            Color::hex("#303840").unwrap(),
            Color::hex("#3d424d").unwrap(),
            Color::hex("#4b4d57").unwrap(),
            Color::hex("#5d5d66").unwrap(),
            Color::hex("#6e6d73").unwrap(),
        ];

        let colors = if cre_type == CreatureFaction::Player {
            player_colors
        } else {
            enemy_colors
        };

        Self {
            colors,
            idx: 0,
            base: img_handle,
        }
    }

    // TODO: Unload old image?
    pub fn update_handle(&mut self, img_handle: Handle<Image>) {
        self.base = img_handle;
    }

    pub fn update_idx(&mut self, idx: i32) {
        self.idx = idx;
    }
}

#[derive(Component)]
pub struct CreatureMaterialHandle(pub Handle<CreatureMaterial>);

// TODO: Events: Death shader, random color shader
