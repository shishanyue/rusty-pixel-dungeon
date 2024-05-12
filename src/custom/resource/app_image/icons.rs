use bevy::prelude::*;

use crate::custom::{
    assets::{dynamic_image::DynamicImage, AppAssetsHandles},
    resource::AppResource,
};

#[derive(Default, Clone)]
pub struct IconsResource {
    pub enter_handle: Handle<Image>,
    pub gold_handle: Handle<Image>,
    pub rankings_handle: Handle<Image>,
    pub badges_handle: Handle<Image>,
    pub news_handle: Handle<Image>,
    pub changes_handle: Handle<Image>,
    pub prefs_handle: Handle<Image>,
    pub shpx_handle: Handle<Image>,
    pub stairs_handle: Handle<Image>,
}

impl IconsResource {
    pub fn load(
        app_res: &mut ResMut<AppResource>,
        app_assets_handles: &Res<AppAssetsHandles>,
        dynamic_images: &mut ResMut<Assets<DynamicImage>>,
        images: &mut ResMut<Assets<Image>>,
    ) {
        let texture_handle = app_assets_handles.app_image.icons.clone();

        let icons_image = dynamic_images.get(texture_handle.clone()).unwrap();
        app_res.app_image.icons.enter_handle =
            images.add(icons_image.uv_by_rect(0, 0, 16, 16).to_image());

        app_res.app_image.icons.gold_handle =
            images.add(icons_image.uv_by_rect(17, 0, 17, 16).to_image());

        app_res.app_image.icons.rankings_handle =
            images.add(icons_image.uv_by_rect(34, 0, 17, 16).to_image());

        app_res.app_image.icons.badges_handle =
            images.add(icons_image.uv_by_rect(51, 0, 16, 16).to_image());

        app_res.app_image.icons.news_handle =
            images.add(icons_image.uv_by_rect(68, 0, 16, 15).to_image());

        app_res.app_image.icons.changes_handle =
            images.add(icons_image.uv_by_rect(85, 0, 15, 15).to_image());

        app_res.app_image.icons.prefs_handle =
            images.add(icons_image.uv_by_rect(102, 0, 14, 14).to_image());

        app_res.app_image.icons.shpx_handle =
            images.add(icons_image.uv_by_rect(119, 0, 16, 16).to_image());

        app_res.app_image.icons.stairs_handle =
            images.add(icons_image.uv_by_rect(0, 16, 13, 16).to_image());
    }
}
