use actors::hero::{HeroBundle, HeroType};
use bevy::app::{App, Plugin, PluginGroup, PluginGroupBuilder};
use bevy_ecs_ldtk::{app::LdtkEntityAppExt, LdtkPlugin};
use bevy_spritesheet_animation::plugin::SpritesheetAnimationPlugin;
use bevy_tween::DefaultTweenPlugins;
use custom::CustomPlugins;
use level::LevelProjectPlugin;
use room::RoomProjectPlugin;
use scenes::ScenePlugin;
use seldom_state::StateMachinePlugin;
use system::SystemPlugin;

pub mod actors;
pub mod bevy_ext;
pub mod custom;
pub mod level;
pub mod panel;
pub mod room;
pub mod scenes;
pub mod system;
pub mod utils;

pub struct RustyPixelDungeonPlugin;

impl Plugin for RustyPixelDungeonPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            RustyPixelDungeonPlugins,
            CustomPlugins,
            DefaultTweenPlugins,
            StateMachinePlugin,
            SpritesheetAnimationPlugin,
        ))
        .register_type::<actors::hero::HeroType>()
        .register_type::<room::home_room::HomeRoomPojectSize>()
        .register_type::<scenes::start_scene::ButtonLabel>()
        .register_type::<scenes::title_scene::ButtonLabel>()
        .register_default_ldtk_entity_for_layer::<HeroBundle>("Hero")
        .register_type::<HeroType>();
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
            .add(SystemPlugin);
        group
    }
}
