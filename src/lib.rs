use assets::AssetsPlugin;
use bevy::app::{PluginGroup, PluginGroupBuilder};
use bevy_ecs_ldtk::LdtkPlugin;
use level::LevelProjectPlugin;
use room::RoomProjectPlugin;
use scenes::ScenePlugin;
use system::SystemPlugin;

pub mod actors;
pub mod assets;
pub mod bevy_ext;
pub mod level;
pub mod room;
pub mod scenes;
pub mod system;
pub mod utils;

pub struct RustyPixelDungeonPlugin;

impl PluginGroup for RustyPixelDungeonPlugin {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        let mut group = PluginGroupBuilder::start::<Self>();
        group = group
            .add(LdtkPlugin)
            .add(RoomProjectPlugin)
            .add(LevelProjectPlugin)
            .add(ScenePlugin)
            .add(SystemPlugin)
            .add(AssetsPlugin);

        group
    }
}
