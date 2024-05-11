use bevy::prelude::*;
use std::{fmt::Debug, marker::PhantomData};
#[derive(Asset, TypePath, Debug, Clone)]
pub struct Raw<T>
where
    T: Asset + TypePath + Debug + Clone,
{
    pub raw: Vec<u8>,
    pub ext: String,
    pub(crate) phantom: PhantomData<T>,
}
