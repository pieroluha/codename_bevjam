// use crate::GameState;
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
    pub color: Color,
    #[texture(1)]
    #[sampler(2)]
    pub base: Handle<Image>,
}

impl Material2d for CreatureMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/creature_shader.wgsl".into()
    }
}

#[derive(Component)]
pub struct CreatureMaterialHandle(pub Handle<CreatureMaterial>);
