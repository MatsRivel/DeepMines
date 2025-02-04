use super::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
pub enum ImageLoadingState {
    #[default]
    Setup,
    Finished,
}

#[derive(Resource, Default, Debug)]
pub struct FolderOfImagesCollection(Vec<Handle<LoadedFolder>>);
impl FolderOfImagesCollection{
    pub fn inner(&self)->&Vec<Handle<LoadedFolder>>{
        &self.0
    }
}
pub fn load_image_folders(mut commands: Commands, assets: Res<AssetServer>) {
    info!("Loading image folders");
    let paths = IMAGE_PATHS;
    let folder_handles = paths.iter().map(|&path|assets.load_folder(path)).collect();
    commands.insert_resource(FolderOfImagesCollection(folder_handles));
}

pub fn check_whether_image_folders_are_loaded(
    mut next_state: ResMut<NextState<ImageLoadingState>>,
    rpg_sprite_folder: Res<FolderOfImagesCollection>,
    mut events: EventReader<AssetEvent<LoadedFolder>>,
) {
    // Advance the `AppState` once all sprite handles have been loaded by the `AssetServer`
    let all_handles_are_loaded = rpg_sprite_folder.inner().iter().all(|handle| events.read().all(|event|event.is_loaded_with_dependencies(handle.id())));
    if all_handles_are_loaded{
        next_state.set(ImageLoadingState::Finished);
    }
}

#[derive(Component, Clone, Debug)]
pub struct AnimationConfig {
    pub first_sprite_index: usize,
    pub last_sprite_index: usize,
    pub fps: u8,
    pub frame_timer: Timer,
}

impl AnimationConfig {
    pub fn new(first: usize, last: usize, fps: u8) -> Self {
        Self {
            first_sprite_index: first,
            last_sprite_index: last,
            fps,
            frame_timer: Self::timer_from_fps(fps),
        }
    }

    pub fn timer_from_fps(fps: u8) -> Timer {
        Timer::new(Duration::from_secs_f32(1.0 / (fps as f32)), TimerMode::Once)
    }
}
#[derive(Component,Debug)]
pub struct MetaTextureAtlas{
    pub animation_congfigs: Vec<(FolderId,AnimationConfig)>,
    pub atlas_layout: TextureAtlasLayout, 
    // pub atlas_sources: TextureAtlasSources,
    pub image_handle: Handle<Image>
}

const ANIMATION_FPS: u8 = 10;

pub fn build_atlas_from_spritesheet(textures: &mut ResMut<Assets<Image>>)->MetaTextureAtlas{
    let mut animation_configs = vec![];
    let mut current_max_idx = 0;
    let texture_atlas_layour = TextureAtlasLayout::from_grid(UVec2::new(116/5,80), 5, 1, 0, 0);
    let mut atlas = TextureAtlas{ layout: texture_atlas_layour, index: 0 };
    let meta_atlas = MetaTextureAtlas{
        animation_congfigs: todo!(),
        atlas_layout: todo!(),
        image_handle: todo!(),
    };
    todo!()
}



type FolderId = usize;
type FolderLength = usize;
pub fn build_atlas_from_folder_of_frames(folders: Vec::<(FolderId, &LoadedFolder)>,textures: &mut ResMut<Assets<Image>>)->MetaTextureAtlas{
    let mut animation_configs = vec![];
    let mut atlas_builder = TextureAtlasBuilder::default();
    let mut current_max_idx = 0;
    for ( id, folder,) in folders.iter(){
        let length = folder.handles.len();
        let config = AnimationConfig::new(current_max_idx, current_max_idx+length-1, ANIMATION_FPS);
        animation_configs.push((*id, config));
        current_max_idx += length;

        for handle in folder.handles.iter(){
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
    }
    let (texture_atlas_layout, texture_atlas_sources, texture_atlas_image) =
        atlas_builder.build().unwrap();
    let texture_atlas_image_handle = textures.add(texture_atlas_image);

    MetaTextureAtlas{   
        animation_congfigs: animation_configs,
        atlas_layout: texture_atlas_layout,
        // atlas_sources: texture_atlas_sources,
        image_handle: texture_atlas_image_handle,
    }
}

pub fn build_animation_from_atlas(mut commands: &mut Commands, assets: &Res<AssetServer>, meta_atlas: MetaTextureAtlas){
    let atlas = TextureAtlas {
        layout: assets.add(meta_atlas.atlas_layout.clone()),
        index: 0,
    };
    let sprite = Sprite::from_atlas_image(meta_atlas.image_handle.clone(), atlas);
    let (id, animation_config) = meta_atlas.animation_congfigs[0].clone();
    let bundle = (
        sprite,
        animation_config,
        Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)).with_scale(Vec3::splat(10.0)),
        Visibility::Visible,
        SyncToRenderWorld,
        MossMonsterVary::new(),
        meta_atlas,
    );
    commands.spawn(bundle);

}