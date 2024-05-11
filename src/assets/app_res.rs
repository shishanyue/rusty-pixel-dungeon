use bevy::prelude::*;

use crate::{custom::assets::dynamic_image::DynamicImage, system::SystemStatus};

use super::{banners::BannersRes, chrome::ChromeRes, AppAssetsHandles};

#[derive(Default, Resource)]
pub struct AppRes {
    pub banners: BannersRes,
    pub chrome: ChromeRes,
    pub font:Handle<Font>
}

pub fn init_app_res(
    mut app_res: ResMut<AppRes>,
    mut system_status: ResMut<SystemStatus>,
    mut dynamic_images: ResMut<Assets<DynamicImage>>,
    mut images: ResMut<Assets<Image>>,
    app_assets_handles: Res<AppAssetsHandles>,
) {
    BannersRes::load(
        &mut app_res,
        &app_assets_handles,
        &mut dynamic_images,
        &mut images,
    );
    ChromeRes::load(
        &mut app_res,
        &app_assets_handles,
        &mut dynamic_images,
        &mut images,
    );

    app_res.font = app_assets_handles.font.clone();

    system_status.inited_assets = true;
}
