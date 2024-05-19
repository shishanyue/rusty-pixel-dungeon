use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FontsPath {
    pub bold: String,
    pub extra_light: String,
    pub heavy: String,
    pub light: String,
    pub medium: String,
    pub normal: String,
    pub regular: String,
    pub fusion_pixel: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SplashesPath {
    pub duelist: String,
    pub huntress: String,
    pub mage: String,
    pub rogue: String,
    pub warrior: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ImagesPath {
    pub banners: String,
    pub chrome: String,
    pub icons: String,
    pub splashes: SplashesPath,
}

#[derive(Asset, TypePath, Debug, Serialize, Deserialize, Clone,Default)]
pub struct AppAssetsPath {
    pub image: ImagesPath,
    pub font: FontsPath,
}

impl Default for FontsPath {
    fn default() -> Self {
        Self {
            bold: "fonts/SourceHanSansCN-Bold.otf".to_string(),
            extra_light: "fonts/SourceHanSansCN-ExtraLight.otf".to_string(),
            heavy: "fonts/SourceHanSansCN-Heavy.otf".to_string(),
            light: "fonts/SourceHanSansCN-Light.otf".to_string(),
            medium: "fonts/SourceHanSansCN-Medium.otf".to_string(),
            normal: "fonts/SourceHanSansCN-Normal.otf".to_string(),
            regular: "fonts/SourceHanSansCN-Regular.otf".to_string(),
            fusion_pixel: "fonts/fusion_pixel.ttf".to_string(),
        }
    }
}

impl Default for SplashesPath {
    fn default() -> Self {
        Self {
            duelist: "splashes/duelist.jpg".to_string(),
            huntress: "splashes/huntress.jpg".to_string(),
            mage: "splashes/mage.jpg".to_string(),
            rogue: "splashes/rogue.jpg".to_string(),
            warrior: "splashes/warrior.jpg".to_string(),
        }
    }
}
impl Default for ImagesPath {
    fn default() -> Self {
        Self {
            banners: "interfaces/banners.png".to_string(),
            chrome: "interfaces/chrome.png".to_string(),
            icons: "interfaces/icons.png".to_string(),
            splashes: SplashesPath::default()
        }
    }
}
