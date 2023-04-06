use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use super::creature_shader;

#[derive(Component)]
pub struct Creature;

#[derive(Default, Component)]
pub struct Health(pub f32);
#[derive(Default, Component)]
pub struct Energy(pub f32);
#[derive(Default, Component)]
pub struct Speed(pub f32);

#[derive(Bundle)]
pub struct CreatureBundle {
    pub marker: Creature,
    pub health: Health,
    pub energy: Energy,
    pub speed: Speed,
    pub mat_handle: creature_shader::CreatureMaterialHandle,
    #[bundle]
    pub mesh2d_bundle: MaterialMesh2dBundle<creature_shader::CreatureMaterial>,
}
impl CreatureBundle {
    pub fn new(
        h: f32,
        e: f32,
        s: f32,
        mat_handle: Handle<creature_shader::CreatureMaterial>,
        mesh2d_bundle: MaterialMesh2dBundle<creature_shader::CreatureMaterial>,
    ) -> Self {
        Self {
            marker: Creature,
            health: Health(h),
            energy: Energy(e),
            speed: Speed(s),
            mat_handle: creature_shader::CreatureMaterialHandle(mat_handle),
            mesh2d_bundle,
        }
    }
}
