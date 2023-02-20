use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::bullet::*;
use crate::common::{self, *};
use crate::wall::*;

pub const TANK_SIZE: Vec2 = Vec2::new(80.0, 80.0);
pub const TANK_SPEED: f32 = 200.0;
// 坦克离墙最近距离限制
pub const TANK_PADDING: f32 = 10.0;

// 坦克刷新子弹间隔
pub const TANK_REFRESH_BULLET_INTERVAL: f32 = 2.0;

// 坦克
#[derive(Component)]
pub struct Tank;

// 坦克刷新子弹计时器
#[derive(Component, Deref, DerefMut)]
pub struct TankRefreshBulletTimer(pub Timer);

// 出生保护盾
#[derive(Component)]
pub struct Shield;

// 出生保护盾计时
#[derive(Component, Deref, DerefMut)]
pub struct ShieldRemoveTimer(pub Timer);

pub fn setup_tank(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let shield_texture_handle = asset_server.load("textures/shield.bmp");
    let shield_texture_atlas = TextureAtlas::from_grid(
        shield_texture_handle,
        Vec2::new(30.0, 30.0),
        1,
        2,
        None,
        None,
    );
    let shield_texture_atlas_handle = texture_atlases.add(shield_texture_atlas);

    let tank_texture_handle = asset_server.load("textures/tank1.bmp");
    let tank_texture_atlas =
        TextureAtlas::from_grid(tank_texture_handle, Vec2::new(28.0, 28.0), 2, 4, None, None);
    let tank_texture_atlas_handle = texture_atlases.add(tank_texture_atlas);

    // 保护盾
    let shield = commands
        .spawn(Shield)
        .insert(SpriteSheetBundle {
            texture_atlas: shield_texture_atlas_handle,
            ..default()
        })
        .insert(AnimationTimer(Timer::from_seconds(
            0.2,
            TimerMode::Repeating,
        )))
        .insert(ShieldRemoveTimer(Timer::from_seconds(5.0, TimerMode::Once)))
        .id();

    // 坦克
    let tank = commands
        .spawn(Tank)
        .insert(SpriteSheetBundle {
            texture_atlas: tank_texture_atlas_handle,
            transform: Transform {
                translation: Vec3::new(0.0, BOTTOM_WALL + 100.0, 0.0),
                ..default()
            },
            ..default()
        })
        .insert(AnimationTimer(Timer::from_seconds(
            0.2,
            TimerMode::Repeating,
        )))
        .insert(TankRefreshBulletTimer(Timer::from_seconds(
            TANK_REFRESH_BULLET_INTERVAL,
            TimerMode::Once,
        )))
        .insert(common::Direction::Up)
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(18.0, 18.0))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(LockedAxes::ROTATION_LOCKED)
        .id();

    commands.entity(tank).add_child(shield);
}

// 移动坦克
pub fn move_tank(
    keyboard_input: Res<Input<KeyCode>>,
    mut transform_query: Query<
        (
            &mut Transform,
            &mut common::Direction,
            &mut TextureAtlasSprite,
        ),
        With<Tank>,
    >,
) {
    for (mut tank_transform, mut direction, mut sprite) in &mut transform_query {
        let mut tank_x_position = tank_transform.translation.x;
        let mut tank_y_position = tank_transform.translation.y;

        let ori_direction = direction.clone();
        // 一次只能移动一个方向
        // 根据速度时间计算新坐标
        if keyboard_input.pressed(KeyCode::Left) {
            tank_x_position -= 1.0 * TANK_SPEED * TIME_STEP;
            *direction = common::Direction::Left;
        } else if keyboard_input.pressed(KeyCode::Right) {
            tank_x_position += 1.0 * TANK_SPEED * TIME_STEP;
            *direction = common::Direction::Right;
        } else if keyboard_input.pressed(KeyCode::Up) {
            tank_y_position += 1.0 * TANK_SPEED * TIME_STEP;
            *direction = common::Direction::Up;
        } else if keyboard_input.pressed(KeyCode::Down) {
            tank_y_position -= 1.0 * TANK_SPEED * TIME_STEP;
            *direction = common::Direction::Down;
        } else {
            return;
        }

        if direction.clone() != ori_direction {
            match *direction {
                common::Direction::Up => {
                    sprite.index = 0;
                }
                common::Direction::Right => {
                    sprite.index = 2;
                }
                common::Direction::Down => {
                    sprite.index = 4;
                }
                common::Direction::Left => {
                    sprite.index = 6;
                }
            }
        }

        // TODO 利用碰撞   区域边界，确保坦克不会超出边界
        let left_bound = LEFT_WALL + WALL_THICKNESS / 2.0 + TANK_SIZE.x / 2.0 + TANK_PADDING;
        let right_bound = RIGHT_WALL - WALL_THICKNESS / 2.0 - TANK_SIZE.x / 2.0 - TANK_PADDING;
        let bottom_bound = BOTTOM_WALL + WALL_THICKNESS / 2.0 + TANK_SIZE.x / 2.0 + TANK_PADDING;
        let top_bound = TOP_WALL - WALL_THICKNESS / 2.0 - TANK_SIZE.x / 2.0 - TANK_PADDING;
        tank_transform.translation.x = tank_x_position.clamp(left_bound, right_bound);
        tank_transform.translation.y = tank_y_position.clamp(bottom_bound, top_bound);
    }
}

// 坦克移动动画播放
pub fn animate_tank(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<
        (
            &mut AnimationTimer,
            &mut TextureAtlasSprite,
            &Handle<TextureAtlas>,
            &common::Direction,
        ),
        With<Tank>,
    >,
) {
    for (mut timer, mut sprite, texture_atlas_handle, direction) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            // 切换到下一个sprite
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            // 每个方向上的sprite数量
            let sprites_each_direction = texture_atlas.len() / 4;
            match direction {
                common::Direction::Up => {
                    sprite.index =
                        (sprite.index + 1) % sprites_each_direction + sprites_each_direction * 0;
                }
                common::Direction::Right => {
                    sprite.index =
                        (sprite.index + 1) % sprites_each_direction + sprites_each_direction * 1;
                }
                common::Direction::Down => {
                    sprite.index =
                        (sprite.index + 1) % sprites_each_direction + sprites_each_direction * 2;
                }
                common::Direction::Left => {
                    sprite.index =
                        (sprite.index + 1) % sprites_each_direction + sprites_each_direction * 3;
                }
            }
        }
    }
}

// 坦克攻击
pub fn tank_attack(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Transform, &common::Direction, &mut TankRefreshBulletTimer), With<Tank>>,
    time: Res<Time>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    for (transform, direction, mut refresh_bullet_timer) in &mut query {
        refresh_bullet_timer.tick(time.delta());
        if keyboard_input.just_pressed(KeyCode::Space) {
            if refresh_bullet_timer.finished() {
                let bullet_texture_handle = asset_server.load("textures/bullet.bmp");
                let bullet_texture_atlas = TextureAtlas::from_grid(
                    bullet_texture_handle,
                    Vec2::new(7.0, 8.0),
                    4,
                    1,
                    None,
                    None,
                );
                let bullet_texture_atlas_handle = texture_atlases.add(bullet_texture_atlas);

                commands
                    .spawn(Bullet)
                    .insert(SpriteSheetBundle {
                        texture_atlas: bullet_texture_atlas_handle,
                        sprite: TextureAtlasSprite {
                            index: match direction {
                                common::Direction::Up => 0,
                                common::Direction::Right => 1,
                                common::Direction::Down => 2,
                                common::Direction::Left => 3,
                            },
                            ..default()
                        },
                        transform: Transform {
                            translation: Vec3::new(
                                transform.translation.x,
                                transform.translation.y,
                                transform.translation.z,
                            ),
                            ..default()
                        },
                        ..default()
                    })
                    .insert((
                        Collider::cuboid(2.0, 2.0),
                        Sensor,
                        RigidBody::Dynamic,
                        ActiveEvents::COLLISION_EVENTS,
                    ))
                    .insert(direction.clone());
                refresh_bullet_timer.reset();
            }
        }
    }
}

// 保护盾动画播放
pub fn animate_shield(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<
        (
            &mut AnimationTimer,
            &mut TextureAtlasSprite,
            &Handle<TextureAtlas>,
        ),
        With<Shield>,
    >,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            // 切换到下一个sprite
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
        }
    }
}

// 移除保护盾
pub fn remove_shield(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut ShieldRemoveTimer), With<Shield>>,
) {
    for (entity, mut timer) in query.iter_mut() {
        timer.tick(time.delta());

        if timer.finished() {
            commands.entity(entity).despawn();
        }
    }
}
