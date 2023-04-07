use bevy::sprite::MaterialMesh2dBundle;
use bevy::{
    app::PluginGroupBuilder,
    prelude::*,
    // prelude::{Component, PluginGroup},
};

mod creature_assets;
mod creature_bundle;
mod creature_shader;

pub use creature_assets::CreatureAssets;
pub use creature_bundle::CreatureBundle;
pub use creature_shader::{CreatureMaterialHandle, CreatureMaterial, CreatureMesh};

pub struct CreaturePlugins;
impl PluginGroup for CreaturePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(creature_assets::CreatureAssetsPlugin)
            .add(creature_shader::CreatureShaderPlugin)
            .add(TestSpawnPlugin)
    }
}

// pub enum CreatureType {
//     PlayerBase,
//     PlayerAttacker,
//     PlayerDefender,
//     Attacker,
//     Defender,
// }

#[derive(PartialEq, Eq)]
pub enum CreatureFaction {
    Player,
    Enemy,
}

pub struct TestSpawnPlugin;
impl Plugin for TestSpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_creature);
    }
}

fn spawn_creature(
    ass_srv: Res<AssetServer>,
    mut cmds: Commands,
    mut mats: ResMut<Assets<creature_shader::CreatureMaterial>>,
    mut mshs: ResMut<Assets<Mesh>>,
) {
    let mesh = mshs.add(Mesh::from(shape::Quad::default()));
    let img_handle = ass_srv.load("graphics/monalisa.png");

    let material = mats.add(creature_shader::CreatureMaterial::new(
        CreatureFaction::Player,
        img_handle,
    ));

    let mat_handle = material.clone();

    let mesh2d_bundle = MaterialMesh2dBundle {
        mesh: mesh.into(),
        material,
        transform: Transform::default().with_scale(Vec3::new(32.0, 32.0, 1.0)),
        ..default()
    };

    cmds.spawn(creature_bundle::CreatureBundle::new(
        100.0,
        100.0,
        100.0,
        mat_handle,
        mesh2d_bundle,
    ));
}
