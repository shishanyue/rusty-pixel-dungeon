use std::fmt::Debug;

use bevy::{prelude::*, render::render_asset::RenderAssetUsages};

#[derive(Asset, TypePath, Debug, Clone)]
pub struct DynamicImage {
    pub dyn_img: image::DynamicImage,
    pub is_srgb: bool,
    pub asset_usage: RenderAssetUsages,
}

impl DynamicImage {
    pub fn to_image(&self) -> Image {
        Image::from_dynamic(self.dyn_img.clone(), self.is_srgb, self.asset_usage)
    }

    pub fn uv_by_rect(&self, x: u32, y: u32, width: u32, height: u32) -> DynamicImage {
        let uv_image = self.dyn_img.crop_imm(x, y, width, height);
        Self {
            dyn_img: uv_image,
            is_srgb: self.is_srgb,
            asset_usage: self.asset_usage,
        }
    }
}
