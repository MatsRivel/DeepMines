mod pos;
use std::{any::Any, env, ops::DerefMut, time::Duration};

use avian2d::prelude::*;
use bevy::{
    ecs::observer::TriggerEvent,
    input::keyboard::{Key, KeyboardInput},
    math::VectorSpace,
    prelude::*,
    render::{sync_world::SyncToRenderWorld, view::WindowRenderPlugin},
    window::{WindowResized, WindowResolution},
};
use bevy_ecs_tilemap::prelude::*;
use bevy_tnua::prelude::*;
use bevy_tnua_avian2d::TnuaAvian2dPlugin;
use camera::MyCamera;
use tile::Tile;
pub const TILE_WIDTH: f32 = 16.0;
pub const TRENCH_WIDT: f32 = 0.0;
pub const STEP_SIZE: f32 = TILE_WIDTH + TRENCH_WIDT;
pub const N_TILES: i32 = 120;
pub const SKY_COLOR: Color = Color::linear_rgb(0.5, 0.5, 0.1);

#[derive(Default, Reflect, GizmoConfigGroup)]
pub struct MyGizmos;

fn main() {
    unsafe {
        std::env::set_var("WGPU_BACKEND", "vk");
    }
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(200., 100.).with_scale_factor_override(1.0),
                ..Default::default()
            }),
            ..default()
        }))
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(TnuaControllerPlugin::default())
        .add_plugins(TnuaAvian2dPlugin::new(Update))
        .insert_resource(ClearColor(SKY_COLOR))
        .init_gizmo_group::<MyGizmos>()
        .insert_resource(Time::<Fixed>::from_hz(30.0)) //This messes with time.
        .add_systems(Startup, setup)
        // .add_systems(FixedUpdate, swap_tiles)
        // .add_systems(FixedUpdate, animate_stuff)
        .add_systems(FixedUpdate, animate_stuff)
        .add_systems(FixedUpdate, key_trigger_animation)
        .run();
}
// const ATLAS_FILE: &str = "Cave Tiles.png";
// const ATLAS_FILE: &str = "Jotem spritesheet.png";
const ATLAS_FILE_IDLE: &str = r"PenUsbMic\Small Monster\small moidle.png";
const ATLAS_FILE_WALK: &str = r"PenUsbMic\Small Monster\small morun.png";
const ATLAS_FILE_ATTACK: &str = r"PenUsbMic\Small Monster\attack.png";
pub fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn((MyCamera, Transform::from_xyz(0.0, 0.0, 0.0)));
    let textue_atlas_layout = assets.add(TextureAtlasLayout::from_grid(
        UVec2::new(28, 39),
        // UVec2::new(234, 39),
        1,
        6,
        None, //Some(UVec2::new(234-82, 0)),
        None,
    ));
    let animation_config = AnimationConfig::new(0, 5, 10);
    let texture_atlas = TextureAtlas {
        layout: textue_atlas_layout,
        index: animation_config.first_sprite_index,
    };
    let idle_sprite = Sprite {
        image: assets.load(ATLAS_FILE_IDLE),
        texture_atlas: Some(texture_atlas.clone()),
        ..default()
    };
    let idle_bundle = (
        idle_sprite,
        animation_config.clone(),
        Transform::from_translation(Vec3::new(-75.0, 0.0, 0.0)).with_scale(Vec3::splat(3.0)),
        Visibility::Visible,
        SyncToRenderWorld,
        MossMonsterIdle,
    );
    // commands.spawn(idle_bundle);

    let walking_sprite = Sprite {
        image: assets.load(ATLAS_FILE_WALK),
        texture_atlas: Some(texture_atlas.clone()),
        ..default()
    };
    let walking_bundle = (
        walking_sprite,
        animation_config.clone(),
        Transform::from_translation(Vec3::new(75.0, 0.0, 0.0)).with_scale(Vec3::splat(3.0)),
        Visibility::Visible,
        SyncToRenderWorld,
        MossMonsterWalk,
    );
    // commands.spawn(walking_bundle);

    let var_sprite = Sprite {
        image: assets.load(ATLAS_FILE_IDLE),
        texture_atlas: Some(texture_atlas),
        ..default()
    };
    let variation_bundle = (
        var_sprite,
        animation_config,
        Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)).with_scale(Vec3::splat(3.0)),
        Visibility::Visible,
        SyncToRenderWorld,
        MossMonsterVary::new(),
    );
    commands.spawn(variation_bundle);
}

pub fn key_trigger_animation(mut commands: Commands, mut event_reader: EventReader<KeyboardInput>, entity_query: Query<(Entity, &mut Sprite), With<MossMonsterVary>>,  mut assets: Res<AssetServer>) {
    for kb in event_reader
        .read()
    {
        let kb_code = kb.key_code;
        let (entity, sprite)= entity_query.single();
        if let Some(texture_atlas) = &sprite.texture_atlas{
            let next_state = match kb_code{
                KeyCode::KeyD => MossMonsterState::WalkRight,
                KeyCode::KeyA => MossMonsterState::WalkLeft,
                KeyCode::KeyS => MossMonsterState::Idle,
                KeyCode::KeyW => MossMonsterState::Attack,
                _ => continue
            };
            // This controls when to change sprites and when to flip sprites.
            let flip = next_state.flip_x_state().unwrap_or(sprite.flip_x);
            let assets_handle = assets.load(next_state.as_source_file());
            let new_sprite = Sprite {
                image: assets_handle,
                texture_atlas: Some(texture_atlas.clone()),
                flip_x: flip,
                ..default()
            };
            commands.entity(entity).remove::<Sprite>();
            commands.entity(entity).insert(new_sprite);
        }
    }
}

pub fn animate_stuff(
    time: Res<Time>,
    mut query: Query<(&mut AnimationConfig, &mut Sprite)>,
) {
    for (mut config, mut sprite) in &mut query {
        // How long the current sprite has been active.
        config.frame_timer.tick(time.delta());

        // If it has ben displayed for the expected number of frames:
        if config.frame_timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                if atlas.index != config.last_sprite_index {
                    config.frame_timer = AnimationConfig::timer_from_fps(config.fps);
                }
                atlas.index =
                    (1 + atlas.index - config.first_sprite_index) % (config.last_sprite_index)
            }
        }
    }
}

#[derive(Component)]
pub struct MossMonsterWalk;
#[derive(Component)]
pub struct MossMonsterIdle;

#[derive(Clone,Copy,Debug,PartialEq, Eq)]
pub enum MossMonsterState{
    WalkLeft,
    WalkRight,
    Idle,
    Attack
}impl MossMonsterState{
    fn as_source_file<'a>(&'a self)->&'a str{
        match self{
            Self::WalkLeft | Self::WalkRight => ATLAS_FILE_WALK,
            Self::Idle => ATLAS_FILE_IDLE,
            Self::Attack => ATLAS_FILE_ATTACK,
        }
    }
    fn new_file_needed<'a>(&'a self, other: &'a Self)->Option<&'a str>{
        if self.as_source_file() == other.as_source_file(){
            None
        }else{
            Some(&other.as_source_file())
        }
    }
    fn flip_x_state<'a>(&'a self)->Option<bool>{
        match self{
            Self::WalkLeft => Some(true),
            Self::WalkRight => Some(false),
            Self::Idle | Self::Attack => None,
        }
    }
    pub fn to_data<'a>(&'a self, other: &'a Self)->(Option<bool>, Option<&'a str>){
        (other.flip_x_state(), self.new_file_needed(other))
    }
}

#[derive(Component)]
pub struct MossMonsterVary{
    state: MossMonsterState
}impl MossMonsterVary{
    pub fn new()->Self{
        Self { state: MossMonsterState::Idle }
    }
    pub fn walk_left<'a>(&'a mut self)->(Option<bool>, Option<&'a str>){
        self.state.to_data(&MossMonsterState::WalkLeft)
    }
    pub fn walk_idle<'a>(&'a mut self)->(Option<bool>, Option<&'a str>){
        self.state.to_data(&MossMonsterState::Idle)
    }
    pub fn walk_right<'a>(&'a mut self)->(Option<bool>, Option<&'a str>){
        self.state.to_data(&MossMonsterState::WalkRight)
    }
    pub fn attack<'a>(&'a mut self)->(Option<bool>, Option<&'a str>){
        self.state.to_data(&MossMonsterState::Attack)
    }
}

#[derive(Component, Clone)]
pub struct AnimationConfig {
    first_sprite_index: usize,
    last_sprite_index: usize,
    fps: u8,
    frame_timer: Timer,
}

impl AnimationConfig {
    fn new(first: usize, last: usize, fps: u8) -> Self {
        Self {
            first_sprite_index: first,
            last_sprite_index: last,
            fps,
            frame_timer: Self::timer_from_fps(fps),
        }
    }

    fn timer_from_fps(fps: u8) -> Timer {
        Timer::new(Duration::from_secs_f32(1.0 / (fps as f32)), TimerMode::Once)
    }
}

mod tile {
    use super::*;
    #[derive(Component, Default, Clone, Copy)]
    #[require(Sprite, Transform)]
    pub struct Tile;
}

mod camera {
    use super::*;
    #[derive(Component, Default, Clone, Copy)]
    #[require(Camera, Camera2d, Transform)]
    pub struct MyCamera;
}
