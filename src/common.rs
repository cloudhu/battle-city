use bevy::prelude::*;
// 关卡地图行数和列数 // Level map row and column numbers
/*
pub const LDTK_MAP: &str = "levels_namcot_original_35.ldtk";
pub const LEVEL_ROWS: i32 = 13;
pub const LEVEL_COLUMNS: i32 = 13;
pub const TILE_SIZE: f32 = 32.0;
 */

// Uncomment it to return to a custom map settings
pub const LDTK_MAP: &str = "levels.ldtk";
pub const LEVEL_ROWS: i32 = 18;
pub const LEVEL_COLUMNS: i32 = 27;
pub const TILE_SIZE: f32 = 32.0;

// 关卡数量 // Number of levels
pub const MAX_LEVELS: i32 = 2;
// 同时共存的敌人最大数量 // The maximum number of enemies that can coexist at the same time
pub const MAX_LIVE_ENEMIES: i32 = 5;
// 每关敌人数量 // Number of enemies per level
pub const ENEMIES_PER_LEVEL: i32 = 12;
// 坦克刷新子弹间隔（秒）// Tank refresh bullet interval (seconds)
pub const PLAYER_REFRESH_BULLET_INTERVAL: f32 = 0.5;
pub const ENEMY_REFRESH_BULLET_INTERVAL: f32 = 2.0;
// 坦克速度、大小和缩放比例 // Tank speed, size and scaling
pub const PLAYER_SPEED: f32 = 150.0;
pub const ENEMY_SPEED: f32 = 100.0;
pub const TANK_SIZE: f32 = 32.0;
pub const TANK_SCALE: f32 = 1.0;
pub const TANK_ROUND_CORNERS: f32 = 8.0;
pub const PHYSICS_SCALE_PER_METER: f32 = 100.0;

// sprite z轴顺序 // Sprite z-axis order
pub const SPRITE_GAME_OVER_Z_ORDER: f32 = 4.0;
pub const SPRITE_TREE_Z_ORDER: f32 = 3.0;
pub const SPRITE_PLAYER_Z_ORDER: f32 = 1.0;
pub const TANKS_SPRITE: &str = "textures/spriteTanks32.png";
pub const TANKS_SPRITE_CELL: u32 = 32;
pub const TANKS_SPRITE_COLS_AMOUNT: i32 = 16;
pub const MAP_SPRITE: &str = "textures/spriteMapObjects32.png";
pub const MAP_SPRITE_CELL: u32 = 32;

pub const BEVY_FRAMERATE: f32 = 0.14; // TODO: Change it to 0.15 for Bevy v0.13

#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Default)]
pub enum AppState {
    #[default]
    Splash,
    StartMenu,
    Playing,
    Paused,
    GameOver,
}

#[derive(Resource, Debug, PartialEq, Eq)]
pub enum MultiplayerMode {
    SinglePlayer,
    TwoPlayers,
}

// 方向 // Direction
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Component, Clone, Default, Debug)]
pub struct AnimationTimer(pub Timer);

#[derive(Component, Clone, Default, Debug)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

// 坦克刷新子弹计时器 // Tank refresh bullet timer
#[derive(Component, Deref, DerefMut)]
pub struct TankRefreshBulletTimer(pub Timer);

#[derive(Default, Event)]
pub struct HomeDyingEvent;

#[derive(Debug, Resource)]
pub struct GameSounds {
    pub start_menu: Handle<AudioSource>,
    pub mode_switch: Handle<AudioSource>,
    pub bullet_explosion: Handle<AudioSource>,
    pub big_explosion: Handle<AudioSource>,
    pub player_fire: Handle<AudioSource>,
    pub game_over: Handle<AudioSource>,
    pub game_pause: Handle<AudioSource>,
}

#[derive(Resource)]
pub struct GameTextureLayout {
    pub tanks: Handle<TextureAtlasLayout>,
    pub map: Handle<TextureAtlasLayout>,
    pub bullet: Handle<TextureAtlasLayout>,
    pub born: Handle<TextureAtlasLayout>,
}

#[derive(Debug, Resource)]
pub struct GameTextureHandles {
    pub tanks: Handle<Image>,
    pub map: Handle<Image>,
    pub bullet: Handle<Image>,
    pub born: Handle<Image>,
}
pub fn setup_game_sounds(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(GameSounds {
        start_menu: asset_server.load("sounds/start_menu.ogg"),
        mode_switch: asset_server.load("sounds/mode_switch.ogg"),
        bullet_explosion: asset_server.load("sounds/bullet_explosion.ogg"),
        big_explosion: asset_server.load("sounds/big_explosion.ogg"),
        player_fire: asset_server.load("sounds/player_fire.ogg"),
        game_over: asset_server.load("sounds/game_over.ogg"),
        game_pause: asset_server.load("sounds/game_pause.ogg"),
    });
}

pub fn setup_game_texture_atlas(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    // 炮弹 // Bullet
    let bullet_texture_handle = asset_server.load("textures/bullet.bmp");
    let bullet_texture_layout = TextureAtlasLayout::from_grid(UVec2::new(7, 8), 4, 1, None, None);
    let bullet_atlas_layout = texture_atlas_layouts.add(bullet_texture_layout);

    // 出生效果 // Birth effects
    let born_texture_handle = asset_server.load("textures/born.bmp");
    let born_texture_layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 4, 1, None, None);
    let born_atlas_layout = texture_atlas_layouts.add(born_texture_layout);

    // Players and enemies tanks sprite
    let tanks_texture_handle = asset_server.load(TANKS_SPRITE);
    let tanks_texture_atlas_layout =
        TextureAtlasLayout::from_grid(UVec2::splat(TANKS_SPRITE_CELL), 16, 16, None, None);
    let tanks_atlas_layout = texture_atlas_layouts.add(tanks_texture_atlas_layout);

    // Map objects sprite with Home and Shield
    let map_texture_handle = asset_server.load(MAP_SPRITE);
    let map_texture_atlas_layout =
        TextureAtlasLayout::from_grid(UVec2::splat(MAP_SPRITE_CELL), 5, 4, None, None);
    let map_atlas_layout = texture_atlas_layouts.add(map_texture_atlas_layout);

    commands.insert_resource(GameTextureLayout {
        tanks: tanks_atlas_layout,
        map: map_atlas_layout,
        bullet: bullet_atlas_layout,
        born: born_atlas_layout,
    });
    commands.insert_resource(GameTextureHandles {
        tanks: tanks_texture_handle,
        map: map_texture_handle,
        bullet: bullet_texture_handle,
        born: born_texture_handle,
    });
    app_state.set(AppState::StartMenu);
}

pub fn animate_sprite<T: Component>(
    time: Res<Time>,
    mut query: Query<(&mut AnimationTimer, &AnimationIndices, &mut TextureAtlas), With<T>>,
) {
    for (mut timer, indices, mut sprite) in &mut query {
        timer.0.tick(time.delta());
        if timer.0.just_finished() {
            // 切换到下一个sprite // Switch to next sprite
            sprite.index = if sprite.index == indices.last {
                indices.first
            } else {
                sprite.index + 1
            };
        }
    }
}
