pub mod app_assets_path;
pub mod dynamic_image;
pub mod raw;

use bevy::prelude::*;

use self::{app_assets_path::AppAssetsPath, dynamic_image::DynamicImage};

pub struct CustomAssetsPlugin;

impl Plugin for CustomAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<AppAssetsPath>()
            .init_asset::<DynamicImage>();
    }
}
