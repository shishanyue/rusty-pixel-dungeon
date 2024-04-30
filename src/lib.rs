use assets::AssetsPlugin;
use bevy::app::{App, Plugin, PluginGroup, PluginGroupBuilder};
use bevy_ecs_ldtk::LdtkPlugin;
use bevy_tween::DefaultTweenPlugins;
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

impl Plugin for RustyPixelDungeonPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((RustyPixelDungeonPlugins, DefaultTweenPlugins));
    }
}
pub struct RustyPixelDungeonPlugins;

impl PluginGroup for RustyPixelDungeonPlugins {
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
