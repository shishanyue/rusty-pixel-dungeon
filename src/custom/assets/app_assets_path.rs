use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Asset, TypePath, Debug, Serialize, Deserialize, Clone)]
pub struct AppAssetsPath {
    pub banners: String,
    pub chrome: String,
    pub font:String
}

impl Default for AppAssetsPath {
    fn default() -> Self {
        Self {
            banners: "interfaces/banners.png".to_string(),
            chrome: "interfaces/chrome.png".to_string(),
            font: "fonts/fusion_pixel.ttf".to_string()
        }
    }
}
