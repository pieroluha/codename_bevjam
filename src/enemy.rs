use crate::creature::{stats::*, CreatureAssets, EnemyBundle};
use crate::player::Player;
use crate::{get_direction, GameState};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

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
            .add_system(set_spawn.in_set(OnUpdate(GameState::Playing)))
            .add_system(hunt_player.in_set(OnUpdate(GameState::Playing)))
            .add_system(stop_enemies.in_schedule(OnExit(GameState::Playing)));
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
                (100.0 - (fastrand::f32() * 10.0)) * power_mult.0,
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

fn set_spawn(
    mut que_enemies: Query<(&mut Transform, &mut Velocity, Entity), With<NewEnemy>>,
    mut cmds: Commands,
) {
    for (mut enemy, mut vel, entity) in que_enemies.iter_mut() {
        enemy.translation = SPAWN_LOC[fastrand::usize(..SPAWN_LOC.len())].extend(1.0);
        vel.linvel = Vec2::ZERO;
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

fn hunt_player(
    que_player: Query<&Transform, (With<Player>, Without<Enemy>)>,
    mut que_enemies: Query<(&mut Velocity, &Transform, &Speed), (With<Enemy>, Without<Player>)>,
) {
    let player = que_player.single();
    for (mut vel, enemy, speed) in que_enemies.iter_mut() {
        let target_pos = player.translation.truncate();
        // let pos = enemy.translation.truncate();
        let pos = enemy.translation.truncate();
        // let move_dir = get_direction(pos, target_pos);
        // enemy.translation += (move_dir * speed.0 * time.delta_seconds()).extend(1.0);

        let move_dir = get_direction(pos, target_pos);

        vel.linvel = move_dir * speed.0;
    }
    // let vel = move_dir * speed.0 * time.delta_seconds();
    // trans.translation += vel.extend(0.0);
}

fn stop_enemies(mut que_enemies: Query<&mut Velocity, With<Enemy>>) {
    for mut enemy in que_enemies.iter_mut() {
        enemy.linvel = Vec2::ZERO;
    }
}
