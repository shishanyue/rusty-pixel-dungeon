use bevy::{
    asset::{io::Reader, AssetLoader, AsyncReadExt, LoadContext},
    prelude::*,
    utils::BoxedFuture,
};
use thiserror::Error;

use crate::custom::assets::raw::Raw;

use std::{fmt::Debug, marker::PhantomData};
#[derive(Default)]
pub struct RawLoader;

#[derive(Debug, Error)]
pub enum RawLoaderError {
    /// An [IO](std::io) Error
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),
}

impl<T> AssetLoader for Raw<T>
where
    T: Asset + TypePath + Debug + Clone,
{
    type Asset = Raw<T>;
    type Settings = ();
    type Error = RawLoaderError;
    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a (),
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let ext = load_context
                .path()
                .extension()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();
            let mut raw = Vec::new();
            reader.read_to_end(&mut raw).await?;
            Ok(Self {
                raw,
                ext,
                phantom: PhantomData,
            })
        })
    }

    fn extensions(&self) -> &[&str] {
        &[]
    }
}
