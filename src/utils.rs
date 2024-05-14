use bevy::prelude::*;

pub mod dungeon_seed;
pub mod dungeon_seed_engine;
pub mod toast;
pub mod ui;

pub fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
