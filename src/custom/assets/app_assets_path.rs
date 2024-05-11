use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Asset, TypePath, Debug, Serialize, Deserialize, Clone)]
pub struct AppAssetsPath {
    pub banners: String,
    pub chrome: String,
}

impl Default for AppAssetsPath {
    fn default() -> Self {
        Self {
            banners: "interfaces/banners.png".to_string(),
            chrome: "interfaces/chrome.png".to_string(),
        }
    }
}
