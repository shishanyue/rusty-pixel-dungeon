use bevy::{
    asset::{io::Reader, AssetLoader, AsyncReadExt, LoadContext},
    prelude::*,
    utils::BoxedFuture,
};
use thiserror::Error;

use crate::custom::assets::app_assets_path::AppAssetsPath;

#[derive(Default)]
pub struct AppAssetsPathLoader;

#[derive(Debug, Error)]
pub enum AppAssetsPathLoaderError {
    /// An [IO](std::io) Error
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),
    /// A [RON](ron) Error
    #[error("Could not parse Toml: {0}")]
    TomlDeError(#[from] toml::de::Error),
}

impl AssetLoader for AppAssetsPathLoader {
    type Asset = AppAssetsPath;
    type Settings = ();
    type Error = AppAssetsPathLoaderError;
    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a (),
        _load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes = String::new();

            reader.read_to_string(&mut bytes).await?;
            let custom_asset = toml::from_str::<AppAssetsPath>(&bytes)?;
            Ok(custom_asset)
        })
    }

    fn extensions(&self) -> &[&str] {
        &["toml"]
    }
}
