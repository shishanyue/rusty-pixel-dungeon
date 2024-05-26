use bevy::prelude::*;

use self::toast::ToastPlugin;

pub mod dungeon_seed;
pub mod dungeon_seed_engine;
pub mod toast;
pub mod ui;

pub struct UtilsPlugin;

impl Plugin for UtilsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ToastPlugin);
    }
}


pub fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
