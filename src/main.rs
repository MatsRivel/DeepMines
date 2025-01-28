mod pos;
mod animation_manager;

use std::time::Duration;
use animation_manager::{build_animation_from_atlas, build_atlas_from_folder_of_frames, check_whether_image_folders_are_loaded, load_image_folders, AnimationConfig, FolderOfImagesCollection, ImageLoadingState, MetaTextureAtlas};
use avian2d::prelude::*;
use bevy::{
    asset::{ErasedAssetLoader, LoadedFolder},
    input::keyboard::KeyboardInput,
    prelude::*,
    render::sync_world::SyncToRenderWorld,
    window::WindowResolution,
};
use bevy_tnua::prelude::*;
use bevy_tnua_avian2d::TnuaAvian2dPlugin;
use camera::MyCamera;
pub const TILE_WIDTH: f32 = 16.0;
pub const TRENCH_WIDT: f32 = 0.0;
pub const STEP_SIZE: f32 = TILE_WIDTH + TRENCH_WIDT;
pub const N_TILES: i32 = 120;
pub const SKY_COLOR: Color = Color::linear_rgb(0.5, 0.5, 0.1);
// const ATLAS_FILE: &str = "Cave Tiles.png";
// const ATLAS_FILE: &str = "Jotem spritesheet.png";
const ATLAS_FILE_IDLE: &str = r"PenUsbMic\Small Monster\small moidle.png";
const ATLAS_FILE_WALK: &str = r"PenUsbMic\Small Monster\small morun.png";
const ATLAS_FILE_ATTACK: &str = r"PenUsbMic\Small Monster\attack.png";
const SOURCE_FOLDER1: &str = r"test_images\first";
const SOURCE_FOLDER2: &str = r"test_images\second";
const IMAGE_PATHS: [&str;2] = [SOURCE_FOLDER1,SOURCE_FOLDER2];
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
        .init_state::<ImageLoadingState>()
        .add_systems(OnEnter(ImageLoadingState::Setup), crate::animation_manager::load_image_folders)
        .add_systems(Update, check_whether_image_folders_are_loaded.run_if(in_state(ImageLoadingState::Setup)))
        .add_systems(OnEnter(ImageLoadingState::Finished), setup)
        // .add_systems(Startup, (
        //     load_textures.before(setup),
        //     setup.after(load_textures)
        // ))
        .add_systems(FixedUpdate, animate_stuff)
        // .add_systems(FixedUpdate, animate_all)
        // .add_systems(FixedUpdate, key_trigger_animation)
        .add_systems(FixedUpdate, animation_trigger_demo)
        .run();
}

/// Takes the files in the folder, in order, and adds them to an atlas.
/// NOTE: Does not work well for sprite-sheets.
pub fn build_atlas_from_folder_of_frames_old(
    folder1: &LoadedFolder,
    folder2: &LoadedFolder,
    textures: &mut ResMut<Assets<Image>>,
) -> (TextureAtlasLayout, TextureAtlasSources, Handle<Image>) {
    let mut atlas_builder = TextureAtlasBuilder::default();

    for handle in folder1.handles.iter() {
        info!("Handle from folders: {}", handle.path().unwrap());
        let Ok(id) = handle.id().try_typed::<Image>() else {
            warn!("Wrong type for {handle:?}: {:?}", handle.path().unwrap());
            continue;
        };
        let Some(texture) = textures.get(id) else {
            warn!("Missing image for {:?}", handle.path().unwrap());
            continue;
        };

        atlas_builder.add_texture(Some(id), texture);
    }
    for handle in folder2.handles.iter() {
        // continue;
        info!("Handle from folders: {}", handle.path().unwrap());
        let Ok(id) = handle.id().try_typed::<Image>() else {
            warn!("Wrong type for {handle:?}: {:?}", handle.path().unwrap());
            continue;
        };
        let Some(texture) = textures.get(id) else {
            warn!("Missing image for {:?}", handle.path().unwrap());
            continue;
        };
        // let a = TextureAtlasLayout::from_grid(UVec2::new(28, 39), 1, 6, None, None);
        // let b = TextureAtlas {
        //     layout: assets.add(a),
        //     index: 0,
        atlas_builder.add_texture(Some(id), texture);
    }

    let (texture_atlas_layout, texture_atlas_sources, texture_atlas_image) =
        atlas_builder.build().unwrap();
    let texture_atlas_image_handle = textures.add(texture_atlas_image);
    (
        texture_atlas_layout,
        texture_atlas_sources,
        texture_atlas_image_handle,
    )
}
/// Takes the files in the folder, in order, and adds them to an atlas.
/// NOTE: Does not work well for sprite-sheets.
pub fn build_atlas_from_folder_of_spritesheets(
    assets: &mut Res<AssetServer>,
    folder1: &LoadedFolder,
    folder2: &LoadedFolder,
    textures: &mut ResMut<Assets<Image>>,
) -> (TextureAtlasLayout, TextureAtlasSources, Handle<Image>) {
    let mut atlas_builder = TextureAtlasBuilder::default();

    for handle in folder1.handles.iter() {
        info!("Handle from folders: {}", handle.path().unwrap());
        let Ok(id) = handle.id().try_typed::<Image>() else {
            warn!("Wrong type for {handle:?}: {:?}", handle.path().unwrap());
            continue;
        };
        let Some(texture) = textures.get(id) else {
            warn!("Missing image for {:?}", handle.path().unwrap());
            continue;
        };
        // let a = TextureAtlasLayout::from_grid(UVec2::new(28, 39), 1, 6, None, None);
        // let b = TextureAtlas {
        //     layout: assets.add(a),
        //     index: 0,
        // };
        atlas_builder.add_texture(Some(id), texture);
    }
        
    
    for handle in folder2.handles.iter() {
        info!("Handle from folders: {}", handle.path().unwrap());
        let Ok(id) = handle.id().try_typed::<Image>() else {
            warn!("Wrong type for {handle:?}: {:?}", handle.path().unwrap());
            continue;
        };
        let Some(texture) = textures.get(id) else {
            warn!("Missing image for {:?}", handle.path().unwrap());
            continue;
        };
        // let a = TextureAtlasLayout::from_grid(UVec2::new(28, 39), 1, 6, None, None);
        // let b = TextureAtlas {
        //     layout: assets.add(a),
        //     index: 0,
        atlas_builder.add_texture(Some(id), texture);
    }
    
    let (texture_atlas_layout, texture_atlas_sources, texture_atlas_image) =
        atlas_builder.build().unwrap();
    let texture_atlas_image_handle = textures.add(texture_atlas_image);
    (
        texture_atlas_layout,
        texture_atlas_sources,
        texture_atlas_image_handle,
    )
}

pub fn setup(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut textures: ResMut<Assets<Image>>,
    loaded_folders: Res<'_, Assets<LoadedFolder>>,
    folder_handles: Res<FolderOfImagesCollection>,
) {
    info!("Setup.");
    commands.spawn((MyCamera, Transform::from_xyz(0.0, 0.0, 0.0)));
    let folders = folder_handles.inner().iter().filter_map(|handle| loaded_folders.get(handle)).enumerate().collect();
    let meta = build_atlas_from_folder_of_frames(folders, &mut textures);
    build_animation_from_atlas(&mut commands, &assets, meta);
}

pub fn key_trigger_animation(
    mut commands: Commands,
    mut event_reader: EventReader<KeyboardInput>,
    entity_query: Query<(Entity, &mut Sprite), With<MossMonsterVary>>,
    assets: Res<AssetServer>,
) {
    for kb in event_reader.read() {
        let kb_code = kb.key_code;
        let (entity, sprite) = entity_query.single();
        if let Some(texture_atlas) = &sprite.texture_atlas {
            let next_state = match kb_code {
                KeyCode::KeyD => MossMonsterState::WalkRight,
                KeyCode::KeyA => MossMonsterState::WalkLeft,
                KeyCode::KeyS => MossMonsterState::Idle,
                KeyCode::KeyW => MossMonsterState::Attack,
                _ => continue,
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


pub fn animation_trigger_demo(
    mut commands: Commands,
    mut event_reader: EventReader<KeyboardInput>,
    entity_query: Query<(Entity, &mut Sprite, &MetaTextureAtlas), (With<MossMonsterVary>, With<AnimationConfig>)>,
    assets: Res<AssetServer>,
) {
    for kb in event_reader.read() {
        let kb_code = kb.key_code;
        let (entity, sprite, meta_atlas) = entity_query.single();
        if let Some(texture_atlas) = &sprite.texture_atlas {
            // This controls when to change sprites and when to flip sprites.
            let flip = kb_code == KeyCode::KeyS;
            let mut index = 0;
            if kb_code == KeyCode::Space{
                index = 1;
            }
            let (_,animation_config) = meta_atlas.animation_congfigs[index].clone();
            let assets_handle = meta_atlas.image_handle.clone();
            let new_sprite = Sprite {
                image: assets_handle,
                texture_atlas: Some(texture_atlas.clone()),
                flip_x: flip,
                ..default()
            };
            commands.entity(entity).remove::<Sprite>();
            commands.entity(entity).insert(new_sprite);
            commands.entity(entity).remove::<AnimationConfig>();
            commands.entity(entity).insert(animation_config);
        }
    }
}

pub fn animate_stuff(time: Res<Time>, mut query: Query<(&mut AnimationConfig, &mut Sprite)>) {
    for (mut config, mut sprite) in &mut query {
        // How long the current sprite has been active.
        config.frame_timer.tick(time.delta());
        // If it has ben displayed for the expected number of frames:
        // println!("{:?}",config.frame_timer);
        if config.frame_timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                let last_idx = config.last_sprite_index;
                let first_idx = config.first_sprite_index;
                let mut next_frame_index = atlas.index +1;

                if next_frame_index <= last_idx || true{
                    println!("### A");
                    config.frame_timer = AnimationConfig::timer_from_fps(config.fps);
                }
                if next_frame_index > last_idx || next_frame_index < first_idx{
                    println!("### B");
                    next_frame_index = first_idx;
                }
                println!("{first_idx} <= {next_frame_index} <= {last_idx}");
                atlas.index = next_frame_index;
            }
        }
    }
}

#[derive(Component)]
pub struct MossMonsterWalk;
#[derive(Component)]
pub struct MossMonsterIdle;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MossMonsterState {
    WalkLeft,
    WalkRight,
    Idle,
    Attack,
}
impl MossMonsterState {
    fn as_source_file(&self) -> &str {
        match self {
            Self::WalkLeft | Self::WalkRight => ATLAS_FILE_WALK,
            Self::Idle => ATLAS_FILE_IDLE,
            Self::Attack => ATLAS_FILE_ATTACK,
        }
    }
    fn new_file_needed<'a>(&'a self, other: &'a Self) -> Option<&'a str> {
        if self.as_source_file() == other.as_source_file() {
            None
        } else {
            Some(other.as_source_file())
        }
    }
    fn flip_x_state(&self) -> Option<bool> {
        match self {
            Self::WalkLeft => Some(true),
            Self::WalkRight => Some(false),
            Self::Idle | Self::Attack => None,
        }
    }
    pub fn to_data<'a>(&'a self, other: &'a Self) -> (Option<bool>, Option<&'a str>) {
        (other.flip_x_state(), self.new_file_needed(other))
    }
}

#[derive(Component)]
pub struct MossMonsterVary {
    state: MossMonsterState,
}
impl Default for MossMonsterVary {
    fn default() -> Self {
        Self::new()
    }
}

impl MossMonsterVary {
    pub fn new() -> Self {
        Self {
            state: MossMonsterState::Idle,
        }
    }
    pub fn walk_left(&mut self) -> (Option<bool>, Option<&str>) {
        self.state.to_data(&MossMonsterState::WalkLeft)
    }
    pub fn walk_idle(&mut self) -> (Option<bool>, Option<&str>) {
        self.state.to_data(&MossMonsterState::Idle)
    }
    pub fn walk_right(&mut self) -> (Option<bool>, Option<&str>) {
        self.state.to_data(&MossMonsterState::WalkRight)
    }
    pub fn attack(&mut self) -> (Option<bool>, Option<&str>) {
        self.state.to_data(&MossMonsterState::Attack)
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
