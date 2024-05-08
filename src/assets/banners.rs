use bevy::prelude::*;

use crate::bevy_ext::{add_atlas_layout, add_atlas_with_rect};

use super::{assets_path::AppAssetsPath, AppRes};

#[derive(Default)]
pub struct BannersRes {
    pub texture_handle: Handle<Image>,
    pub pixel_dungeon_atlas: TextureAtlas,
    pub pixel_dungeon_signs_atlas: TextureAtlas,
}

#[derive(Component)]
pub struct PixelDungeon;
#[derive(Component)]
pub struct PixelDungeonSigns;

impl BannersRes {
    pub fn load(
        app_res: &mut ResMut<AppRes>,
        app_assets_path: &Res<AppAssetsPath>,
        asset_server: &Res<AssetServer>,
        atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    ) {
        let banners_handle = asset_server.load(&app_assets_path.banners);

        let banners_atlas_layout_handle = add_atlas_layout(
            TextureAtlasLayout::new_empty(Vec2::new(256., 256.)),
            atlas_layouts,
        );

        app_res.banners.pixel_dungeon_atlas = add_atlas_with_rect(
            &banners_atlas_layout_handle,
            Rect::new(0., 0., 132., 90.),
            atlas_layouts,
        );

        app_res.banners.pixel_dungeon_signs_atlas = add_atlas_with_rect(
            &banners_atlas_layout_handle,
            Rect::new(132., 0., 256., 90.),
            atlas_layouts,
        );

        app_res.banners.texture_handle = banners_handle;
    }
}
