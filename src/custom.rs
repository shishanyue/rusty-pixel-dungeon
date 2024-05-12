use bevy::app::{PluginGroup, PluginGroupBuilder};

use self::{
    assets::CustomAssetsPlugin, loader::CustomLoaderPlugin, resource::CustomResourcePlugin,
};

pub mod assets;
pub mod bundle;
pub mod loader;
pub mod resource;
pub mod handle;

pub struct CustomPlugins;

impl PluginGroup for CustomPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        let mut group = PluginGroupBuilder::start::<Self>();
        group = group
            .add(CustomLoaderPlugin)
            .add(CustomAssetsPlugin)
            .add(CustomResourcePlugin);

        group
    }
}
