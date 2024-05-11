use bevy::app::{PluginGroup, PluginGroupBuilder};

use self::{assets::CustomAssetsPlugin, loader::CustomLoaderPlugin};

pub mod assets;
pub mod bundle;
pub mod loader;

pub struct CustomPlugins;

impl PluginGroup for CustomPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        let mut group = PluginGroupBuilder::start::<Self>();
        group = group.add(CustomLoaderPlugin).add(CustomAssetsPlugin);

        group
    }
}
