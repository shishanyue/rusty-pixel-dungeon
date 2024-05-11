use bevy::prelude::*;

use crate::custom::assets::{dynamic_image::DynamicImage, AppAssetsHandles};

use super::AppRes;

#[derive(Default)]
pub struct ChromeRes {
    pub grey_button_tr_handle: Handle<Image>,
    pub grey_button_tr_slicer: TextureSlicer,
}

impl ChromeRes {
    pub fn load(
        app_res: &mut ResMut<AppRes>,
        app_assets_handles: &Res<AppAssetsHandles>,
        dynamic_images: &mut ResMut<Assets<DynamicImage>>,
        images: &mut ResMut<Assets<Image>>,
    ) {
        let texture_handle = app_assets_handles.chrome.clone();

        let chrome_image = dynamic_images.get(texture_handle.clone()).unwrap();
        let grey_button_tr_image = chrome_image.uv_by_rect(20, 9, 9, 9);

        app_res.chrome.grey_button_tr_handle = images.add(grey_button_tr_image.to_image());

        app_res.chrome.grey_button_tr_slicer = TextureSlicer {
            border: BorderRect::square(4.0),
            center_scale_mode: SliceScaleMode::Stretch,
            sides_scale_mode: SliceScaleMode::Stretch,
            max_corner_scale: 1.0,
        };
    }
}
