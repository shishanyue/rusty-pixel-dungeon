use bevy::prelude::*;


use crate::{
    level::level_project::LevelProject,
    room::room_project::RoomProject,
    scenes::{Scene, SceneState},
    utils::despawn_screen,
};

pub trait AppExt {
    fn init_level<T: LevelProject>(&mut self) -> &mut Self;
    fn init_room<T: RoomProject>(&mut self) -> &mut Self;
    fn init_scene<T: Scene>(&mut self) -> &mut Self;
    fn add_scene_system<T: Component, M>(
        &mut self,
        states: SceneState,
        systems: impl IntoSystemConfigs<M>,
    ) -> &mut Self;
}

impl AppExt for App {
    fn init_level<T: LevelProject>(&mut self) -> &mut Self {
        let level = T::default();
        level.build(self);
        self.insert_resource(level)
    }

    fn init_room<T: RoomProject>(&mut self) -> &mut Self {
        let room = T::default();
        room.build(self);
        self.insert_resource(room)
    }

    fn init_scene<T: Scene>(&mut self) -> &mut Self {
        T::default().build(self);
        self
    }

    fn add_scene_system<T: Component, M>(
        &mut self,
        states: SceneState,
        systems: impl IntoSystemConfigs<M>,
    ) -> &mut Self {
        self.add_systems(OnEnter(states), systems)
            .add_systems(OnExit(states), despawn_screen::<T>)
    }
}

pub fn add_atlas_layout(
    atlas_layout:TextureAtlasLayout,
    atlas_layout_res: &mut ResMut<Assets<TextureAtlasLayout>>,
) -> Handle<TextureAtlasLayout> {
    atlas_layout_res.add(atlas_layout)
}


pub fn add_atlas_with_rect(
    atlas_layout_handle:&Handle<TextureAtlasLayout>,
    rect:Rect,
    atlas_layouts:&mut Assets<TextureAtlasLayout>
) -> TextureAtlas{
    TextureAtlas {
        layout: atlas_layout_handle.clone(),
        index: atlas_layouts.get_mut(atlas_layout_handle).unwrap().add_texture(rect),
    }
}