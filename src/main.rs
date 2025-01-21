mod pos;
use std::{env, ops::DerefMut, time::Duration};

use avian2d::prelude::*;
use bevy::{input::keyboard::{Key, KeyboardInput}, math::VectorSpace, prelude::*, render::sync_world::SyncToRenderWorld};
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
        .add_plugins(DefaultPlugins)
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

        .run();
}
// const ATLAS_FILE: &str = "Cave Tiles.png";
// const ATLAS_FILE: &str = "Jotem spritesheet.png";
const ATLAS_FILE_IDLE: &str = r"PenUsbMic\Small Monster\small moidle.png";
const ATLAS_FILE_WALK: &str = r"PenUsbMic\Small Monster\small morun.png";
pub fn setup(mut commands: Commands, mut assets: Res<AssetServer>) {
    commands.spawn((MyCamera,Transform::from_xyz(0.0, 0.0, 0.0)));
    {
        let textue_atlas_layout = assets.add(TextureAtlasLayout::from_grid(
            UVec2::new(28, 39),//UVec2::new(82, 39),
            1,
            6,
            None,//Some(UVec2::new(0, 234-39)),
            None,
        ));
        let animation_config = AnimationConfig::new(0, 5, 10);
        let texture_atlas = TextureAtlas {
            layout: textue_atlas_layout,
            index: animation_config.first_sprite_index,
        };
        let bundle = (
            Sprite {
                image: assets.load(ATLAS_FILE_IDLE),
                texture_atlas: Some(texture_atlas),
                ..default()
            },
            animation_config,
            Transform::from_translation(Vec3::new(-50.0,0.0,0.0)).with_scale(Vec3::splat(3.0)),
            Visibility::Visible,
            SyncToRenderWorld,
        );
        commands.spawn(bundle);

    }
    {
        let textue_atlas_layout = assets.add(TextureAtlasLayout::from_grid(
            UVec2::new(28, 39),//UVec2::new(82, 39),
            1,
            6,
            None,//Some(UVec2::new(0, 234-39)),
            None,
        ));
        let animation_config = AnimationConfig::new(0, 5, 10);
        let texture_atlas = TextureAtlas {
            layout: textue_atlas_layout,
            index: animation_config.first_sprite_index,
        };
        let bundle = (
            Sprite {
                image: assets.load(ATLAS_FILE_WALK),
                texture_atlas: Some(texture_atlas),
                ..default()
            },
            animation_config,
            Transform::from_translation(Vec3::new(50.0,0.0,0.0)).with_scale(Vec3::splat(3.0)),
            Visibility::Visible,
            SyncToRenderWorld,
            MossMonsterWalk
        );
        commands.spawn(bundle);
    }
}


pub fn swap_tiles(time: Res<Time>, mut objs: Query<&mut Sprite>){
    // if (time.elapsed_secs()) % 3.0 != 0.0{
    //     return
    // }
    println!("SWAP: {}", time.elapsed_secs());
    for mut sprite in objs.iter_mut(){
        if let Some(atlas) = &mut sprite.texture_atlas{
            atlas.index = (atlas.index+1) % 6;
        }
    }   
}
pub fn animate_stuff(time: Res<Time>, mut query: Query<(&mut AnimationConfig, &mut Sprite)>){
    for (mut config, mut sprite) in &mut query{
        // How long the current sprite has been active.
        config.frame_timer.tick(time.delta());

        // If it has ben displayed for the expected number of frames:
        if config.frame_timer.just_finished(){
            if let Some(atlas) = &mut sprite.texture_atlas{
                if atlas.index != config.last_sprite_index{
                    config.frame_timer = AnimationConfig::timer_from_fps(config.fps);
                }
                atlas.index = (1 + atlas.index - config.first_sprite_index) % (config.last_sprite_index)
            }
        }
    }
}
#[derive(Component)]
pub struct MossMonsterWalk;
#[derive(Component)]
pub struct MossMonsterIdle;

#[derive(Component)]
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
