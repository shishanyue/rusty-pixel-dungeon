use std::{fmt::Debug, io::Cursor};

use bevy::render::texture::ImageType as BevyImageType;
use bevy::{
    asset::{io::Reader, AssetLoader, AsyncReadExt, LoadContext},
    prelude::*,
    render::texture::{FileTextureError, ImageFormatSetting, ImageLoaderSettings, TextureError},
    utils::BoxedFuture,
};
use image::io::Reader as ImageReader;
use thiserror::Error;

use crate::custom::assets::dynamic_image::DynamicImage;

#[derive(Default)]
pub struct DynamicImageLoader;

#[derive(Debug, Error)]
pub enum DynamicImageLoaderError {
    #[error("Could load shader: {0}")]
    Io(#[from] std::io::Error),
    #[error("Could not load texture file: {0}")]
    FileTexture(#[from] FileTextureError),
    #[error("Could not load texture file: {0}")]
    Texture(#[from] TextureError),
    #[error("Could not decode dynamic image: {0}")]
    Decode(#[from] image::ImageError),
}

impl AssetLoader for DynamicImageLoader {
    type Asset = DynamicImage;
    type Settings = ImageLoaderSettings;
    type Error = DynamicImageLoaderError;
    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        settings: &'a ImageLoaderSettings,
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<DynamicImage, Self::Error>> {
        Box::pin(async move {
            // use the file extension for the image type
            let ext = load_context.path().extension().unwrap().to_str().unwrap();

            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            let image_type = match settings.format {
                ImageFormatSetting::FromExtension => BevyImageType::Extension(ext),
                ImageFormatSetting::Format(format) => BevyImageType::Format(format),
            };

            let format = image_type.to_image_format()?;

            let image_crate_format = format
                .as_image_crate_format()
                .ok_or_else(|| TextureError::UnsupportedTextureFormat(format!("{format:?}")))?;

            let mut reader = ImageReader::new(Cursor::new(bytes));
            reader.set_format(image_crate_format);
            reader.no_limits();
            let dyn_img = reader.decode()?;

            Ok(DynamicImage {
                dyn_img,
                is_srgb: settings.is_srgb,
                asset_usage: settings.asset_usage,
            })
        })
    }
}
