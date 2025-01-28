mod pos;
mod animation_manager;

use std::time::Duration;
use animation_manager::{FolderOfImagesCollection,load_image_folders};
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
        .init_state::<AppState>()
        .add_systems(OnEnter(AppState::Setup), crate::animation_manager::load_image_folders)
        .add_systems(Update, check_textures.run_if(in_state(AppState::Setup)))
        .add_systems(OnEnter(AppState::Finished), setup)
        // .add_systems(Startup, (
        //     load_textures.before(setup),
        //     setup.after(load_textures)
        // ))
        .add_systems(FixedUpdate, animate_stuff)
        // .add_systems(FixedUpdate, animate_all)
        // .add_systems(FixedUpdate, key_trigger_animation)
        .run();
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
enum AppState {
    #[default]
    Setup,
    Finished,
}
fn check_textures(
    mut next_state: ResMut<NextState<AppState>>,
    rpg_sprite_folder: Res<FolderOfImagesCollection>,
    mut events: EventReader<AssetEvent<LoadedFolder>>,
) {
    // Advance the `AppState` once all sprite handles have been loaded by the `AssetServer`
    for handle in rpg_sprite_folder.inner().iter(){
        for event in events.read() {
            if event.is_loaded_with_dependencies(handle.id()) {
                next_state.set(AppState::Finished);
            }
        }
    }
}

// #[derive(Resource, Default, Debug)]
// pub struct SmallMosterFolder(Vec<Handle<LoadedFolder>>);
// pub fn load_textures(mut commands: Commands, assets: Res<AssetServer>) {
//     info!("Load textures");
//     let paths = [SOURCE_FOLDER1,SOURCE_FOLDER2];
//     let folder_handles = paths.iter().map(|&path|assets.load_folder(path)).collect();
//     commands.insert_resource(SmallMosterFolder(folder_handles));
// }

/// Takes the files in the folder, in order, and adds them to an atlas.
/// NOTE: Does not work well for sprite-sheets.
pub fn build_atlas_from_folder_of_frames(
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
    println!("{folder_handles:?}");
    let smf1 = folder_handles.inner()[0].id();
    let smf2 = folder_handles.inner()[1].id();
    let loaded_folder = loaded_folders.get(smf1).unwrap();
    let loaded_folder2 = loaded_folders.get(smf2).unwrap();

    let (texture_atlas_layout, _texture_atlas_sources, texture_atlas_image_handle) =
        build_atlas_from_folder_of_frames(loaded_folder, loaded_folder2, &mut textures);
    // let texture_atlas_layout_adjusted = TextureAtlasLayout::from_grid(UVec2::new(28,39), 5, 13, None, None);
    // texture_atlas_layout.
    // println!("texture_atlas_layout: {texture_atlas_layout:#?}");
    println!("{_texture_atlas_sources:#?}");
    let atlas = TextureAtlas {
        layout: assets.add(texture_atlas_layout),
        index: 0,
    };
    let sprite = Sprite::from_atlas_image(texture_atlas_image_handle, atlas);
    // commands.spawn((sprite.clone(),Visibility::Visible));

    // let sprite_two = Sprite::from_image(assets.load(ATLAS_FILE_ATTACK));
    // commands.spawn((sprite_two,Visibility::Visible));

    let first_frame = 0;
    let animation_length = 18;
    let animation_config = AnimationConfig::new(first_frame, first_frame+animation_length-1, 10);
    let variation_bundle = (
        sprite,
        animation_config,
        Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)).with_scale(Vec3::splat(10.0)),
        Visibility::Visible,
        SyncToRenderWorld,
        MossMonsterVary::new(),
    );
    commands.spawn(variation_bundle);
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

pub fn animate_stuff(time: Res<Time>, mut query: Query<(&mut AnimationConfig, &mut Sprite)>) {
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
