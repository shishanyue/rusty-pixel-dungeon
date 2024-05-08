use bevy::prelude::*;

use crate::bevy_ext::{add_atlas_layout, add_atlas_with_rect};

use super::{assets_path::AppAssetsPath, AppRes};

#[derive(Default)]
pub struct ChromeRes {
    pub texture_handle: Handle<Image>,
    pub grey_button_tr_slicer:TextureSlicer,
    pub grey_button_tr_atlas: TextureAtlas,
}



impl ChromeRes {
    pub fn load(
        app_res: &mut ResMut<AppRes>,
        app_assets_path: &Res<AppAssetsPath>,
        asset_server: &Res<AssetServer>,
        atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    ) {
        let chrome_handle = asset_server.load(&app_assets_path.chrome);

        let chrome_atlas_layout_handle = add_atlas_layout(
            TextureAtlasLayout::new_empty(Vec2::new(128., 64.)),
            atlas_layouts,
        );

        app_res.chrome.grey_button_tr_atlas = add_atlas_with_rect(
            &chrome_atlas_layout_handle,
            Rect::new(20., 9., 29., 18.),
            atlas_layouts,
        );

        app_res.chrome.grey_button_tr_slicer = TextureSlicer {
            border: BorderRect::square(4.0),
            center_scale_mode: SliceScaleMode::Stretch,
            sides_scale_mode: SliceScaleMode::Stretch,
            max_corner_scale: 1.0,
        };
        app_res.chrome.texture_handle = chrome_handle;
    }
}