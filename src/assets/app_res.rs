use bevy::prelude::*;

use crate::{custom::assets::dynamic_image::DynamicImage, system::SystemStatus};

use super::{banners::BannersRes, chrome::ChromeRes, AppAssetsHandles};

#[derive(Default, Resource)]
pub struct AppRes {
    pub banners: BannersRes,
    pub chrome: ChromeRes,
}

pub fn init_app_res(
    mut app_res: ResMut<AppRes>,
    mut system_status: ResMut<SystemStatus>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut dynamic_images: ResMut<Assets<DynamicImage>>,
    mut images: ResMut<Assets<Image>>,
    app_assets_handles: Res<AppAssetsHandles>,
) {
    BannersRes::load(&mut app_res, &app_assets_handles, &mut atlas_layouts);
    ChromeRes::load(
        &mut app_res,
        &app_assets_handles,
        &mut dynamic_images,
        &mut images,
    );
    system_status.inited_assets = true;
}
