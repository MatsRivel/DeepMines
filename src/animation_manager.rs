use super::*;

#[derive(Resource, Default, Debug)]
pub struct FolderOfImagesCollection(Vec<Handle<LoadedFolder>>);
impl FolderOfImagesCollection{
    pub fn inner(&self)->&Vec<Handle<LoadedFolder>>{
        &self.0
    }
}
pub fn load_image_folders(mut commands: Commands, assets: Res<AssetServer>) {
    info!("Load textures");
    let paths = IMAGE_PATHS;
    let folder_handles = paths.iter().map(|&path|assets.load_folder(path)).collect();
    commands.insert_resource(FolderOfImagesCollection(folder_handles));
}
