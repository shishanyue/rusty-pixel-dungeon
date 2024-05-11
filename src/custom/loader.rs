use bevy::prelude::*;

use self::{app_assets_path::AppAssetsPathLoader, dynamic_image::DynamicImageLoader};

pub mod app_assets_path;
pub mod dynamic_image;
pub mod raw;

pub struct CustomLoaderPlugin;

impl Plugin for CustomLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset_loader::<DynamicImageLoader>()
            .init_asset_loader::<AppAssetsPathLoader>();
    }
}
