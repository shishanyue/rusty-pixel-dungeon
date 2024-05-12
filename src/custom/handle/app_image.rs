use bevy::prelude::*;

use crate::custom::assets::dynamic_image::DynamicImage;

#[derive(Default,Debug)]
pub struct AppImageHandle{
    pub banners: Handle<DynamicImage>,
    pub chrome: Handle<DynamicImage>,
    pub icons: Handle<DynamicImage>,
}