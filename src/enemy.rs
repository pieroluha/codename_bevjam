use crate::creature::{CreatureAssets, EnemyBundle};
use crate::GameState;
use bevy::prelude::*;

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnEnemies>()
            .init_resource::<SpawnTimer>()
            .init_resource::<EnemyPowerMult>()
            .init_resource::<EnemyCount>()
            .add_system(spawn_timer.in_set(OnUpdate(GameState::Playing)))
            .add_system(count_enemies.in_set(OnUpdate(GameState::Playing)))
            .add_system(spawn_enemies.in_set(OnUpdate(GameState::Playing)))
            .add_system(set_spawn.in_set(OnUpdate(GameState::Playing)));
    }
}

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct NewEnemy;

const SPAWN_LOC: [Vec2; 8] = [
    Vec2 {
        x: -1100.0,
        y: 1100.0,
    },
    Vec2 { x: -1100.0, y: 0.0 },
    Vec2 {
        x: -1100.0,
        y: -1100.0,
    },
    Vec2 { x: 0.0, y: 1100.0 },
    Vec2 {
        x: 1100.0,
        y: 1100.0,
    },
    Vec2 { x: 1100.0, y: 0.0 },
    Vec2 {
        x: 1100.0,
        y: -1100.0,
    },
    Vec2 { x: 0.0, y: -1100.0 },
];

#[derive(Resource)]
struct EnemyPowerMult(f32);
impl Default for EnemyPowerMult {
    fn default() -> Self {
        Self(1.0)
    }
}

#[derive(Resource, Deref, DerefMut)]
struct SpawnTimer(Timer);
impl Default for SpawnTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(10.0, TimerMode::Repeating))
    }
}
#[derive(Resource, Default)]
struct EnemyCount(u32);

const SPAWN_CAP: u32 = 30;
const ENEMY_CAP: u32 = 100;

struct SpawnEnemies {
    count: u32,
}

fn spawn_timer(
    time: Res<Time>,
    power_mult: Res<EnemyPowerMult>,
    enemy_count: Res<EnemyCount>,
    mut event_writer: EventWriter<SpawnEnemies>,
    mut spawn_timer: ResMut<SpawnTimer>,
) {
    spawn_timer.0.tick(time.delta());

    if spawn_timer.0.just_finished() && enemy_count.0 < ENEMY_CAP {
        event_writer.send(SpawnEnemies {
            count: fastrand::u32(8..(15.0 * power_mult.0) as u32).min(SPAWN_CAP),
            // loc: SPAWN_LOC[fastrand::usize(..SPAWN_LOC.len())],
        })
    }
}

fn spawn_enemies(
    cre_ass: Res<CreatureAssets>,
    power_mult: Res<EnemyPowerMult>,
    mut cmds: Commands,
    mut event_reader: EventReader<SpawnEnemies>,
) {
    for event in event_reader.iter() {
        let mut batch = vec![];
        let sprite_sheet = SpriteSheetBundle {
            texture_atlas: cre_ass.enemy.clone(),
            sprite: TextureAtlasSprite::new(0),
            transform: Transform::from_xyz(100000.0, 100000.0, 1.0),
            ..default()
        };
        for _ in 0..event.count {
            batch.push(EnemyBundle::new(
                100.0 * power_mult.0,
                100.0 * power_mult.0,
                100.0 * power_mult.0,
                0.5,
                0.5,
                10.0,
                10.0,
                sprite_sheet.clone(),
            ));
        }

        cmds.spawn_batch(batch);
    }
}

fn set_spawn(mut que_enemies: Query<(&mut Transform, Entity), With<NewEnemy>>, mut cmds: Commands) {
    for (mut enemy, entity) in que_enemies.iter_mut() {
        enemy.translation = SPAWN_LOC[fastrand::usize(..SPAWN_LOC.len())].extend(1.0);
        cmds.entity(entity).remove::<NewEnemy>();
    }
}

fn count_enemies(que_enemies: Query<&Enemy>, mut enemy_count: ResMut<EnemyCount>) {
    let mut count = 0;
    for _ in que_enemies.iter() {
        count += 1;
    }
    enemy_count.0 = count;
}
