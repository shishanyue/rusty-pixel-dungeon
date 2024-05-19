pub mod splashes;

use bevy::prelude::*;

use crate::custom::assets::dynamic_image::DynamicImage;

use self::splashes::SplashesHandles;

#[derive(Default,Debug)]
pub struct AppImageHandle{
    pub banners: Handle<DynamicImage>,
    pub chrome: Handle<DynamicImage>,
    pub icons: Handle<DynamicImage>,
    pub splashes: SplashesHandles,
}