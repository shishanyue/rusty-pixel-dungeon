use actors::{hero::{HeroBundle, HeroType}, ActorPlugin};
use bevy::app::{App, Plugin, PluginGroup, PluginGroupBuilder};
use bevy_ecs_ldtk::{app::LdtkEntityAppExt, LdtkPlugin};
use bevy_mod_picking::prelude::*;
use bevy_replicon::RepliconPlugins;
use bevy_replicon_renet::RepliconRenetPlugins;
use bevy_spritesheet_animation::plugin::SpritesheetAnimationPlugin;
use bevy_tween::DefaultTweenPlugins;
use client::plugin::RustyPixelDungeonClientPlugin;
use custom::CustomPlugins;
use level::LevelProjectPlugin;
use panel::PanelPlugin;
use room::RoomProjectPlugin;
use rusty_pixel_dungeon_server::RustyPixelDungeonServerPlugin;
use scenes::ScenePlugin;
use seldom_state::StateMachinePlugin;
use system::SystemPlugin;
pub mod client;
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
            DefaultPickingPlugins,
            RepliconPlugins,
            RepliconRenetPlugins,
            RustyPixelDungeonClientPlugin,
            RustyPixelDungeonServerPlugin
        ))
        .insert_resource(DebugPickingMode::Normal)
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
            .add(PanelPlugin)
            .add(SystemPlugin)
            .add(ActorPlugin);
        group
    }
}
