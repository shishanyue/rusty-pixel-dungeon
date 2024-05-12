use bevy::prelude::*;

use crate::custom::{assets::{dynamic_image::DynamicImage, AppAssetsHandles}, resource::AppResource};


#[derive(Default)]
pub struct ChromeResource {
    pub grey_button_tr_handle: Handle<Image>,
    pub grey_button_tr_slicer: TextureSlicer,
}

impl ChromeResource {
    pub fn load(
        app_res: &mut ResMut<AppResource>,
        app_assets_handles: &Res<AppAssetsHandles>,
        dynamic_images: &mut ResMut<Assets<DynamicImage>>,
        images: &mut ResMut<Assets<Image>>,
    ) {
        let texture_handle = app_assets_handles.app_image.chrome.clone();

        let chrome_image = dynamic_images.get(texture_handle.clone()).unwrap();
        let grey_button_tr_image = chrome_image.uv_by_rect(20, 9, 9, 9);

        app_res.app_image.chrome.grey_button_tr_handle = images.add(grey_button_tr_image.to_image());

        app_res.app_image.chrome.grey_button_tr_slicer = TextureSlicer {
            border: BorderRect::square(4.0),
            center_scale_mode: SliceScaleMode::Stretch,
            sides_scale_mode: SliceScaleMode::Stretch,
            max_corner_scale: 3.0,
        };
    }
}
