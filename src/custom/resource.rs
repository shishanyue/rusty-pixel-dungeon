pub mod app_image;

use bevy::prelude::*;

use crate::states::system::SystemStatus;

use self::app_image::{
    banners::BannersResource, chrome::ChromeResource, icons::IconsResource, AppImageResource,
};

use super::{
    assets::{dynamic_image::DynamicImage, AppAssetsHandles},
    handle::{app_font::AppFontHandle, assets_path::AppAssetsPathHandle},
};

pub struct CustomResourcePlugin;

impl Plugin for CustomResourcePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AppResource>()
            .init_resource::<AppAssetsPathHandle>();
    }
}

#[derive(Default, Resource)]
pub struct AppResource {
    pub app_image: AppImageResource,
    pub app_font: AppFontHandle,
}

pub fn init_app_res(
    mut app_res: ResMut<AppResource>,
    mut system_status: ResMut<SystemStatus>,
    mut dynamic_images: ResMut<Assets<DynamicImage>>,
    mut images: ResMut<Assets<Image>>,
    app_assets_handles: Res<AppAssetsHandles>,
) {
    BannersResource::load(
        &mut app_res,
        &app_assets_handles,
        &mut dynamic_images,
        &mut images,
    );
    ChromeResource::load(
        &mut app_res,
        &app_assets_handles,
        &mut dynamic_images,
        &mut images,
    );
    IconsResource::load(
        &mut app_res,
        &app_assets_handles,
        &mut dynamic_images,
        &mut images,
    );

    app_res.app_font = app_assets_handles.app_font.clone();
    app_res.app_image.splashes = app_assets_handles.app_image.splashes.clone();

    system_status.inited_assets = true;
}
