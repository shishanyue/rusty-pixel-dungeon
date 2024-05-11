use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Asset, TypePath, Debug, Serialize, Deserialize, Clone)]
pub struct AppAssetsPath {
    pub banners: String,
    pub chrome: String,
    pub fusion_pixel_font:String,
    pub han_sans_font:String,
}

impl Default for AppAssetsPath {
    fn default() -> Self {
        Self {
            banners: "interfaces/banners.png".to_string(),
            chrome: "interfaces/chrome.png".to_string(),
            fusion_pixel_font: "fonts/fusion_pixel.ttf".to_string(),
            han_sans_font: "fonts/SourceHanSansSC-Light.otf".to_string(),
        }
    }
}
