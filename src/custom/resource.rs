pub mod assets_path;
pub mod banners;
pub mod chrome;
pub mod han_sans_fonts;

use bevy::prelude::*;

use crate::system::SystemStatus;

use self::{assets_path::AppAssetsPathHandle, banners::BannersRes, chrome::ChromeRes};

use super::assets::{app_assets_path::AppAssetsPath, dynamic_image::DynamicImage, AppAssetsHandles};

pub struct CustomResourcePlugin;

impl Plugin for CustomResourcePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AppAssetsPathHandle>();
    }
}



#[derive(Default, Resource)]
pub struct AppRes {
    pub banners: BannersRes,
    pub chrome: ChromeRes,
    pub fusion_pixel_font: Handle<Font>,
    pub han_sans_font: Handle<Font>,
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

    app_res.fusion_pixel_font = app_assets_handles.fusion_pixel_font.clone();
    app_res.han_sans_font = app_assets_handles.han_sans_font.clone();
    system_status.inited_assets = true;
}
