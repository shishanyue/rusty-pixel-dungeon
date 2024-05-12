use bevy::prelude::*;

use crate::custom::{assets::{dynamic_image::DynamicImage, AppAssetsHandles}, resource::AppResource};



#[derive(Default)]
pub struct BannersResource {
    pub pixel_dungeon_handle: Handle<Image>,
    pub pixel_dungeon_signs_handle: Handle<Image>,
}

#[derive(Component)]
pub struct PixelDungeon;
#[derive(Component)]
pub struct PixelDungeonSigns;

impl BannersResource {
    pub fn load(
        app_res: &mut ResMut<AppResource>,
        app_assets_handles: &Res<AppAssetsHandles>,
        dynamic_images: &mut ResMut<Assets<DynamicImage>>,
        images: &mut ResMut<Assets<Image>>,
    ) {
        let texture_handle = app_assets_handles.app_image.banners.clone();

        let banners_image = dynamic_images.get(texture_handle.clone()).unwrap();
        app_res.app_image.banners.pixel_dungeon_handle =
            images.add(banners_image.uv_by_rect(0, 0, 132, 90).to_image());
        app_res.app_image.banners.pixel_dungeon_signs_handle = images.add(
            banners_image
                .uv_by_rect(132, 0, 256, 90)
                .brighten(150)
                .to_image(),
        );
    }
}
